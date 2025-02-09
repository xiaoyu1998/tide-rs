use anchor_client::{
    solana_sdk::{
        signature::Keypair,
    },
};

use std::fs::File;
use std::path::Path;
use std::io::Read;

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

#[derive(Debug)]
pub struct Instruction {
    pub action: String,
    pub params: Option<String>,
}

pub fn get_instrs(log: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut extracted_param: Option<String> = None;

    for entry in log.iter() {
        if entry.starts_with("Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA") {
            // Extract the last value after space
            let parts: Vec<&str> = entry.split_whitespace().collect();
            if let Some(param) = parts.last() {
                extracted_param = Some(param.to_string());
            }
        } else if entry.contains("SellVestedTokens") {
            instructions.push(Instruction { action: "sell".to_string(), params: None });
        } else if entry.contains("CreateVestedToken") {
            instructions.push(Instruction { action: "create".to_string(), params: None });
        } else if entry.contains("Buy") {
            instructions.push(Instruction { action: "buy".to_string(), params: None });
        }
    }

    // Attach the extracted parameter to "create"
    if let Some(param) = extracted_param {
        for instr in instructions.iter_mut() {
            if instr.action == "create" {
                instr.params = Some(param.clone());
            }
        }
    }

    instructions
}
