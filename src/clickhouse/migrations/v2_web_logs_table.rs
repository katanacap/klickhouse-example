use super::BoxedFuture;
use super::ClickhousePool;
use super::Migration;

pub struct LogsTableMigration;

impl Migration for LogsTableMigration {
    fn version(&self) -> i32 {
        2
    }

    fn name(&self) -> &str {
        "Create logs table"
    }

    fn apply<'a>(&self, pool: &'a ClickhousePool) -> BoxedFuture<'a> {
        Box::pin(async move {
            let connection = pool.get_connection().await?;
            connection
                .execute(
                    "CREATE TABLE IF NOT EXISTS web_server_logs (
                    timestamp       DateTime64(6, 'UTC'),
                    level           LowCardinality(String),
                    message         String,
                    module          String,
                    request_id      String,
                    uri             String,
                    method          String,
                    status_code     Int32,
                    response_time   Float64,
                ) ENGINE = MergeTree()
                PARTITION BY (toYYYYMM(timestamp))
                ORDER BY (timestamp)
                SETTINGS index_granularity = 8192;",
                )
                .await?;

            Ok(())
        })
    }
}
