mod strategies;
mod tx_router;
mod utils;

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
    /// Start Tx Rounter
    StartTxRouter,
    /// Buy
    Buy{
        #[arg(short, long, default_value_t = String::from("solana"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("pumpfun"))]
        contract: String, 

        #[arg(short, long)]
        mint_str: String,   

        #[arg(short, long)]
        amount: u64,             
    },
    /// Sell
    Sell{
        #[arg(short, long, default_value_t = String::from("solana"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("pumpfun"))]
        contract: String, 

        #[arg(short, long)]
        mint_str: String,   

        #[arg(short, long)]
        amount: u64,             
    },
    /// Monitor pumpfun launch raydium and buy
    MonitorCreateBuy {
        #[arg(short, long, default_value_t = String::from("solana"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("pumpfun"))]
        contract: String, 

        /// Name of the input file
        #[arg(short, long)]
        amount: u64,
    },
    /// Create Token and pumpfun pairs
    CreateBuy{
        #[arg(short, long, default_value_t = String::from("solana"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("pumpfun"))]
        contract: String, 

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

        #[arg(short = 'g', long)]
        telegram: Option<String>,  

        #[arg(short, long)]
        website: Option<String>,  

        #[arg(short, long)]
        amount: u64,             
    },
    /// start mm-tide
    Start,
}


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => {
            //let _ = start_server().await;
        }
        Commands::StartTxRouter => {
            let _ = tx_router::handlers::start().await;
        }
        Commands::Buy { network, contract, mint_str, amount } => {
            let _ = strategies::monitor_create_buy::execute(
                network.clone(),
                contract.clone(),
                *amount
            ).await;
        }
        Commands::Sell { network, contract, mint_str, amount } => {
            let _ = strategies::monitor_create_buy::execute(
                network.clone(),
                contract.clone(),
                *amount
            ).await;
        }
        Commands::MonitorCreateBuy { network, contract, amount } => {
            let _ = strategies::monitor_create_buy::execute(
                network.clone(),
                contract.clone(),
                *amount
            ).await;
        }
        Commands::CreateBuy { 
            network,
            contract,
            name,
            symbol,
            description,
            photo,
            twitter,
            telegram,
            website,
            amount
        } => {
            let _ = strategies::create_buy::execute(
                network.clone(),
                contract.clone(),
                name.clone(),
                symbol.clone(),
                description.clone(),
                photo.clone(),
                twitter.clone(),
                telegram.clone(),
                website.clone(),
                *amount
            ).await;;
        }
        Commands::Create { numbers } => {
            let _ = strategies::create_account::execute(*numbers);
        }
    }

     Ok(())

}