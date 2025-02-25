use warp::Filter;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
//use base_sdk::signer::keypair::Keypair;
use tokio::task;
use crate::tx_router::types;
use crate::tx_router::margin_mm;

// Buy handler function
async fn buy_handler(body: types::BuyRequest) -> Result<warp::reply::Json, Infallible> {
    println!("buy_handler: {:?}", body);
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm::buy(body.mint_str, body.amount, 0.0).await
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
    let result = if body.network == "base" && body.market == "marginmm" {
        margin_mm::sell(body.mint_str, body.amount, 0.0).await
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
    let buy_route = warp::path("buy")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(buy_handler);

    let sell_route = warp::path("sell")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(sell_handler);

    // Combine both routes
    let routes = buy_route.or(sell_route);
    //let routes = create_and_buy_route;

    println!("tx_router listening on: 127.0.0.1:3030");

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
