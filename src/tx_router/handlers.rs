use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use warp::Filter;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
//use base_sdk::signer::keypair::Keypair;
use tokio::task;
use crate::tx_router::types;
use crate::tx_router::margin_mm_apis;
use alloy_primitives::{
    U256,
};
use alloy::transports::http::reqwest::Url;
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    sol_types::private::{Address},
    providers::{Provider, ProviderBuilder}, 
};
use crate::utils::keypair;
use crate::tx_router::margin_mm::constants;
use crate::tx_router::margin_mm::contracts;
use crate::tx_router::margin_mm::utils;
use crate::tx_router::types::RouterState;


impl<P> RouterState<P>
where
    P: Provider<Ethereum> + Send + Sync + 'static,
{
    fn new(network: &str, market: &str, provider: Arc<P>, owner: Address) -> Result<Self, String> {

        let contracts = contracts::load_contracts("deployments/contracts.json");

        let data_store_address = contracts::get_contract_address(&contracts, "DataStore")
            .ok_or("Missing DataStore contract")?;
        let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter")
            .ok_or("Missing ExchangeRouter contract")?;
        let reader_address = contracts::get_contract_address(&contracts, "Reader")
            .ok_or("Missing Reader contract")?;
        let router_address = contracts::get_contract_address(&contracts, "Router")
            .ok_or("Missing Router contract")?;
        let base_address = contracts::get_contract_address(&contracts, "USDT")
            .ok_or("Missing USDT contract")?;

        Ok(Self {
            client: provider,
            network: network.to_string(),
            market: market.to_string(),
            contracts,
            owner,
            data_store_address,
            exchange_router_address,
            reader_address,
            router_address,
            base_address,
        })
    }
}

async fn add_handler<P>(
    state: Arc<RouterState<P>>, 
    body: types::AddRequest
) -> Result<warp::reply::Json, Infallible>
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    println!("add_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::add(state.clone(), body.token, U256::from(body.liquidity)).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok((liquidity, amount0, amount1)) => {
            let response = types::AddResponse {
                success: true,
                message: "Liquidity added successfully.".to_string(),
                liquidity: Some(liquidity),
                amount0: Some(amount0),
                amount1: Some(amount1),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::AddResponse {
                success: false,
                message: err,
                liquidity: None,
                amount0: None,
                amount1: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}
 
async fn buy_handler<P>(
    state: Arc<RouterState<P>>, 
    body: types::BuyRequest
) -> Result<warp::reply::Json, Infallible>
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    println!("buy_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::buy(state.clone(), body.token, body.amount, body.price_limit).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok((amount_in, amount_out)) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
                amount_in: Some(amount_in),
                amount_out: Some(amount_out),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                amount_in: None,
                amount_out: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Sell handler function
async fn sell_handler<P>(
    state: Arc<RouterState<P>>, 
    body: types::SellRequest
) -> Result<warp::reply::Json, Infallible>
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    println!("sell_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::sell(state, body.token, body.amount, body.price_limit).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok((amount_in, amount_out)) => {
            let response = types::Response {
                success: true,
                message: "Tokens sold successfully.".to_string(),
                amount_in: Some(amount_in),
                amount_out: Some(amount_out),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                amount_in: None,
                amount_out: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Get Price handler function
async fn get_pool_handler<P>(
    state: Arc<RouterState<P>>, 
    body: types::GetPoolRequest
) -> Result<warp::reply::Json, Infallible> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    println!("get_pool_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::get_pool(state, body.token).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(pool) => {
            let response = types::GetPoolResponse {
                success: true,
                message: "Tokens bought successfully.".to_string(),
                pool: Some(types::Pool{
                    price: pool.price,
                    price_decimals: pool.priceDecimals,
                    base_token: pool.assets[0].token,
                    base_symbol: pool.assets[0].symbol.clone(),
                    base_token_decimals: pool.assets[0].decimals,
                    meme_token: pool.assets[1].token,
                    meme_symbol: pool.assets[1].symbol.clone(),
                    meme_token_decimals: pool.assets[1].decimals,
                }),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::GetPoolResponse {
                success: false,
                message: err,
                pool: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn start(network:String, market:String) {
    
    let signer = keypair::load_signer_from_file(".env").map_err(|e| e.to_string()).unwrap();
    let wallet = EthereumWallet::from(signer.clone());
    let owner = wallet.default_signer().address();
    let rpc = Url::parse(constants::BASE_SEPOLIA).map_err(|e| e.to_string()).unwrap();
    let client = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc);

    let state = match RouterState::new(&network, &market, Arc::new(client.clone()), owner) {
        Ok(state) => Arc::new(state),
        Err(e) => {
            eprintln!("Failed to initialize RouterState: {}", e);
            return;
        }
    };

    let add_state = Arc::clone(&state);
    let buy_state = Arc::clone(&state);
    let sell_state = Arc::clone(&state);
    let get_price_state = Arc::clone(&state);

    let add_route = warp::path("add")
        .and(warp::post())
        .and(warp::any().map(move || Arc::clone(&add_state)))
        .and(warp::body::json())
        .and_then(add_handler);

    let buy_route = warp::path("buy")
        .and(warp::post())
        .and(warp::any().map(move || Arc::clone(&buy_state)))
        .and(warp::body::json())
        .and_then(buy_handler);

    let sell_route = warp::path("sell")
        .and(warp::post())
        .and(warp::any().map(move || Arc::clone(&sell_state)))
        .and(warp::body::json())
        .and_then(sell_handler);

    let get_pool_route = warp::path("get_pool")
        .and(warp::post())
        .and(warp::any().map(move || Arc::clone(&get_price_state)))
        .and(warp::body::json())
        .and_then(get_pool_handler);

    let routes = buy_route.or(sell_route).or(get_pool_route).or(add_route);

    println!("tx_router listening on: 127.0.0.1:3030");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}