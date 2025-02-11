use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use crate::utils;

use crate::tx_router::pumpfun_apis::create_and_buy;

pub async fn execute(
    name: String,
    symbol: String,
    description: String,
    photo: String,
    twitter: Option<String>,
    telegram: Option<String>,
    website: Option<String>,
    amount_sol: u64
) -> Result<(), String> {

   match create_and_buy(
       name,
       symbol,
       description,
       photo,
       twitter,
       telegram,
       website,
       amount_sol
   ).await {
        Ok(_) => Ok(()), // If successful, return Ok
        Err(e) => {
            // Handle error (print/log it, or return a custom error message)
            Err(format!("Failed to create and buy: {}", e))
        }
   }

}