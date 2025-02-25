use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyRequest {
    pub network: String,
    pub market: String,
    pub token: String,
    pub amount: f64,
    pub price_limit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellRequest {
    pub network: String,
    pub market: String,
    pub token: String,
    pub amount: f64,
    pub price_limit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub message: String,
}