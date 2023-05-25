mod commands;
mod database;

use std::io;

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
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Cli::parse();

    let datbase = database::Database::new(
        std::env::var("POSTGRES_USER").unwrap(),
        std::env::var("POSTGRES_PASSWORD").unwrap(),
        std::env::var("POSTGRES_HOST").unwrap(),
        std::env::var("POSTGRES_PORT").unwrap(),
        std::env::var("POSTGRES_DATABASE").unwrap(),
    );

    match args.command {
        Commands::Drop => {
            println!("Do you want to continue? (y/n)");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            if input.trim() == "y" {
                println!("Droping DB...");
                datbase.drop().await.unwrap();
            } else {
                println!("Exiting...");
                // Add your code here for when the user does not confirm with "y"
            }
        }
        Commands::Setup => {
            println!("Setting up DB");
        }
    }
}
