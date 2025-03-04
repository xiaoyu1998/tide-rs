use crate::tx_router::client_apis;
use alloy_primitives::{
    U256,
};
pub async fn execute(
    network: String,
    market: String,
    token: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   match client_apis::buy(
       network,
       market,
       token,
       U256::from(amount)*U256::from(10u64.pow(6)),
       U256::from(price_limit * (10u64.pow(12/2)) as f64) * U256::from(10u64.pow((12+1)/2)),
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("buy: {:?}", format!("Failed to buy: {}", e));

            Err(format!("Failed to buy: {}", e))
        }
   }

}