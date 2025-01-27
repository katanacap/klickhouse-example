use actix_web::{get, HttpResponse, Responder};
use tracing_actix_web::RequestId;

#[get("/")]
pub async fn index(request_id: RequestId) -> impl Responder {
    format!("request_id: {}", request_id)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    "I'm alive!"
}

#[get("/fail")]
#[allow(clippy::unnecessary_literal_unwrap)]
pub async fn fail_endpoint() -> impl Responder {
    // Сымитируем панику через unwrap
    let result: Option<&str> = None;
    let value = result.unwrap();
    HttpResponse::Ok().body(format!("Value: {}", value))
}
