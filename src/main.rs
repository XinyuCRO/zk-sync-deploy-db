mod commands;
mod database;
mod utils;

use dotenv::dotenv;
use tokio;

use clap::{Parser, Subcommand};

/// CLI struct for zkSync db setup
#[derive(Debug, Parser)]
#[command(name = "setup")]
#[command(about = "zkSync db setup CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Drop database
    Drop,

    /// Setup database
    Init,

    /// Run database migrations
    Migrate,

    /// Re-init database, this will drop the tables and re-create them
    Reinit,

    /// Start database
    Start,

    /// Stop database
    Stop,

    /// Destroy database data dir
    Destroy,

    /// Check database status, print all tables
    Check,

    /// Sync schemas changes from upstream
    Sync,

    /// Print connection link to the db
    Print
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Cli::parse();

    let db = database::Database::new(
        std::env::var("POSTGRES_USER").unwrap(),
        std::env::var("POSTGRES_PASSWORD").unwrap(),
        std::env::var("POSTGRES_HOST").unwrap(),
        std::env::var("POSTGRES_PORT").unwrap(),
        std::env::var("POSTGRES_DATABASE").unwrap(),
    );

    match args.command {
        Commands::Drop => commands::drop(&db).await,
        Commands::Init => commands::setup(&db).await,
        Commands::Reinit => {
            println!("Re-initing DB...");
            commands::drop(&db).await;
            commands::setup(&db).await;
        }
        Commands::Start => {
            println!("Starting DB...");
            commands::stop();
            commands::start();
        }
        Commands::Stop => {
            println!("Stopping DB...");
            commands::stop();
        }
        Commands::Destroy => {
            println!("Destroying DB...");
            commands::stop();
            commands::destroy();
        }
        Commands::Check => commands::check(&db).await,
        Commands::Sync => {
            commands::sync().await
        }
        Commands::Migrate => {
            commands::migrate(&db).await
        }
        Commands::Print => {
            commands::print(&db)
        }
    }
}
