use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
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
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
                            let duration = start_time.elapsed();
                            let request_id = res
                                .request()
                                .extensions()
                                .get::<RequestId>()
                                .map(|id| id.to_string())
                                .unwrap_or_default();

                            let uri = res.request().uri().to_string();
                            let method = res.request().method().to_string();
                            let status_code = res.status().as_u16() as i32;

                            let log = WebServerLog {
                                timestamp: chrono::Utc::now(),
                                level: "INFO".to_string(),
                                message: "Request processed".to_string(),
                                module: "web_server".to_string(),
                                request_id,
                                uri,
                                method,
                                status_code,
                                response_time: duration.as_secs_f64(),
                            };

                            let _ = app_state.ch_logger().log(log).await;

                            Ok(res)
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
