use serde::{Serialize, Deserialize};
use alloy_primitives::{
    U256,
};

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
    pub price: Option<f64>, 
    pub price_decimals: Option<u32>, 
}


// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Token {
//     address: Address,
//     symbol: U256,
//     decimals: U256,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pool {
    price: U256,
    price_decimals: U256,
    base_token: Address,
    base_symbol: String,
    base_token_decimals: U256,
    meme_token: Address,
    meme_symbol: String,
    meme_token_decimals: U256,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPoolRequest {
    pub network: String,
    pub market: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPoolResponse {
    pub success: bool,
    pub message: String,
    pub pool: Option<Pool>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    pools: HashMap<Address, Pool>,
}

