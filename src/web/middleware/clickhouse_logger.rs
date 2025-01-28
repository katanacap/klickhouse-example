use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Bytes,
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ok, Ready};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
    time::Instant,
};
use tracing_actix_web::RequestId;

use crate::clickhouse::models::WebServerLog;
use crate::web::app_state::AppState;

pub struct ClickhouseLogger;

impl<S, B> Transform<S, ServiceRequest> for ClickhouseLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ClickhouseLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ClickhouseLoggerMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ClickhouseLoggerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ClickhouseLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let start_time = Instant::now();

        let app_state = req
            .app_data::<actix_web::web::Data<AppState>>()
            .expect("AppState is missing")
            .clone();

        Box::pin(async move {
            // Перехватываем панику при создании future
            let future = catch_unwind(AssertUnwindSafe(|| srv.call(req)));

            // Если future создан успешно, дожидаемся его выполнения
            match future {
                Ok(fut) => {
                    let result = fut.await;

                    match result {
                        Ok(res) => {
                            // Успешный запрос
                            let (req_head, http_resp) = res.into_parts();

                            let duration = start_time.elapsed();
                            let request_id = req_head
                                .extensions()
                                .get::<RequestId>()
                                .map(|id| id.to_string())
                                .unwrap_or_default();

                            let uri = req_head.uri().to_string();
                            let method = req_head.method().to_string();
                            let status_code = http_resp.status();

                            let body_bytes_result =
                                actix_web::body::to_bytes(http_resp.into_body()).await;
                            let message = match body_bytes_result {
                                Ok(ref bytes) => format!(
                                    "Request processed. Body: {}",
                                    String::from_utf8_lossy(bytes)
                                ),
                                Err(_) => "Request processed. Failed to read body.".to_string(),
                            };

                            let log = WebServerLog {
                                timestamp: chrono::Utc::now(),
                                level: "INFO".to_string(),
                                message,
                                module: "web_server".to_string(),
                                request_id,
                                uri,
                                method,
                                status_code: status_code.as_u16() as i32,
                                response_time: duration.as_secs_f64(),
                            };

                            let _ = app_state.ch_logger().log(log).await;

                            let new_body = match body_bytes_result {
                                Ok(ref b) => b.clone(),
                                Err(_) => Bytes::new(),
                            };

                            let new_http_response = HttpResponse::build(status_code).body(new_body);
                            let new_srv_response =
                                ServiceResponse::new(req_head, new_http_response)
                                    .map_into_boxed_body();

                            Ok(new_srv_response)
                        }
                        Err(e) => {
                            // Ошибка от следующего middleware/handler
                            let duration = start_time.elapsed();

                            let log = WebServerLog {
                                timestamp: chrono::Utc::now(),
                                level: "ERROR".to_string(),
                                message: format!("Handler error: {}", e),
                                module: "web_server".to_string(),
                                request_id: "".to_string(),
                                uri: "".to_string(),
                                method: "".to_string(),
                                status_code: 500, // Или другой код, если его можно получить из e
                                response_time: duration.as_secs_f64(),
                            };

                            let _ = app_state.ch_logger().log(log).await;

                            Err(e)
                        }
                    }
                }
                Err(panic) => {
                    // Обрабатываем панику при создании future
                    let duration = start_time.elapsed();

                    let panic_message = if let Some(message) = panic.downcast_ref::<&str>() {
                        message.to_string()
                    } else if let Some(message) = panic.downcast_ref::<String>() {
                        message.clone()
                    } else {
                        "Unknown panic".to_string()
                    };

                    let log = WebServerLog {
                        timestamp: chrono::Utc::now(),
                        level: "CRITICAL".to_string(),
                        message: format!("Panic occurred: {}", panic_message),
                        module: "web_server".to_string(),
                        request_id: "".to_string(),
                        uri: "".to_string(),
                        method: "".to_string(),
                        status_code: 500,
                        response_time: duration.as_secs_f64(),
                    };

                    let _ = app_state.ch_logger().log(log).await;

                    std::panic::resume_unwind(panic);
                }
            }
        })
    }
}
