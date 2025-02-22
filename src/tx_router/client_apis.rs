use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signature::{Keypair, Signature},
        signer::Signer,
        pubkey::Pubkey,
    },
    Cluster,
};

use pumpfun::{
    accounts::BondingCurveAccount, 
    utils::CreateTokenMetadata, 
    PriorityFee, 
    PumpFun,
    error::ClientError
};
use hex;
use reqwest::Client;
use crate::tx_router::types;

pub const TX_ROUTER_URL : &str = "http://127.0.0.1:3030";

pub async fn create_and_buy(
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

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/create_and_buy");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let create_and_buy_request = types::CreateAndBuyRequest {
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
    };

    println!("create_and_buy: {:?}", create_and_buy_request);

    // Send the POST request
    let response = client
        .post(url)
        .json(&create_and_buy_request)  // Serialize the BuyRequest struct into JSON
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
                Ok(())  // Or use response_body if needed
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
    mint_str: String,
    amount: f64
) -> Result<(), String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/buy");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let buy_request = types::BuyRequest {
        network,
        market,
        mint_str,
        amount,
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
                Ok(())  // Or use response_body if needed
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
    mint_str: String,
    amount: f64
) -> Result<(), String> {

    // Define the URL of the API endpoint
    let url = format!("{}{}", TX_ROUTER_URL, "/sell");

    // Create the client and send the request
    let client = Client::new();

    // Create the request body
    let sell_request = types::SellRequest {
        network,
        market,
        mint_str,
        amount,
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
                Ok(())  // Or use response_body if needed
            } else {
                Err(format!("Request failed with status: {}", res.status()))
            }
        }
        Err(e) => Err(format!("Request error: {}", e.to_string())),  // Convert reqwest::Error to String
    }
}



