
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
   // let first_half_32 = delta_decimals_u32/2;
   // let rest_half_32 = delta_decimals_u32 - first_half_u32;
   let meme_decimals_u32:u32 = meme_decimals.try_into().unwrap();

   match client_apis::sell(
       network,
       market,
       token,
       //U256::from(amount)*U256::from(10).pow(meme_decimals)
       utils::adjust_by_type(amount, meme_decimals_u32),
       utils::adjust_by_type(price_limit, delta_decimals_u32)
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("sell: {:?}", format!("Failed to sell: {}", e));

            Err(format!("Failed to sell: {}", e))
        }
   }

}