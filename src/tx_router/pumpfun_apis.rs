use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use hex;

use crate::utils::keypair;

use solana_client::rpc_client::{RpcClient};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Signer, Keypair, Signature},
};

use crate::tx_router::pumpfun::{
    accounts::BondingCurveAccount, 
    utils::CreateTokenMetadata, 
    PriorityFee, 
    PumpFun,
    error::ClientError
};

pub const SOLANA_DEVNET_URL : &str = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS";


fn hex_to_pubkey(mint_str: &String) -> Result<Pubkey, String> {
    // Convert the string to a 32-byte array
    let mint_bytes = match hex::decode(mint_str.to_string()) {
        Ok(bytes) if bytes.len() == 32 => bytes,
        Ok(_) => return Err("Invalid pubkey length".to_string()),
        Err(err) => return Err(format!("Failed to decode pubkey string: {}", err)),
    };

    // Create Pubkey from the 32-byte array
    let pubkey = Pubkey::new_from_array(mint_bytes.try_into().unwrap());
    Ok(pubkey)
}

pub async fn create_and_buy(
    name: String,
    symbol: String,
    description: String,
    photo: String,
    twitter: Option<String>,
    telegram: Option<String>,
    website: Option<String>,
    amount_sol: f64
) -> Result<(), String> {
    //let payer: Keypair = Keypair::new();
    let payer = keypair::load_keypair_from_file("~/.config/solana/id.json").expect("Failed to load keypair");
    let public_key = payer.pubkey();
    println!("Loaded Solana Address: {}", public_key);

    //let SOLANA_DEVNET_URL = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS".to_string();

    // let rpc = RpcClient::new(&SOLANA_DEVNET_URL);

    // let amount = 2_000_000_000; // 2 SOL (in lamports)
    // match rpc.request_airdrop(&public_key, amount) {
    //     Ok(sig) => println!("Airdrop requested. Transaction Signature: {}", sig),
    //     Err(err) => eprintln!("Airdrop failed: {:?}", err),
    // }

    let client: PumpFun<'_> = PumpFun::new(&SOLANA_DEVNET_URL.to_string(), &payer, None);
    //dbg!(Cluster::Devnet.url());

    // Mint keypair
    let mint: Keypair = Keypair::new();
    dbg!(&mint);

    // Token metadata
    let metadata: CreateTokenMetadata = CreateTokenMetadata {
        name: name,
        symbol: symbol,
        description: description,
        file: photo,
        twitter: twitter,
        telegram: telegram,
        website: website,
    };

    let fee: Option<PriorityFee> = Some(PriorityFee {
        limit: Some(200_000),
        price: Some(100_000_000),
    });  

    let amount_lamports: u64 = (LAMPORTS_PER_SOL as f64 * amount_sol).round() as u64;
    println!("Amount in SOL: {}", amount_sol);
    println!("Amount in LAMPORTS: {}", amount_lamports);

     // Create and buy tokens with metadata
    let signature: Signature = client.create_and_buy(&mint, metadata.clone(), amount_lamports, None, fee).await.unwrap();
    println!("Created and bought tokens: {}", signature); 

    Ok(())  

}

pub async fn buy(
    mint_str: String,
    amount_sol: f64
) -> Result<(), String> {

    let path = "~/.config/solana/id.json";
    let payer = keypair::load_keypair_from_file(path).expect("Failed to load keypair");
    let public_key = payer.pubkey();
    println!("Loaded Solana Address: {}", public_key);
    //dbg!(rpc.get_balance(&public_key).unwrap());

    //let SOLANA_DEVNET_URL = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS".to_string();
    let client: PumpFun<'_> = PumpFun::new(&SOLANA_DEVNET_URL.to_string(), &payer, None);

    // Mint keypair
    let mint: Pubkey = match hex_to_pubkey(&mint_str) {
        Ok(pubkey) => pubkey,  // Create Pubkey from byte vector
        Err(err) => {
            //println!("Failed to decode pubkey string: {}", err);
            return Err(format!("Failed to decode pubkey string: {}", err));  // Return on error
        }
    };

    let fee: Option<PriorityFee> = Some(PriorityFee {
        limit: Some(200_000),
        price: Some(100_000_000),
    });  

    let amount_lamports: u64 = (LAMPORTS_PER_SOL as f64 * amount_sol).round() as u64;
    println!("Amount in SOL: {}", amount_sol);
    println!("Amount in LAMPORTS: {}", amount_lamports);

    // // Buy tokens (ATA will be created automatically if needed)
    let signature: Signature = client.buy(&mint, amount_lamports, None, fee).await.unwrap();
    println!("Bought tokens: {}", signature);

    Ok(()) 

}

pub async fn sell(
    mint_str: String,
    amount: f64
) -> Result<(), String> {

    let path = "~/.config/solana/id.json";
    let payer = keypair::load_keypair_from_file(path).expect("Failed to load keypair");
    let public_key = payer.pubkey();
    println!("Loaded Solana Address: {}", public_key);
    //dbg!(rpc.get_balance(&public_key).unwrap());

    //let SOLANA_DEVNET_URL = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS".to_string();
    let client: PumpFun<'_> = PumpFun::new(&SOLANA_DEVNET_URL.to_string(), &payer, None);

    // Mint keypair
    let mint: Pubkey = match hex_to_pubkey(&mint_str) {
        Ok(pubkey) => pubkey,  // Create Pubkey from byte vector
        Err(err) => {
            //println!("Failed to decode pubkey string: {}", err);
            return Err(format!("Failed to decode pubkey string: {}", err));  // Return on error
        }
    };

    let fee: Option<PriorityFee> = Some(PriorityFee {
        limit: Some(200_000),
        price: Some(100_000_000),
    });  

    let amount_meme = if amount as u64 == 0 {
        None
    } else {
        Some(amount as u64)
    };

    // Sell tokens (sell all tokens)
    let signature: Signature = client.sell(&mint, amount_meme, None, fee).await.unwrap();
    println!("Sold tokens: {}", signature);

    Ok(()) 

}

