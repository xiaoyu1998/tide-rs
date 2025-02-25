use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::hex::FromHex;
use alloy::network::Ethereum;

use std::env;
use dotenvy::dotenv;
use std::error::Error;

use alloy_primitives::{
    FixedBytes,
};

pub fn load_signer_from_file(path: &str) -> Result<PrivateKeySigner, Box<dyn Error>> {
    // Load environment variables from .env (if not already loaded)
    dotenv().ok();

    // Get private key from environment
    let private_key_str = env::var("PrivateKey").map_err(|_| "PrivateKey not found in environment")?;

    // Convert hex string to byte array
    let private_key_bytes = <FixedBytes<32>>::from_hex(private_key_str.trim())
        .map_err(|_| "Invalid private key hex format")?;

    // Create the signer
    let signer = PrivateKeySigner::<Ethereum>::from_bytes((&private_key_bytes).into())
        .map_err(|_| "Failed to create signer")?;

    Ok(signer)
}
