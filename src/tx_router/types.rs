use serde::{Serialize, Deserialize};
use std::collections::{HashMap};
use alloy_primitives::{
    FixedBytes,
    Address,
    U256,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPriceRequest {
    pub network: String,
    pub market: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyRequest {
    pub network: String,
    pub market: String,
    pub token: String,
    pub amount: U256,
    pub price_limit: U256,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellRequest {
    pub network: String,
    pub market: String,
    pub token: String,
    pub amount: U256,
    pub price_limit: U256,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub message: String,
    // pub price: Option<f64>, 
    // pub price_decimals: Option<u32>, 
    pub amount_in: Option<U256>,
    pub amount_out: Option<U256>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    pub price: U256,
    pub price_decimals: U256,
    pub base_token: Address,
    pub base_symbol: String,
    pub base_token_decimals: U256,
    pub meme_token: Address,
    pub meme_symbol: String,
    pub meme_token_decimals: U256,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPoolRequest {
    pub network: String,
    pub market: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPoolResponse {
    pub success: bool,
    pub message: String,
    pub pool: Option<Pool>, 
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Swap {
//     pub amount_in: U256,
//     pub amount_out: U256,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    pub pools: HashMap<Address, Pool>,
}

