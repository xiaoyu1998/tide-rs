
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use crate::tx_router::types;
use crate::tx_router::client_apis;
use alloy::{
    sol_types::private::{Address},
};
use std::str::FromStr;

const STATE_CACHE_FILE: &str = "state_cache.json";
// load borrower state cache from file if exists
fn load_cache() -> Result<types::StateCache, String> {
    if !Path::new(STATE_CACHE_FILE).exists() {
        return Err("No state cache file found".to_string());
    }

    let file = File::open(STATE_CACHE_FILE)
        .map_err(|e| format!("Failed to open cache file: {}", e))?;
    let cache: types::StateCache = serde_json::from_reader(file)
        .map_err(|e| format!("Failed to deserialize state cache: {}", e))?;

    //println!("Read state cache from file");
    Ok(cache)
}

// Save state cache to file
fn save_cache(cache: &types::StateCache) -> Result<(), String> {
    let file = File::create(STATE_CACHE_FILE)
        .map_err(|e| format!("Failed to create cache file: {}", e))?;
    serde_json::to_writer_pretty(file, &cache)
        .map_err(|e| format!("Failed to write state cache to file: {}", e))?;

    Ok(())
}

pub async fn load_or_create_cache(
    network: String,
    market: String,
    token: String
) -> Result<types::StateCache, String> {
    let mut cache = match load_cache() {
        Ok(mut cache) => cache,
        Err(e) => {
            println!("Failed to load cache: {}", e);
            println!("Creating a new empty cache...");
            types::StateCache {
                pools: HashMap::new(),
            }
        }
    };

    let token_address = match Address::from_str(&token) {
        Ok(addr) => addr,
        Err(_) => return Err("Invalid token address".to_string()),
    };

    if !cache.pools.contains_key(&token_address) {
        println!("Pool not found in cache, fetching from API...");
        match client_apis::get_pool(network.clone(), market.clone(), token.clone()).await {
            Ok(pool) => {
                cache.pools.insert(pool.meme_token, pool);
                if let Err(save_err) = save_cache(&cache) {
                    println!("Failed to save updated cache: {}", save_err);
                } else {
                    println!("Updated cache saved successfully.");
                }
            },
            Err(e) => return Err(format!("Failed to fetch pool: {}", e)),
        }
    }
    
    Ok(cache)
}

// pub async fn load_or_create_cache(
//     network: String,
//     market: String,
//     token: String
// ) -> Result<types::StateCache, String> {

//     let cache = match load_cache() {
//         Ok(cache) => {
//             //println!("Successfully loaded cache: {:?}", cache);
//             cache
//         }
//         Err(e) => {
//             println!("Failed to load cache: {}", e);
//             println!("Creating a new empty cache...");
//             let mut cache = types::StateCache {
//                 pools: HashMap::new(),
//             };

//             match client_apis::get_pool(
//                 network,
//                 market,
//                 token
//             ).await {
//                 Ok(pool) => {
//                     cache.pools.insert(
//                         pool.meme_token,
//                         pool
//                     );  
//                 },
//                 Err(e) => {
//                     return Err(format!("Failed to get_pool: {}", e));
//                 }
//             }

//             // Try saving the new cache
//             if let Err(save_err) = save_cache(&cache) {
//                 println!("Failed to save new cache: {}", save_err);
//             } else {
//                 println!("New cache saved successfully.");
//             }

//             cache // Return the newly created empty cache
//         }
//     };

//     Ok(cache)

// }
