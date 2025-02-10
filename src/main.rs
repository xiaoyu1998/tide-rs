mod create_account;
mod create_buy;
mod monitor_create_buy;
mod utils;
mod monitor;
mod pumpfun_api;
mod tx_parser;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::env;

use clap::{Parser, Subcommand};

/// My Project with Subcommands
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create accounts
    Create {
        /// Name of the input file
        #[arg(short, long)]
        numbers: u64,
    },

    /// Create Token and pumpfun pairs
    CreateBuy{
        #[arg(short, long)]
        name: String,   

        #[arg(short, long)]
        symbol: String,    

        #[arg(short, long)]
        description: String,  

        #[arg(short, long)]
        photo: String,  

        #[arg(short, long)]
        twitter: Option<String>,  

        #[arg(short, long)]
        telegram: Option<String>,  

        #[arg(short, long)]
        website: Option<String>,  

        #[arg(short, long)]
        amount_sol: u64,             
    },

    /// Monitor pumpfun launch raydium and buy
    MonitorCreateBuy {
        /// Name of the input file
        #[arg(short, long)]
        amount_sol: u64,
    },
    // /// Convert a file to a specific format
    // Convert {
    //     /// Input file name
    //     #[arg(short, long)]
    //     input: String,

    //     /// Output format
    //     #[arg(short, long)]
    //     format: String,
    // },
    /// start mm-tide
    Start,
}


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { numbers } => {
            let _ = create_account::execute(*numbers);
        }
        Commands::CreateBuy { 
            name,
            symbol,
            description,
            photo,
            twitter,
            telegram,
            website,
            amount_sol
        } => {
            let _ = create_buy::execute(
                name.clone(),
                symbol.clone(),
                description.clone(),
                photo.clone(),
                twitter.clone(),
                telegram.clone(),
                website.clone(),
                *amount_sol
            );
        }
        Commands::MonitorCreateBuy { amount_sol } => {
            let _ = monitor_create_buy::execute(*amount_sol).await;
        }
        Commands::Start => {
            //let _ = start_server().await;
        }
    }

     Ok(())

}