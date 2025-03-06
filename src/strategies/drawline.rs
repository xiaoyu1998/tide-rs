use rand::Rng;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt};  // For reading input asynchronously
use tokio::signal::unix::{signal, SignalKind}; // For listening to Ctrl + C
use std::str::FromStr;
use alloy::{
    sol_types::private::{Address},
};
use alloy_primitives::{U256};
use crate::tx_router::client_apis;
use crate::tx_router::margin_mm::utils;
use crate::tx_router::margin_mm::cache;
use crate::tx_router::margin_mm::constants;


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

   let cache = cache::load_or_create_cache(network.clone(), market.clone(), token.clone()).await?;
   let pool = cache.pools.get(&Address::from_str(&token.clone()).unwrap()).unwrap();
   let meme_decimals_u256 = pool.meme_token_decimals;
   let base_decimals_u256 = pool.base_token_decimals;
   let delta_decimals_u256 = meme_decimals_u256 - base_decimals_u256;

   let delta_decimals_u32:u32 = delta_decimals_u256.try_into().unwrap();
   let base_decimals_u32:u32 = base_decimals_u256.try_into().unwrap();
   let price_ceiling_u256 = utils::mul_pow_2_half(price_ceiling, constants::PRICE_DECIMALS);
   let price_floor_u256 = utils::mul_pow_2_half(price_floor, constants::PRICE_DECIMALS);

   let meme_decimals_u32:u32 = meme_decimals_u256.try_into().unwrap();
   let tx_trade_size_max = tx_trade_size_max*10u128.pow(meme_decimals_u32);
   let tx_trade_size_min = tx_trade_size_min*10u128.pow(meme_decimals_u32);

    // Infinite loop to continuously trade
    loop {
        // Get the current price of the token
        let current_price_u256 = match client_apis::get_pool(network.clone(), market.clone(), token.clone()).await {
            Ok(pool) => pool.price,
            Err(e) => return Err(format!("Failed to fetch price: {}", e)),
        };

        // Check if the current price is outside the allowed range
        if current_price_u256 > price_ceiling_u256 || current_price_u256 < price_floor_u256 {
            // Pause trading if the price is above the price_ceiling_u256 or below the price_floor_u256
            println!("Current price {} is out of range. Pausing trading.", current_price_u256);
            
            loop {
                // Wait until the price returns within the acceptable range
                 let current_price_u256 = match client_apis::get_pool(network.clone(), market.clone(), token.clone()).await {
                     Ok(pool) => pool.price,
                     Err(e) => return Err(format!("Failed to fetch price: {}", e)),
                 };

                if current_price_u256 <= price_ceiling_u256 && current_price_u256 >= price_floor_u256 {
                    println!("Price is within range. Resuming trading.");
                    break;
                }

                // Sleep for a while before checking the price again
                sleep(Duration::from_secs(5)).await; // Check every 5 seconds
            }
        }

        // Randomize the trade size for both buy and sell (in tokens) between min and max
        let sell_size_tokens_u128 = rng.gen_range(tx_trade_size_min..=tx_trade_size_max);
        let trade_size_tokens_u128 = sell_size_tokens_u128/10u128.pow(meme_decimals_u32);
        let sell_size_tokens_u256 = U256::from(sell_size_tokens_u128);

        // Randomly decide whether to buy or sell based on the price and trend
        let price_action = rng.gen_bool(0.5); // 50% chance to buy or sell (adjust as needed)

        if price_action {
            //let money_required_for_buy = current_price * trade_size_tokens as f64; // Money needed to buy the tokens
            let base_units_required_for_buy_u256 = utils::get_base_units_from_tokens(
                sell_size_tokens_u256, 
                current_price_u256,
                delta_decimals_u256
            ); 

            // Execute buy
            match client_apis::buy(
                network.clone(),
                market.clone(),
                token.clone(),
                base_units_required_for_buy_u256, // Using the calculated money amount for buy
                utils::div_pow(price_ceiling_u256, delta_decimals_u256)
            ).await {
                Ok((amount_in, amount_out)) => println!("Buy trade executed: {} {}  {} {}  {}", 
                    pool.meme_symbol,
                    utils::format_to_f32(amount_out,meme_decimals_u256),  
                    pool.base_symbol,
                    utils::format_to_f32(amount_in, base_decimals_u256), 
                    utils::calc_price(amount_in, base_decimals_u256, amount_out, meme_decimals_u256)
                ),
                Err(e) => return Err(format!("Buy trade failed: {}", e)),
            }
        } else {

            // Execute sell
            match client_apis::sell(
                network.clone(),
                market.clone(),
                token.clone(),
                sell_size_tokens_u256, // Selling the token amount
                utils::div_pow(price_floor_u256, delta_decimals_u256)
            ).await {
                Ok((amount_in, amount_out)) => println!("Sell trade executed: {} {}  {} {}  {}",
                    pool.meme_symbol, 
                    utils::format_to_f32(amount_in,meme_decimals_u256),  
                    pool.base_symbol,
                    utils::format_to_f32(amount_out, base_decimals_u256),  
                    utils::calc_price(amount_out, base_decimals_u256, amount_in, meme_decimals_u256)
                ),
                Err(e) => return Err(format!("Sell trade failed: {}", e)),
            }
        }

        // Sleep for the specified trading frequency in seconds (e.g., 1 second, 60 seconds)
        sleep(Duration::from_secs(trading_frequency)).await;

    }

    Ok(())
}
