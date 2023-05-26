mod commands;
mod database;

use dotenv::dotenv;
use tokio;

use clap::{Parser, Subcommand};

/// zkSync db setup CLI
#[derive(Debug, Parser)]
#[command(name = "setup")]
#[command(about = "zkSync db setup CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// drop database
    #[command(arg_required_else_help = false)]
    Drop,

    /// setup database
    #[command(arg_required_else_help = false)]
    Init,

    /// re-init database, this will drop the tables and re-create them
    #[command(arg_required_else_help = false)]
    Reinit,

    /// start database
    #[command(arg_required_else_help = false)]
    Start,

    /// stop database
    #[command(arg_required_else_help = false)]
    Stop,

    /// destroy database data dir
    #[command(arg_required_else_help = false)]
    Destroy,
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
    }
}
