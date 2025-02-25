use crate::tx_router::client_apis;

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
       amount,
       price_limit,
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("buy: {:?}", format!("Failed to buy: {}", e));

            Err(format!("Failed to buy: {}", e))
        }
   }

}