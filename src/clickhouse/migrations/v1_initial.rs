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
                    "CREATE TABLE IF NOT EXISTS trades (
                    market_type LowCardinality(String),
                    symbol LowCardinality(String),
                    trade_id UInt64,
                    price Decimal(18, 8),
                    quantity Decimal(18, 8),
                    quote_quantity Decimal(18, 8),
                    is_buyer_maker Boolean,
                    timestamp DateTime
                ) ENGINE = MergeTree()
                PARTITION BY (market_type, toYYYYMM(timestamp))
                ORDER BY (market_type, symbol, timestamp)
                SETTINGS index_granularity = 8192;",
                )
                .await?;

            Ok(())
        })
    }
}
