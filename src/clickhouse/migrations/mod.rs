// Migrations
pub mod v1_initial;
pub mod v2_web_logs_table;

// External dependencies
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use eyre::{eyre, Result};
use klickhouse::*;
// Local dependencies
use super::ClickhousePool;

// Types
type BoxedFuture<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

pub trait Migration {
    /// Returns the migration version number
    fn version(&self) -> i32;
    /// Returns the migration name
    fn name(&self) -> &str;

    /// Applies the migration
    fn apply<'a>(&self, client: &'a ClickhousePool) -> BoxedFuture<'a>;
}

#[derive(Row, Debug)]
pub struct MigrationModel {
    pub version: i32,
    pub applied_at: DateTime,
}

#[derive(Default)]
pub struct MigrationManager {
    migrations: HashMap<i32, Box<dyn Migration>>,
}

impl MigrationManager {
    pub fn new() -> Self {
        Self {
            migrations: HashMap::new(),
        }
    }

    pub fn register_all_migrations(&mut self) {
        self.register_migration(Box::new(v1_initial::InitialMigration));
        self.register_migration(Box::new(v2_web_logs_table::LogsTableMigration));
    }

    /// Register a new migration
    pub fn register_migration(&mut self, migration: Box<dyn Migration>) {
        let version = migration.version();
        if self.migrations.insert(version, migration).is_some() {
            panic!("Migration version {} is already registered", version);
        }
    }

    pub async fn run_migrations(&self, pool: &ClickhousePool) -> Result<()> {
        let connection = pool.get_connection().await?;

        // Ensure that the `schema_migrations` table exists
        let create_migrations_table = r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version Int32,
                applied_at DateTime
            ) ENGINE = MergeTree()
            ORDER BY (version);
        "#;
        connection.execute(create_migrations_table).await?;

        // Get the list of already applied migrations
        let applied_versions = connection
            .query_collect::<MigrationModel>("SELECT version, applied_at FROM schema_migrations")
            .await?
            .into_iter()
            .map(|m| m.version)
            .collect::<Vec<i32>>();

        // Get all registered migrations
        let mut pending_versions: Vec<i32> = self.migrations.keys().cloned().collect();
        pending_versions.sort();

        for applied_version in &applied_versions {
            if !self.migrations.contains_key(applied_version) {
                return Err(eyre!(
                    "Migration version {} is not registered",
                    applied_version
                ));
            }
        }

        // Run pending migrations in a transaction

        for version in pending_versions {
            if !applied_versions.contains(&version) {
                let migration = self.migrations.get(&version).unwrap();

                // Log migration start
                tracing::info!("Running migration v{}: {}", version, migration.name());

                // Run migration
                migration.apply(pool).await?;

                let migration_model_row = MigrationModel {
                    version,
                    applied_at: chrono::Utc::now().try_into().unwrap(),
                };

                connection
                    .insert_native_block(
                        "INSERT INTO schema_migrations FORMAT native",
                        vec![migration_model_row],
                    )
                    .await?;
            }
        }

        tracing::info!("All migrations applied successfully!");
        Ok(())
    }

    // Get current database version
    pub async fn get_current_version(pool: &ClickhousePool) -> Result<i32> {
        let connection = pool.get_connection().await?;

        #[derive(Row)] // Row for mapping
        struct VersionRow {
            version: i32,
        }

        let result = connection
            .query_one::<VersionRow>("select max(version) as version from schema_migrations;")
            .await?;

        Ok(result.version)
    }
}
