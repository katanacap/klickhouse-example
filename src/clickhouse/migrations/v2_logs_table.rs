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
                    "CREATE TABLE IF NOT EXISTS logs (
                    level LowCardinality(String),
                    message String,
                    timestamp DateTime
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
