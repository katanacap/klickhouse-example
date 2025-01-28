use eyre::{eyre, Result};

use crate::clickhouse::ClickhouseLogger;
use crate::clickhouse::ClickhousePool;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct AppState {
    clickhouse_pool: ClickhousePool,
    config: AppConfig,
    ch_logger: ClickhouseLogger,
}

impl AppState {
    pub async fn build(config: AppConfig) -> Result<Self> {
        let clickhouse_pool = Self::connect_clickhouse(&config).await?;
        let ch_logger = ClickhouseLogger::new(&clickhouse_pool);

        Ok(Self {
            clickhouse_pool,
            config,
            ch_logger,
        })
    }

    async fn connect_clickhouse(config: &AppConfig) -> Result<ClickhousePool> {
        let clickhouse_config = config.get_clickhouse_config()?;

        let pool = ClickhousePool::connect(&clickhouse_config)
            .await
            .map_err(|e| eyre!("Failed to connect to Clickhouse: {}", e))?;

        pool.check_pool().await?;

        Ok(pool)
    }

    pub async fn check_clickhouse_connection(&self) -> Result<()> {
        self.clickhouse_pool.check_pool().await
    }

    pub fn clickhouse_pool(&self) -> &ClickhousePool {
        &self.clickhouse_pool
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn ch_logger(&self) -> &ClickhouseLogger {
        &self.ch_logger
    }
}
