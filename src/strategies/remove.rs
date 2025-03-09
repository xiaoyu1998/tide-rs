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
    liquidity: u64,
) -> Result<(), String> {

   match client_apis::remove(
       network,
       market,
       token,
       liquidity
   ).await {
        Ok((amount0, amount1)) => {
            println!("remove trade executed: {} {}",  amount0, amount1);
            Ok(()) // If successful, return Ok
        },
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("remove: {:?}", format!("Failed to remove: {}", e));

            Err(format!("Failed to remove: {}", e))
        }
   }

}