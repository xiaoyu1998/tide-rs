use crate::tx_router::client_apis;
use alloy_primitives::{
    U256,
};

use crate::tx_router::margin_mm::cache;

pub async fn execute(
    network: String,
    market: String,
    token: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   let cache = cache::load_or_create_cache(network, market, token).await?;
   let pool = cache.pools.get(token);
   let meme_decimals = pool.meme_token_decimals;
   let base_decimals = pool.base_token_decimals;
   let decimals = meme_decimals - base_decimals;
   let first_half = decimals/2;
   let rest_half = decimals - first_half;

   match client_apis::sell(
       network,
       market,
       token,
       U256::from(amount)*U256::from(10u64.pow(meme_decimals)),
       U256::from(price_limit * (10u64.pow(first_half)) as f64) * U256::from(10u64.pow(rest_half))
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("sell: {:?}", format!("Failed to sell: {}", e));

            Err(format!("Failed to sell: {}", e))
        }
   }

}