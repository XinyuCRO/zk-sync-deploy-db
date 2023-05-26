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
    Setup,

    /// start database
    #[command(arg_required_else_help = false)]
    Start,

    /// stop database
    #[command(arg_required_else_help = false)]
    Stop,
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
        Commands::Drop => {
            println!("Droping DB...");
            match db.drop().await {
                Ok(_) => println!("DB dropped"),
                Err(e) => println!("DP drop failed, {}", e),
            }
        }
        Commands::Setup => {
            println!("Setting up DB");
            match db.setup().await {
                Ok(_) => println!("DB setup complete"),
                Err(e) => println!("DB setup failed, {}", e),
            }
        }
        Commands::Start => {
            println!("Starting DB");
            commands::stop();
            commands::start();
        }
        Commands::Stop => {
            println!("Stopping DB");
            commands::stop();
        }
    }
}
