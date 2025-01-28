use std::sync::Arc;

use eyre::{eyre, Result};

use crate::clickhouse::{models::WebServerLog, ClickhousePool};

#[derive(Clone, Debug)]
pub struct ClickhouseLogger {
    clickhouse_pool: Arc<ClickhousePool>,
}

impl ClickhouseLogger {
    pub fn new(clickhouse_pool: &ClickhousePool) -> Self {
        let clickhouse_pool = Arc::new(clickhouse_pool.clone());

        Self { clickhouse_pool }
    }

    pub async fn log(&self, log: WebServerLog) -> Result<()> {
        tracing::debug!("Logging to Clickhouse: {:?}", log);

        let connection = self
            .clickhouse_pool
            .get_connection()
            .await
            .map_err(|e| eyre!("Failed to get Clickhouse connection: {}", e))?;

        let result = connection
            .insert_native_block("insert into web_server_logs format native", vec![log])
            .await
            .map_err(|e| eyre!("Failed to write log to ClickHouse: {}", e));

        if let Err(e) = result {
            tracing::error!("Error: {:?}", e);
        }

        Ok(())
    }
}
