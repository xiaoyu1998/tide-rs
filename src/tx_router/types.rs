use serde::Deserialize;
use serde::Serialize;

// Define structs for request and response
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAndBuyRequest {
    pub network: String,
    pub contract: String,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub photo: String,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>,
    pub amount: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyRequest {
    pub network: String,
    pub contract: String,
    pub mint_str: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SellRequest {
    pub network: String,
    pub contract: String,
    pub mint_str: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub message: String,
}