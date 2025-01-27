use super::BoxedFuture;
use super::ClickhousePool;
use super::Migration;

pub struct InitialMigration;

impl Migration for InitialMigration {
    fn version(&self) -> i32 {
        1
    }

    fn name(&self) -> &str {
        "InitialMigration"
    }

    fn apply<'a>(&self, pool: &'a ClickhousePool) -> BoxedFuture<'a> {
        Box::pin(async move {
            let connection = pool.get_connection().await?;
            connection
                .execute(
                    "CREATE TABLE IF NOT EXISTS test_table (
                    id UInt64,
                    name String,
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
