// src/handlers/mod.rs
// use crate::app_state::AppState;
// use actix_web::web::Data;
use actix_web::{get, Responder};
use tracing_actix_web::RequestId;

#[get("/")]
pub async fn index(request_id: RequestId) -> impl Responder {
    format!("request_id: {}", request_id)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    "I'm alive!"
}
