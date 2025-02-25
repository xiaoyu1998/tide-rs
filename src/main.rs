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
    /// Start Tx Rounter
    StartTxRouter,
    
    /// Drawline
    Drawline{
        #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(short, long)]
        token: String,   

        #[arg(short, long)]
        price_ceiling: f64,  

        #[arg(short, long)]
        price_floor: f64,  

        #[arg(short, long)]
        tx_trade_size_Max: u64,  

        #[arg(short, long)]
        tx_trade_size_Min: u64,   

        #[arg(short, long)]
        trading_frequency: u64,    

        #[arg(short, long)]
        gas: u64,     

        #[arg(short, long)]
        task_mode: u64, 
 
        #[arg(short, long)]
        trend: u64,    

        #[arg(short, long)]
        wallets: u64,              
    },
    /// Buy
    Buy{
        #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(short, long)]
        token: String,   

        #[arg(short, long)]
        amount: f64,  

    },
    /// Sell
    Sell{
        #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(short, long)]
        token: String,   

        #[arg(short, long, default_value_t = 0.0)]
        amount: f64, 
            
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
        Commands::Buy { network, market, token, amount, limit_price } => {
            let _ = strategies::buy::execute(
                network.clone(),
                market.clone(),
                *amount
            ).await;
        }
        Commands::Sell { network, market, token, amount, limit_price } => {
            let _ = strategies::sell::execute(
                network.clone(),
                market.clone(),
                *amount
            ).await;
        }
        Commands::Drawline { 
            network, 
            market, 
            token, 
            price_ceiling, 
            price_floor,
            tx_trade_size_Max,
            tx_trade_size_Min,
            trading_frequency,
            gas,
            task_mode,
            trend,
            wallets,
        } => {
            let _ = strategies::drawline::execute(
                network.clone(),
                market.clone(),
                token.clone(),
                *price_ceiling,
                *price_floor,
                *tx_trade_size_Max,
                *tx_trade_size_Min,
                *trading_frequency,
                *gas,
                *task_mode,
                *trend,
                *wallets,
            ).await;
        }
    }

     Ok(())

}