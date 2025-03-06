
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
use crate::tx_router::margin_mm::constants;

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
   let meme_decimals_u32:u32 = meme_decimals.try_into().unwrap();

   match client_apis::sell(
       network,
       market,
       token,
       utils::mul_pow_2_half(amount, meme_decimals_u32),
       utils::div_pow(utils::mul_pow_2_half(price_limit, constants::PRICE_DECIMALS), delta_decimals)
   ).await {
        Ok((amount_in, amount_out)) => {
            println!("Sell trade executed: {} {}  {} {}  {}", 
                pool.meme_symbol, 
                utils::format_to_f32(amount_in,meme_decimals), 
                pool.base_symbol, 
                utils::format_to_f32(amount_out, base_decimals),  
                utils::calc_price(amount_out, base_decimals, amount_in, meme_decimals)
            );
            Ok(())// If successful, return Ok
        }, 
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("sell: {:?}", format!("Failed to sell: {}", e));

            Err(format!("Failed to sell: {}", e))
        }
   }

}