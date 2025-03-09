use reqwest::Client;
use crate::tx_router::types;
use alloy_primitives::{
    U256,
};

pub const TX_ROUTER_URL : &str = "http://127.0.0.1:3030";

pub async fn add(
    network: String,
    market: String,
    token: String,
    liquidity: u64,
) -> Result<(U256, U256, U256), String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/add");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let add_request = types::AddRequest {
        network,
        market,
        token,
        liquidity
    };

    // Send the POST request
    let response = client
        .post(url)
        .json(&add_request)  // Serialize the BuyRequest struct into JSON
        .send()
        .await;

    // Handle errors properly
    match response {
        Ok(res) => {
            // Check if the response status is OK (status code 200)
            if res.status().is_success() {
                let response_body: types::AddResponse = res.json()
                    .await
                    .map_err(|e| format!("Error deserializing response: {}", e))?;  // Handle json error
                match (response_body.liquidity, response_body.amount0, response_body.amount1) {
                    (Some(liquidity), Some(amount0), Some(amount1)) => Ok((liquidity, amount0, amount1)),
                    _ => Err("Missing amount0 or amount1".to_string()),  // Error if any is None
                }
            } else {
                Err(format!("Request failed with status: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Request error: {}", e.to_string())),  // Convert reqwest::Error to String
    }
}

pub async fn buy(
    network: String,
    market: String,
    token: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/buy");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let buy_request = types::BuyRequest {
        network,
        market,
        token,
        amount,
        price_limit,
    };

    // Send the POST request
    let response = client
        .post(url)
        .json(&buy_request)  // Serialize the BuyRequest struct into JSON
        .send()
        .await;

    // Handle errors properly
    match response {
        Ok(res) => {
            // Check if the response status is OK (status code 200)
            if res.status().is_success() {
                let response_body: types::Response = res.json()
                    .await
                    .map_err(|e| format!("Error deserializing response: {}", e))?;  // Handle json error
                match (response_body.amount_in, response_body.amount_out) {
                    (Some(amount_in), Some(amount_out)) => Ok((amount_in, amount_out)),
                    _ => Err("Missing amount_in or amount_out".to_string()),  // Error if any is None
                }
                //Ok((response_body.amount_in, response_body.amount_out))  // Or use response_body if needed
            } else {
                Err(format!("Request failed with status: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Request error: {}", e.to_string())),  // Convert reqwest::Error to String
    }
}

pub async fn sell(
    network: String,
    market: String,
    token: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/sell");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let sell_request = types::SellRequest {
        network,
        market,
        token,
        amount,
        price_limit,
    };

    // Send the POST request
    let response = client
        .post(url)
        .json(&sell_request)  // Serialize the SellRequest struct into JSON
        .send()
        .await;

    // Handle errors properly
    match response {
        Ok(res) => {
            // Check if the response status is OK (status code 200)
            if res.status().is_success() {
                let response_body: types::Response = res.json()
                    .await
                    .map_err(|e| format!("Error deserializing response: {}", e))?;  // Handle json error
                match (response_body.amount_in, response_body.amount_out) {
                    (Some(amount_in), Some(amount_out)) => Ok((amount_in, amount_out)),
                    _ => Err("Missing amount_in or amount_out".to_string()),  // Error if any is None
                }
                //Ok((response_body.amount_in, response_body.amount_out))  // Or use response_body if needed
            } else {
                Err(format!("Request failed with status: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Request error: {}", e.to_string())),  // Convert reqwest::Error to String
    }
}

pub async fn get_pool(
    network: String,
    market: String,
    token: String,
) -> Result<types::Pool, String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/get_pool");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let get_pool_request = types::GetPoolRequest {
        network,
        market,
        token,
    };

    // Send the POST request
    let response = client
        .post(url)
        .json(&get_pool_request)  // Serialize the GetPriceRequest struct into JSON
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                // Deserialize response body into the correct struct
                let response_body: types::GetPoolResponse = res.json()
                    .await
                    .map_err(|e| format!("Error deserializing response: {}", e))?;

                if response_body.success {
                    match response_body.pool {
                        Some(pool) => Ok(pool),  // Return both values
                        _ => Err("Pool not available.".to_string()),  // Handle missing data
                    }
                } else {
                    Err(response_body.message)  // Return the error message
                }
            } else {
                Err(format!("Request failed with status: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Request error: {}", e.to_string())),  // Handle request errors
    }
}



