use actix_web::middleware::NormalizePath;
use actix_web::{web, App, HttpServer};
use eyre::Result;
use tracing_actix_web::TracingLogger;

// config
use crate::config::AppConfig;
// web
use crate::web::middleware::ClickhouseLogger;

use crate::web::app_state::AppState;
use crate::web::global_panic_handler::setup_global_panic_handler;
use crate::web::handlers::{fail_endpoint, health, index};

pub async fn run_serve(config: AppConfig) -> Result<actix_web::dev::Server> {
    let app_state = AppState::build(config).await?;
    let data_app_state = web::Data::new(app_state.clone());
    setup_global_panic_handler(data_app_state.clone());

    let port = app_state.config().http_port;
    let addr = format!("0.0.0.0:{}", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data_app_state.clone())
            .wrap(TracingLogger::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .wrap(ClickhouseLogger)
            .service(index)
            .service(health)
            .service(fail_endpoint)
    })
    .bind(addr)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_index() {
        let app =
            test::init_service(App::new().wrap(TracingLogger::default()).service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
