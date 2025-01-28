use eyre::Result;

use crate::clickhouse::models::WebServerLog;
use crate::clickhouse::pool::ClickhouseConnection;

pub struct LogsQueries<'a> {
    conn: ClickhouseConnection<'a>,
}

impl<'a> LogsQueries<'a> {
    pub fn new(conn: ClickhouseConnection<'a>) -> Self {
        Self { conn }
    }

    pub async fn get_logs(&self, limit: usize, offset: usize) -> Result<Vec<WebServerLog>> {
        let query = format!(
            "SELECT * FROM web_server_logs ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            limit, offset
        );
        let logs = match self.conn.query_collect::<WebServerLog>(query).await {
            Ok(logs) => {
                tracing::debug!("Fetched {} logs", logs.len());
                logs
            }
            Err(e) => {
                tracing::error!("Error fetching logs: {}", e);
                return Err(e.into());
            }
        };

        Ok(logs)
    }
}
