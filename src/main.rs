use eyre::{eyre, Result};

use klickhouse_example::config::AppConfig;
use klickhouse_example::web::startup::run_serve;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = AppConfig::build();

    let serve_handle = run_serve(config)
        .await
        .map_err(|e| eyre!("Failed to run web server: {}", e))?;
    serve_handle.await?;

    Ok(())
}
