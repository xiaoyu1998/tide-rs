use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

// use crate::utils;

use crate::tx_router::client_apis::create_and_buy;

pub async fn execute(
    network: String,
    market: String,
    name: String,
    symbol: String,
    description: String,
    photo: String,
    twitter: Option<String>,
    telegram: Option<String>,
    website: Option<String>,
    amount: f64
) -> Result<(), String> {

   match create_and_buy(
       network,
       market,
       name,
       symbol,
       description,
       photo,
       twitter,
       telegram,
       website,
       amount
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            println!("create_and_buy: {:?}", format!("Failed to create and buy: {}", e));

            Err(format!("Failed to create and buy: {}", e))
        }
   }

}