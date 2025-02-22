use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use hex;



pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";


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

pub async fn exeute(
    tx: String,
) -> Result<(), String> {

}

