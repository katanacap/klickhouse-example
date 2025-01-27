use clap::{Parser, Subcommand};

/// Main CLI structure
#[derive(Parser, Debug)]
#[command(
    name = "klickhouse_example",
    version,
    about = "Klickhouse example app with web server and db migrations"
)]
pub struct Cli {
    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run web server
    #[command(name = "serve")]
    Serve,
    /// Run migrations
    #[command(name = "migrate")]
    Migrate,
}
