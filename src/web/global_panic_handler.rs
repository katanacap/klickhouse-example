use crate::{clickhouse::models::WebServerLog, web::app_state::AppState};
use actix_web::web::Data;
use std::panic;

pub fn setup_global_panic_handler(app_state: Data<AppState>) {
    panic::set_hook(Box::new(move |info| {
        println!("Global panic handler called");
        let app_state = app_state.clone();

        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()));
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "Unknown panic".to_string());

        tokio::task::spawn(async move {
            // Create log
            let log = WebServerLog {
                timestamp: chrono::Utc::now(),
                level: "CRITICAL".to_string(),
                message: format!("Global panic occurred: {} at {:?}", message, location),
                module: "global".to_string(),
                request_id: "unknown".to_string(),
                uri: "unknown".to_string(),
                method: "unknown".to_string(),
                status_code: 500,
                response_time: 0.0,
            };

            // Write log to Clickhouse
            if let Err(e) = app_state.ch_logger().log(log).await {
                eprintln!("Failed to log panic to ClickHouse: {:?}", e);
            }
        });
    }));
}
