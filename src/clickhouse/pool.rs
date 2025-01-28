use eyre::{eyre, Result};
use klickhouse::*;

use super::config::ClickhouseConfig;
use super::migrations::MigrationManager;
use super::queries::LogsQueries;

// Queries
// use super::queries::trades_queries::TradesQueries;

#[derive(Debug, Clone)]
pub struct ClickhousePool {
    pool: bb8::Pool<ConnectionManager>,
}

pub type ClickhouseConnection<'a> = bb8::PooledConnection<'a, ConnectionManager>;

impl ClickhousePool {
    pub async fn connect(config: &ClickhouseConfig) -> Result<Self> {
        let options = ClientOptions {
            username: config.user.clone(),
            password: config.password.clone(),
            default_database: config.database.clone(),
            tcp_nodelay: true,
        };

        let database_url = format!("{}:{}", config.host, config.port);

        let manager = ConnectionManager::new(database_url, options)
            .await
            .map_err(|e| eyre!("Failed to create Clickhouse connection manager: {}", e))?;

        let pool = bb8::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .await
            .map_err(|e| eyre!("Failed to create Clickhouse connection pool: {}", e))?;

        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> Result<ClickhouseConnection> {
        let client = self.pool.get().await?;
        Ok(client)
    }

    pub async fn check_pool(&self) -> Result<()> {
        let client = self.get_connection().await?;
        let result = client
            .query_one::<UnitValue<u8>>("select 1")
            .await
            .map_err(|e| eyre!("Failed to check Clickhouse connection: {}", e))?;

        if result.0 == 1 {
            tracing::info!("Clickhouse connection is getting ok");
        } else {
            return Err(eyre!("Failed to check Clickhouse connection"));
        }

        Ok(())
    }

    pub async fn run_migrations(&self) -> Result<()> {
        let mut migration_manager = MigrationManager::new();
        migration_manager.register_all_migrations();
        migration_manager.run_migrations(self).await?;

        Ok(())
    }

    // Queries
    pub async fn get_logs_queries(&self) -> Result<LogsQueries> {
        let connection = self.get_connection().await?;
        Ok(LogsQueries::new(connection))
    }
}

#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    use super::*;

    #[tokio::test]
    async fn test_clickhouse_pool() {
        let clickhouse_config = AppConfig::build().get_clickhouse_config().unwrap();
        let pool = ClickhousePool::connect(&clickhouse_config).await.unwrap();

        pool.run_migrations().await.unwrap();
        pool.check_pool().await.unwrap();

        let version = MigrationManager::get_current_version(&pool).await.unwrap();
        assert!(version > 0);
    }
}
