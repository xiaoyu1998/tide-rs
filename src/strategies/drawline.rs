use rand::Rng;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use tokio::io::{self, AsyncBufReadExt};  // For reading input asynchronously
use tokio::signal::unix::{signal, SignalKind}; // For listening to Ctrl + C

// Assume `get_price` is a function that returns the current price from the market.
async fn get_price(network: String, market: String, token: String) -> Result<f64, String> {
    // Placeholder for the actual implementation of fetching the price
    Ok(100.0) // Example: Return a dummy price of 100 for testing
}

pub async fn execute(
    network: String,
    market: String,
    token: String,
    price_ceiling: f64,
    price_floor: f64,
    tx_trade_size_Max: u64, // For both buy and sell, this is the token amount
    tx_trade_size_Min: u64, // For both buy and sell, this is the token amount
    trading_frequency: u64, // Time in seconds
    gas: u64,
    task_mode: u64,
    trend: u64,
    wallets: u64,
    token_balance: f64, // The current token balance for sell
) -> Result<(), String> {

    let mut rng = rand::thread_rng();
    let (tx, mut rx) = mpsc::channel(1);

    // Input handling task
    tokio::spawn(async move {
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        
        loop {
            let line = stdin.next_line().await.unwrap();
            match line {
                Some(input) if input == " " => {
                    // Pause the bot when space is pressed
                    tx.send("pause".to_string()).await.unwrap();
                    println!("Bot paused. Press Enter to resume.");
                }
                Some(input) if input == "\n" => {
                    // Resume the bot when Enter is pressed
                    tx.send("resume".to_string()).await.unwrap();
                    println!("Bot resumed.");
                }
                _ => {}
            }
        }
    });

    // Set up Ctrl+C (SIGINT) listener
    let mut sigint = signal(SignalKind::interrupt()).await.unwrap();

    // Infinite loop to continuously trade
    loop {
        // Get the current price of the token
        let current_price = match get_price(network.clone(), market.clone(), token.clone()).await {
            Ok(price) => price,
            Err(e) => return Err(format!("Failed to fetch price: {}", e)),
        };

        // Check if the current price is outside the allowed range
        if current_price > price_ceiling || current_price < price_floor {
            // Pause trading if the price is above the price_ceiling or below the price_floor
            println!("Current price {} is out of range. Pausing trading.", current_price);
            
            loop {
                // Wait until the price returns within the acceptable range
                let current_price = match get_price(network.clone(), market.clone(), token.clone()).await {
                    Ok(price) => price,
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
        let trade_size_tokens = rng.gen_range(tx_trade_size_Min..=tx_trade_size_Max);

        // Randomly decide whether to buy or sell based on the price and trend
        let price_action = rng.gen_bool(0.5); // 50% chance to buy or sell (adjust as needed)

        if price_action {
            // Buy trade: Trading money for tokens at price_ceiling
            let price_to_buy = price_ceiling; // Buying at the price_ceiling

            let money_required_for_buy = price_to_buy * trade_size_tokens as f64; // Money needed to buy the tokens

            // Execute buy
            match client_apis::buy(
                network.clone(),
                market.clone(),
                token.clone(),
                money_required_for_buy as u64, // Using the calculated money amount for buy
                price_to_buy,
            ).await {
                Ok(_) => println!("Buy trade executed: {} tokens for {} money at price {}", trade_size_tokens, money_required_for_buy, price_to_buy),
                Err(e) => return Err(format!("Buy trade failed: {}", e)),
            }
        } else {
            // Sell trade: Trading tokens for money at price_floor
            if token_balance < trade_size_tokens as f64 {
                return Err("Insufficient token balance to sell".into());
            }

            let price_to_sell = price_floor; // Selling at the price_floor

            // Execute sell
            match client_apis::sell(
                network.clone(),
                market.clone(),
                token.clone(),
                trade_size_tokens, // Selling the token amount
                price_to_sell,
            ).await {
                Ok(_) => println!("Sell trade executed: {} tokens at price {}", trade_size_tokens, price_to_sell),
                Err(e) => return Err(format!("Sell trade failed: {}", e)),
            }
        }

        // Sleep for the specified trading frequency in seconds (e.g., 1 second, 60 seconds)
        sleep(Duration::from_secs(trading_frequency)).await;

        // Check for Ctrl+C to exit gracefully
        if sigint.pending().await {
            println!("Ctrl+C pressed. Exiting...");
            break;  // Exit the loop if Ctrl + C is pressed
        }
    }

    Ok(())
}
