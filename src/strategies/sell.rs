use crate::tx_router::client_apis;

pub async fn execute(
    network: String,
    market: String,
    token: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   match client_apis::sell(
       network,
       market,
       token,
       amount,
       price_limit
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("sell: {:?}", format!("Failed to sell: {}", e));

            Err(format!("Failed to sell: {}", e))
        }
   }

}