use crate::clickhouse::{models::WebServerLog, ClickhousePool};
use eyre::{eyre, Result};

pub struct GetWebLogsService {}

impl GetWebLogsService {
    pub async fn get_logs(
        clickhouse_pool: &ClickhousePool,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<WebServerLog>> {
        // setup default values
        let logs_queries = clickhouse_pool.get_logs_queries().await?;

        match logs_queries.get_logs(limit, offset).await {
            Ok(logs) => Ok(logs),
            Err(e) => Err(eyre!("Error getting logs: {}", e)),
        }
    }
}
