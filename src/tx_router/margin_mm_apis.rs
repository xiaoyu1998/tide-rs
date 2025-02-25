use std::sync::Arc;
use std::str::FromStr;
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    sol_types::private::{Address},
    providers::{ProviderBuilder}, 
    abi::encode,
};
use alloy_primitives::{
    FixedBytes,
    U256,
};
use margin_mm::{
    reader::Reader,
    exchangerouter::{
        ExchangeRouter,
        SwapUtils,
    }

};
use crate::tx_router::margin_mm::contracts;
use crate::tx_router::margin_mm::utils;
use crate::utils::keypair;

pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";

pub async fn buy(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.signer_addresses()[0];

   let rpc = (BASE_SEPOLIA).parse().map_err(|e| e.to_string())?;
   let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("contracts.json");
   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "BASE").unwrap();

   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), client.clone());

   let pools = reader.getPools2(data_store_address,  vec![utils::hash_pool_key(base_address, meme_address)]).call().await.unwrap();
   let pools = pools._0;
   let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));

   let base_decimals = pools[0].assets[1].decimals;
   let amount0_in = (amount * base_decimals as f64) as U256;
   let amount1_out = if price_limit == U256::ZERO {
        U256::ZERO
   } else {
        (amount / price_limit) as U256
   };

   let params_send_tokens = ExchangeRouter::sendTokensCall {
        token: base_address,
        receiver: pools[0].bank,
        amount: amount0_in,
    };

   let params_swap = SwapUtils::SwapParams{
        token0: base_address,
        token1: meme_address,
        amount0In: amount0_in,
        amount1In: U256::ZERO,
        amount0Out: U256::ZERO,
        amount1Out: amount1_out,
        to: owner,
    };

    let multicallArgs = [
        encode(params_send_tokens.tokenize()),
        encode(params_swap.tokenize()),
    ];

    let call_build = exchange_router.multicall(multicallArgs);
    let mut tx = call_build.into_transaction_request();
    let _ = utils::send_transaction(tx).await?;

    Ok(()) 

}

pub async fn sell(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

    Ok(()) 

}