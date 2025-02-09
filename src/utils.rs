use anchor_client::{
    solana_sdk::{
        signature::Keypair,
    },
};

use std::fs::File;
use std::path::Path;
use std::io::Read;

use solana_transaction_status::{
    EncodedTransactionWithStatusMeta,
    UiMessage, 
    EncodedTransaction, 
};

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
    pub params: String,
}

pub fn get_instrs(log: &Vec<String>, account_keys: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut create_found = false;
    let mut buy_found = false;
    let mut params = String::new();

    // Iterate through the log entries
    for entry in log.iter() {
        if entry.contains("Instruction: Create") {
            create_found = true;
            params = account_keys[1].clone(); // Store params from the create action
        } else if entry.contains("SellVestedTokens") {
            instructions.push(Instruction { action: "sell".to_string(), params: account_keys[2].clone() });
        } else if entry.contains("Buy") {
            buy_found = true;
            // We don't add the "buy" instruction directly
        }
    }

    // After the loop, handle the merging and individual cases
    if create_found && buy_found {
        instructions.push(Instruction { action: "create_buy".to_string(), params });
    } else {
        // If only create was found, add create instruction
        if create_found {
            instructions.push(Instruction { action: "create".to_string(), params });
        }

        // If only buy was found, add buy instruction
        if buy_found {
            instructions.push(Instruction { action: "buy".to_string(), params: account_keys[2].clone() });
        }
    }

    instructions
}

pub fn get_accounts(tx: &EncodedTransactionWithStatusMeta) -> Vec<String> {

    if let EncodedTransaction::Json(uiTransaction) = &tx.transaction {
        if let UiMessage::Raw(uiRawMessage) = &uiTransaction.message{
            return uiRawMessage.account_keys.clone();
        }

    }

    Vec::new()
}
