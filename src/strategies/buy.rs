use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

// use crate::utils;

use crate::tx_router::client_apis;

pub async fn execute(
    network: String,
    market: String,
    mint_str: String,
    amount: f64
) -> Result<(), String> {

   match client_apis::buy(
       network,
       market,
       mint_str,
       amount
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("buy: {:?}", format!("Failed to buy: {}", e));

            Err(format!("Failed to buy: {}", e))
        }
   }

}