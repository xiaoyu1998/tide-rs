use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

// use crate::utils;

use crate::tx_router::client_apis;

pub async fn execute(
    network: String,
    market: String,
    token: String,
    price_ceiling: f64,
    price_floor: f64,
    tx_trade_size_Max: u64,
    tx_trade_size_Min: u64,
    trading_frequency: u64,
    gas: u64,
    task_mode: u64,
    trend: u64,
    wallets: u64,
) -> Result<(), String> {

   // match client_apis::sell(
   //     network,
   //     market,
   //     token,
   //     amount
   // ).await {
   //      Ok(_) => Ok(()), // If successful, return Ok
   //      Err(e) => {
   //          // Handle error (print/log it, or return a custom error message)
   //          println!("sell: {:?}", format!("Failed to sell: {}", e));

   //          Err(format!("Failed to sell: {}", e))
   //      }
   // }
   Ok(())

}