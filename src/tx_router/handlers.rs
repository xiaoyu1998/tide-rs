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


// Buy handler function
async fn buy_handler(body: types::BuyRequest) -> Result<warp::reply::Json, Infallible> {
    println!("buy_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::buy(body.token, body.amount, body.price_limit).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(_) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
                price: None,
                price_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                price: None,
                price_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Sell handler function
async fn sell_handler(body: types::SellRequest) -> Result<warp::reply::Json, Infallible> {
    println!("sell_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::sell(body.token, body.amount, body.price_limit).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(_) => {
            let response = types::Response {
                success: true,
                message: "Tokens sold successfully.".to_string(),
                price: None,
                price_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                price: None,
                price_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Get Price handler function
async fn get_price_handler(body: types::GetPriceRequest) -> Result<warp::reply::Json, Infallible> {
    println!("get_price_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::get_price(body.token).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok((price, price_decimals)) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
                price: Some(price),
                price_decimals: Some(price_decimals),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                price: None,
                price_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Get Price handler function
async fn get_pool_handler(body: types::GetPriceRequest) -> Result<warp::reply::Json, Infallible> {
    println!("get_pool_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm_apis::get_pool(body.token).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(pool) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
                pool: Pool{
                    price: pool.price,
                    price_decimals: pool.priceDecimals,
                    base_token: pool.assets[0].token,
                    base_symbol: pool.assets[0].symbol.clone(),
                    base_token_decimals: pool.assets[0].decimals,
                    meme_token: pool.assets[1].token,
                    meme_symbol: pool.assets[1].symbol.clone(),
                    meme_token_decimals: pool.assets[1].decimals,
                },
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
                pool: None,
                pool_decimals: None,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn start() {
    //Define the routes for both `buy` and `sell` functions
    let buy_route = warp::path("buy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(buy_handler);

    let sell_route = warp::path("sell")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sell_handler);

    let get_price_route = warp::path("get_price")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(get_price_handler);

    let get_price_route = warp::path("get_pool")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(get_pool_handler);

    // Combine both routes
    let routes = buy_route.or(sell_route).or(get_price_route);
    //let routes = create_and_buy_route;

    println!("tx_router listening on: 127.0.0.1:3030");

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
