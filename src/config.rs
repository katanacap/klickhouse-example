use confik::{Configuration, EnvSource, FileSource};
use eyre::Result;

use crate::clickhouse::config::ClickhouseConfig;
// use eyre::Result;

#[derive(Debug, PartialEq, Configuration, Clone)]
pub struct AppConfig {
    pub http_port: u16,
    pub log_level: String,

    // ## Clickhouse
    // -- public
    pub clickhouse_pool_size: u32,

    // -- private
    #[confik(secret)]
    pub clickhouse_host: String,
    #[confik(secret)]
    pub clickhouse_port: u16,
    #[confik(secret)]
    pub clickhouse_user: String,
    #[confik(secret)]
    pub clickhouse_password: String,
    #[confik(secret)]
    pub clickhouse_database: String,
}

impl AppConfig {
    pub fn build() -> Self {
        AppConfig::builder()
            .override_with(FileSource::new("confik.toml").allow_secrets())
            .override_with(EnvSource::new().allow_secrets())
            .try_build()
            .unwrap()
    }

    pub fn get_clickhouse_config(&self) -> Result<ClickhouseConfig> {
        let config = ClickhouseConfig {
            host: self.clickhouse_host.clone(),
            port: self.clickhouse_port,
            user: self.clickhouse_user.clone(),
            password: self.clickhouse_password.clone(),
            database: self.clickhouse_database.clone(),
            pool_size: self.clickhouse_pool_size,
        };
        config.validate()?;

        Ok(config)
    }
}
