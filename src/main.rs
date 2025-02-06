mod account;
use account::{create_account};

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
        twitter: String,  

        #[arg(short, long)]
        telegram: String,  

        #[arg(short, long)]
        website: String,  

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
            let _ = create_account(*numbers);
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
            let _ = create_buy(
                *name,
                *symbol,
                *description,
                *photo,
                *twitter,
                *telegram,
                *website
                *amount_sol
            );
        }
        Commands::MonitorCreateBuy { amount_sol } => {
            let _ = monitor_create_buy(*amount_sol);
        }
        Commands::Start => {
            //let _ = start_server().await;
        }
    }

     Ok(())

}