use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use crate::utils;

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Cluster,
};

use pumpfun::{
    accounts::BondingCurveAccount, 
    utils::CreateTokenMetadata, 
    PriorityFee, 
    PumpFun,
    error::ClientError
};
// use std::fs::File;
// use std::path::Path;
// use std::io::Read;

// /// Function to load the Solana keypair from the given file path
// fn load_keypair_from_file(path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
//     // Resolve the home directory to handle "~"
//     let expanded_path = shellexpand::tilde(path).to_string();
//     let path = Path::new(&expanded_path);

//     // Read the JSON file with the private key array
//     let mut file = File::open(path)?;
//     let mut private_key_bytes = Vec::new();
//     file.read_to_end(&mut private_key_bytes)?;

//     // Parse the file as a JSON array (assuming the private key is a JSON array of bytes)
//     let private_key_vec: Vec<u8> = serde_json::from_slice(&private_key_bytes)?;

//     // Deserialize the private key into a Keypair
//     let keypair = Keypair::from_bytes(&private_key_vec)?;

//     Ok(keypair)
// }

pub async fn execute(
    name: String,
    symbol: String,
    description: String,
    photo: String,
    twitter: Option<String>,
    telegram: Option<String>,
    website: Option<String>,
    amount_sol: u64
) {

    let path = "~/.config/solana/id.json";
    let payer = utils::load_keypair_from_file(path).expect("Failed to load keypair");
    let public_key = payer.pubkey();
    println!("Loaded Solana Address: {}", public_key);

    let rpc = RpcClient::new(Cluster::Devnet.url());
    dbg!(rpc.get_balance(&public_key).unwrap());

    let client: PumpFun<'_> = PumpFun::new(Cluster::Devnet, &payer, None, None);
    dbg!(Cluster::Devnet.url());

    // Mint keypair
    let mint: Keypair = Keypair::new();

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

    let amount_lamports: u64 = LAMPORTS_PER_SOL * amount_sol;
    println!("Amount in SOL: {}", amount_sol);
    println!("Amount in LAMPORTS: {}", amount_lamports);

     // Create and buy tokens with metadata
    let signature: Signature = client.create_and_buy(&mint, metadata.clone(), amount_lamports, None, fee).await.unwrap();
    println!("Created and bought tokens: {}", signature);   

}

