use std::collections::HashMap;
use std::fs;
use serde_json::Value;

use alloy::{
    sol_types::private::{Address},
};

/// Reads the JSON file and parses it into a HashMap
pub fn load_contracts(file_path: &str) -> HashMap<String, Address> {
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let json: HashMap<String, Address> = serde_json::from_str(&data).expect("Invalid JSON format");
 
    return json;
    // json.as_object()
    //     .expect("JSON is not an object")
    //     .iter()
    //     .map(|(key, value)| (key.clone(), value.clone()))
    //     .collect()
}

/// Gets the contract address by key (e.g., `"RoleStore"` → `"RoleStore#RoleStore"`)
pub fn get_contract_address(contracts: &HashMap<String, Address>, key: &str) -> Option<Address> {
    let formatted_key = format!("{}#{}", key, key); // Convert "RoleStore" → "RoleStore#RoleStore"
    contracts.get(&formatted_key).map(|&addr| addr)
}

// fn main() {
//     let contracts = load_contracts("contracts.json");

//     if let Some(address) = get_contract_address(&contracts, "RoleStore") {
//         println!("RoleStore address: {}", address);
//     } else {
//         println!("Contract cnot found.");
//     }
// }
