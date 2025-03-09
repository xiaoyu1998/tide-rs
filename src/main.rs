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
    StartTxRouter{
         #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 
    },
    /// Add
    Add{
        #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(short, long)]
        token: String,   

        #[arg(short, long)]
        liquidity: u64,  
    },
    /// Remove
    Remove{
        #[arg(short, long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(short, long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(short, long)]
        token: String,   

        #[arg(short, long)]
        liquidity: u64,  
    },
    /// Drawline
    Drawline{
        #[arg(long, default_value_t = String::from("base"))]
        network: String,   

        #[arg(long, default_value_t = String::from("marginmm"))]
        market: String, 

        #[arg(long)]
        token: String,   

        #[arg(long)]
        price_ceiling: f64,  

        #[arg(long)]
        price_floor: f64,  

        #[arg(long)]
        tx_trade_size_max: u128,  

        #[arg(long)]
        tx_trade_size_min: u128,   

        #[arg(long)]
        trading_frequency: u64,    

        #[arg(long)]
        gas: u64,     

        #[arg(long)]
        task_mode: u64, 
 
        #[arg(long)]
        trend: u64,    

        #[arg(long)]
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

        #[arg(short, long)]
        price_limit: f64, 
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

        #[arg(short, long, default_value_t = 0.0)]
        price_limit: f64, 
            
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
        Commands::StartTxRouter { network, market }=> {
            let _ = tx_router::handlers::start(network.clone(), market.clone()).await;
        }
        Commands::Remove { network, market, token, liquidity } => {
            let _ = strategies::add::execute(
                network.clone(),
                market.clone(),
                token.clone(),
                *liquidity,
            ).await;
        }
        Commands::Add { network, market, token, liquidity } => {
            let _ = strategies::add::execute(
                network.clone(),
                market.clone(),
                token.clone(),
                *liquidity,
            ).await;
        }
        Commands::Buy { network, market, token, amount, price_limit } => {
            let _ = strategies::buy::execute(
                network.clone(),
                market.clone(),
                token.clone(),
                *amount,
                *price_limit,
            ).await;
        }
        Commands::Sell { network, market, token, amount, price_limit } => {
            let _ = strategies::sell::execute(
                network.clone(),
                market.clone(),
                token.clone(),
                *amount,
                *price_limit,
            ).await;
        }
        Commands::Drawline { 
            network, 
            market, 
            token, 
            price_ceiling, 
            price_floor,
            tx_trade_size_max,
            tx_trade_size_min,
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
                *tx_trade_size_max,
                *tx_trade_size_min,
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