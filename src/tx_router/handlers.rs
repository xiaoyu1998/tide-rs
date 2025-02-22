use warp::Filter;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
//use solana_sdk::signer::keypair::Keypair;
use tokio::task;

use crate::tx_router::pumpfun_apis;
use crate::tx_router::types;

// Buy handler function
async fn create_and_buy_handler(body: types::CreateAndBuyRequest) -> Result<warp::reply::Json, Infallible> {
    println!("create_and_buy_handler: {:?}", body);
    let result = if body.network == "solana" && body.market == "pumpfun" {
        pumpfun_apis::create_and_buy(
            body.name, 
            body.symbol,
            body.description,
            body.photo,
            body.twitter,
            body.telegram,
            body.website,
            body.amount
        ).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(_) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Buy handler function
async fn buy_handler(body: types::BuyRequest) -> Result<warp::reply::Json, Infallible> {
    println!("buy_handler: {:?}", body);
    let result = if body.network == "solana" && body.market == "pumpfun" {
        pumpfun_apis::buy(body.mint_str, body.amount).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(_) => {
            let response = types::Response {
                success: true,
                message: "Tokens bought successfully.".to_string(),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// Sell handler function
async fn sell_handler(body: types::SellRequest) -> Result<warp::reply::Json, Infallible> {
    println!("sell_handler: {:?}", body);
    let result = if body.network == "solana" && body.market == "pumpfun" {
        pumpfun_apis::sell(body.mint_str, body.amount).await
    } else {
        Err("Network and market mismatch".to_string())
    };

    match result {
        Ok(_) => {
            let response = types::Response {
                success: true,
                message: "Tokens sold successfully.".to_string(),
            };
            Ok(warp::reply::json(&response))
        }
        Err(err) => {
            let response = types::Response {
                success: false,
                message: err,
            };
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn start() {
    //Define the routes for both `buy` and `sell` functions
    let create_and_buy_route = warp::path("create_and_buy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create_and_buy_handler);

    let buy_route = warp::path("buy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(buy_handler);

    let sell_route = warp::path("sell")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sell_handler);

    // Combine both routes
    let routes = buy_route.or(sell_route).or(create_and_buy_route);
    //let routes = create_and_buy_route;

    println!("tx_router listening on: 127.0.0.1:3030");

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
