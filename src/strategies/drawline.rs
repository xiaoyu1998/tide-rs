use rand::Rng;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt};  // For reading input asynchronously
use tokio::signal::unix::{signal, SignalKind}; // For listening to Ctrl + C
use crate::tx_router::client_apis;
use crate::tx_router::margin_mm::utils;

use alloy_primitives::{
    U256,
};

pub async fn execute(
    network: String,
    market: String,
    token: String,
    price_ceiling: f64,
    price_floor: f64,
    tx_trade_size_max: u128, // For both buy and sell, this is the token amount
    tx_trade_size_min: u128, // For both buy and sell, this is the token amount
    trading_frequency: u64, // Time in seconds
    gas: u64,
    task_mode: u64,
    trend: u64,
    wallets: u64,
    //token_balance: f64, // The current token balance for sell
) -> Result<(), String> {
   let mut rng = rand::thread_rng();
    //let (tx, mut rx) = mpsc::channel(1);

    // // Input handling task
    // tokio::spawn(async move {
    //     let mut stdin = io::BufReader::new(io::stdin()).lines();
        
    //     loop {
    //         let line = stdin.next_line().await.unwrap();
    //         match line {
    //             Some(input) if input == " " => {
    //                 // Pause the bot when space is pressed
    //                 tx.send("pause".to_string()).await.unwrap();
    //                 println!("Bot paused. Press Enter to resume.");
    //             }
    //             Some(input) if input == "\n" => {
    //                 // Resume the bot when Enter is pressed
    //                 tx.send("resume".to_string()).await.unwrap();
    //                 println!("Bot resumed.");
    //             }
    //             _ => {}
    //         }
    //     }
    // });
   let (price, decimals) = match client_apis::get_price(network.clone(), market.clone(), token.clone()).await {
       Ok((price, decimals)) => (price, decimals),
       Err(e) => {
           println!("Failed to fetch price: {}", e);
           return Err(format!("Failed to fetch price: {}", e));
       }
   };


   let price_decimals = decimals - utils::USDT_DECIMALS;
   let price_ceiling_u256 = U256::from(price_ceiling * (10u64.pow(price_decimals/2)) as f64) * U256::from(10u64.pow((price_decimals+1)/2));
   let price_floor_u256 = U256::from(price_floor * (10u64.pow(price_decimals/2)) as f64) * U256::from(10u64.pow((price_decimals+1)/2));

   let tx_trade_size_max = tx_trade_size_max*10u128.pow(decimals);
   let tx_trade_size_min = tx_trade_size_min*10u128.pow(decimals);

    // Infinite loop to continuously trade
    loop {
        // Get the current price of the token
        let (current_price, price_decimals) = match client_apis::get_price(network.clone(), market.clone(), token.clone()).await {
            Ok((current_price, price_decimals)) => (current_price, price_decimals),
            Err(e) => return Err(format!("Failed to fetch price: {}", e)),
        };
        //println!("Current price {} ", current_price);

        // Check if the current price is outside the allowed range
        if current_price > price_ceiling || current_price < price_floor {
            // Pause trading if the price is above the price_ceiling or below the price_floor
            println!("Current price {} is out of range. Pausing trading.", current_price);
            
            loop {
                // Wait until the price returns within the acceptable range
                 let (current_price, price_decimals) = match client_apis::get_price(network.clone(), market.clone(), token.clone()).await {
                     Ok((current_price, price_decimals)) => (current_price, price_decimals),
                     Err(e) => return Err(format!("Failed to fetch price: {}", e)),
                 };

                if current_price <= price_ceiling && current_price >= price_floor {
                    println!("Price is within range. Resuming trading.");
                    break;
                }

                // Sleep for a while before checking the price again
                sleep(Duration::from_secs(5)).await; // Check every 5 seconds
            }
        }

        // Randomize the trade size for both buy and sell (in tokens) between min and max
        let sell_size_tokens = rng.gen_range(tx_trade_size_min..=tx_trade_size_max);
        let trade_size_tokens = sell_size_tokens/10u128.pow(decimals);

        // Randomly decide whether to buy or sell based on the price and trend
        let price_action = rng.gen_bool(0.5); // 50% chance to buy or sell (adjust as needed)

        if price_action {
            //let money_required_for_buy = current_price * trade_size_tokens as f64; // Money needed to buy the tokens
            let money_required_for_buy = U256::from(current_price * 10u64.pow(utils::USDT_DECIMALS) as f64 ) * U256::from(trade_size_tokens) ; // Money needed to buy the tokens

            // Execute buy
            match client_apis::buy(
                network.clone(),
                market.clone(),
                token.clone(),
                money_required_for_buy, // Using the calculated money amount for buy
                price_ceiling_u256,
            ).await {
                Ok(_) => println!("Buy trade executed: {} tokens for {} money at price {}", trade_size_tokens, money_required_for_buy, current_price),
                Err(e) => return Err(format!("Buy trade failed: {}", e)),
            }
        } else {

            // Execute sell
            match client_apis::sell(
                network.clone(),
                market.clone(),
                token.clone(),
                U256::from(sell_size_tokens), // Selling the token amount
                // price_floor_u256,
                U256::ZERO,
            ).await {
                Ok(_) => println!("Sell trade executed: {} tokens at price {}", trade_size_tokens, current_price),
                Err(e) => return Err(format!("Sell trade failed: {}", e)),
            }
        }

        // Sleep for the specified trading frequency in seconds (e.g., 1 second, 60 seconds)
        sleep(Duration::from_secs(trading_frequency)).await;

    }

    Ok(())
}
