use std::str::FromStr;
use alloy_primitives::{U256};
use alloy::{
    sol_types::private::{Address},
};
use crate::tx_router::{
    client_apis,
    margin_mm::{
        cache,
        utils,
    }
};
pub async fn execute(
    network: String,
    market: String,
    token: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   let cache = cache::load_or_create_cache(network.clone(), market.clone(), token.clone()).await?;
   let pool = cache.pools.get(&Address::from_str(&token.clone()).unwrap()).unwrap();
   let meme_decimals = pool.meme_token_decimals;
   let base_decimals = pool.base_token_decimals;
   let delta_decimals = meme_decimals - base_decimals;
   let delta_decimals_u32 :u32 = delta_decimals.try_into().unwrap();
   let base_decimals_u32:u32 = base_decimals.try_into().unwrap();

   dbg!(base_decimals_u32, delta_decimals_u32);

   match client_apis::buy(
       network,
       market,
       token,
       // U256::from(amount)*U256::from(10u64.pow(6)),
       // U256::from(price_limit * (10u64.pow(12/2)) as f64) * U256::from(10u64.pow((12+1)/2)),
       utils::adjust_by_type(amount, base_decimals_u32),
       utils::adjust_by_type(price_limit, delta_decimals_u32)
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("buy: {:?}", format!("Failed to buy: {}", e));

            Err(format!("Failed to buy: {}", e))
        }
   }

}