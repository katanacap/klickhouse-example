use clap::Parser;
use eyre::{eyre, Result};

use klickhouse_example::cli::{Cli, Commands};
use klickhouse_example::clickhouse::ClickhousePool;
use klickhouse_example::config::AppConfig;
use klickhouse_example::web::startup::run_serve;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = AppConfig::build();

    let cli = Cli::try_parse().map_err(|e| eyre!("Failed to parse CLI command: {}", e))?;

    match cli.command {
        Some(Commands::Serve) => {
            let serve_handle = run_serve(config)
                .await
                .map_err(|e| eyre!("Failed to run web server: {}", e))?;
            serve_handle.await?;
        }
        Some(Commands::Migrate) => {
            let clickhouse_config = config.get_clickhouse_config().unwrap();
            let pool = ClickhousePool::connect(&clickhouse_config).await?;

            pool.run_migrations().await?;
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }

    Ok(())
}
