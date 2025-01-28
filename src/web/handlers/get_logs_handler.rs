use crate::web::app_state::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::GetWebLogsService;

#[derive(Deserialize)]
pub struct WebLogsQuery {
    pub limit: Option<usize>,  // Limit of logs to return, default 10
    pub offset: Option<usize>, // Offset of logs to return, default 0
}

#[get("/logs")]
pub async fn get_logs(
    app_state: web::Data<AppState>,
    query: web::Query<WebLogsQuery>,
) -> impl Responder {
    let clickhouse_pool = app_state.clickhouse_pool();

    // setup default values
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    match GetWebLogsService::get_logs(clickhouse_pool, limit, offset).await {
        Ok(logs) => HttpResponse::Ok().json(logs),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
