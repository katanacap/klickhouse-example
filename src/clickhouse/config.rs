use eyre::{eyre, Result};

pub struct ClickhouseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_size: u32,
}

impl ClickhouseConfig {
    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        database: &str,
        pool_size: u32,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            pool_size,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty()
            || self.port == 0
            || self.user.is_empty()
            || (self.user != "default" && self.password.is_empty())
            || self.database.is_empty()
            || self.pool_size == 0
        {
            return Err(eyre!("Invalid Clickhouse configuration"));
        }
        Ok(())
    }
}
