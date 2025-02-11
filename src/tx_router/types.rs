use serde::Deserialize;
use serde::Serialize;

// Define structs for request and response
#[derive(Serialize, Deserialize)]
pub struct CreateAndBuyRequest {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub photo: String,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>,
    pub amount_sol: u64
}

#[derive(Serialize, Deserialize)]
pub struct BuyRequest {
    pub mint_str: String,
    pub amount_sol: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SellRequest {
    pub mint_str: String,
    pub amount_sol: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: String,
}