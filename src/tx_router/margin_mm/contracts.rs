use std::collections::HashMap;
use std::fs;
use serde_json::Value;

/// Reads the JSON file and parses it into a HashMap
pub fn load_contracts(file_path: &str) -> HashMap<String, String> {
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON format");

    json.as_object()
        .expect("JSON is not an object")
        .iter()
        .map(|(key, value)| (key.clone(), value.as_str().unwrap().to_string()))
        .collect()
}

/// Gets the contract address by key (e.g., `"RoleStore"` → `"RoleStore#RoleStore"`)
pub fn get_contract_address(contracts: &HashMap<String, String>, key: &str) -> Option<&String> {
    let formatted_key = format!("{}#{}", key, key); // Convert "RoleStore" → "RoleStore#RoleStore"
    contracts.get(&formatted_key)
}

// fn main() {
//     let contracts = load_contracts("contracts.json");

//     if let Some(address) = get_contract_address(&contracts, "RoleStore") {
//         println!("RoleStore address: {}", address);
//     } else {
//         println!("Contract not found.");
//     }
// }
