use crate::web::app_state::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use tracing_actix_web::RequestId;

#[get("/")]
pub async fn index(request_id: RequestId) -> impl Responder {
    format!("request_id: {}", request_id)
}

#[get("/fail")]
#[allow(clippy::unnecessary_literal_unwrap)]
pub async fn fail_endpoint() -> impl Responder {
    // Сымитируем панику через unwrap
    let result: Option<&str> = None;
    let value = result.unwrap();
    HttpResponse::Ok().body(format!("Value: {}", value))
}

#[get("/health")]
pub async fn health(app_state: web::Data<AppState>) -> impl Responder {
    match app_state.check_clickhouse_connection().await {
        Ok(_) => HttpResponse::Ok().body("I'm alive!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
