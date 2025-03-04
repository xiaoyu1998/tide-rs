
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use crate::tx_router::types;

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

    println!("Read state cache from file");
    Ok(cache)
}

// Save state cache to file
fn save_cache(cache: &types::StateCache) -> Result<()> {
    let file = File::create(STATE_CACHE_FILE)
        .with_context(|| format!("Failed to create cache file: {}", STATE_CACHE_FILE))?;
    serde_json::to_writer_pretty(file, &cache)
        .with_context(|| "Failed to write state cache to file")?;

    Ok(())
}

pub async fn load_or_create_cache(
    network: String,
    market: String,
    token: String
) -> Result<types::StateCache, String> {

    let cache = match load_cache() {
        Ok(cache) => {
            println!("Successfully loaded cache: {:?}", cache);
            cache
        }
        Err(e) => {
            println!("Failed to load cache: {}", e);
            println!("Creating a new empty cache...");
            let cache = types::StateCache {
                tokens: HashMap::new(),
            };

            match client_apis::get_pool(
                network,
                market,
                token
            ).await {
                Ok(pool) => {
                    cache.tokens.insert(
                        pool.meme_token,
                        Pool
                    );  
                },
                Err(e) => {
                    Err(format!("Failed to get_pool: {}", e))
                }
            }

            // Try saving the new cache
            if let Err(save_err) = save_cache(&cache) {
                println!("Failed to save new cache: {}", save_err);
            } else {
                println!("New cache saved successfully.");
            }

            cache // Return the newly created empty cache
        }
    };

    Ok(cache)

}