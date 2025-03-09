use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use alloy_primitives::{
    FixedBytes,
    Address,
    U256,
};

use alloy::{
    network::{ Ethereum},
    providers::{Provider}, 
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
    pub amount_in: Option<U256>,
    pub amount_out: Option<U256>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddRequest {
    pub network: String,
    pub market: String,
    pub token: String,
    pub liquidity: u64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddResponse {
    pub success: bool,
    pub message: String,
    pub liquidity: Option<U256>,
    pub amount0: Option<U256>,
    pub amount1: Option<U256>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    pub pools: HashMap<Address, Pool>,
}

#[derive(Clone)]
pub struct RouterState<P>
where
    P: Provider<Ethereum> + Send + Sync + 'static,
{
    pub client: Arc<P>, // Use trait object
    pub network: String,
    pub market: String,
    pub contracts: HashMap<String, Address>,
    pub owner: Address,
    pub data_store_address: Address,
    pub exchange_router_address: Address,
    pub reader_address: Address,
    pub router_address: Address,
    pub base_address: Address,
}

