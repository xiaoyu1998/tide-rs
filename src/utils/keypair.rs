use solana_sdk::{signature::Keypair};
use std::fs::File;
use std::path::Path;
use std::io::Read;

use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::hex::FromHex;
use alloy::network::Ethereum;

use std::env;
use dotenvy::dotenv;

/// Function to load the Solana keypair from the given file path
pub fn load_keypair_from_file(path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    // Resolve the home directory to handle "~"
    let expanded_path = shellexpand::tilde(path).to_string();
    let path = Path::new(&expanded_path);

    // Read the JSON file with the private key array
    let mut file = File::open(path)?;
    let mut private_key_bytes = Vec::new();
    file.read_to_end(&mut private_key_bytes)?;

    // Parse the file as a JSON array (assuming the private key is a JSON array of bytes)
    let private_key_vec: Vec<u8> = serde_json::from_slice(&private_key_bytes)?;

    // Deserialize the private key into a Keypair
    let keypair = Keypair::from_bytes(&private_key_vec)?;

    Ok(keypair)
}

pub fn load_signer_from_file(path: &str) -> Result<PrivateKeySigner<Ethereum>, Box<dyn Error>> {
    // Load environment variables from .env (if not already loaded)
    dotenv().ok();

    // Get private key from environment
    let private_key_str = env::var("PrivateKey").map_err(|_| "PrivateKey not found in environment")?;

    // Convert hex string to byte array
    let private_key_bytes = <[u8; 32]>::from_hex(private_key_str.trim())
        .map_err(|_| "Invalid private key hex format")?;

    // Create the signer
    let signer = PrivateKeySigner::<Ethereum>::from_bytes(&private_key_bytes)
        .map_err(|_| "Failed to create signer")?;

    Ok(signer)
}
