use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use hex;
use crate::tx_router::margin_meme::contracts;
use crate::tx_router::margin_meme::utils;

pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    providers::ProviderBuilder, 
};

pub async fn buy(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".PrivateKey.txt").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let rpc = BASE_SEPOLIA;
   let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);

   let owner = wallet.PublicKey();
   let contracts = load_contracts("contracts.json");
   let data_store_address = contracts::get_contract_address("DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address("ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address("Reader").unwrap();
   let base_address = contracts::get_contract_address("BASE").unwrap();

   let meme_address = Address.from(meme);
   let reader = Reader::new(reader_address.clone(), client.clone());

   let pools = reader.getPoolsInfo_2(data_store_address, [hash_pool_key(base_address, meme_address)]).call().await.unwrap();
   let pool = pools[0];
   let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone());

   let base_decimals = pool.decimals;
   let amount0_in = (amount * base_decimals as f64) as U256;
   let amount1_out = if price_limit == U256::ZERO {
        U256::ZERO
   } else {
        (amount / price_limit) as U256
   };

   let params_send_tokens: SwapUtils.sendTokensCall = {
        token: base_address,
        receiver: owner.address,
        amount0In: amount0_in,
    };

   let params_swap: SwapUtils.SwapParams = {
        token0: base_address,
        token1: meme_address,
        amount0In: amount0_in,
        amount1In: U256::ZERO,
        amount0Out: U256::ZERO,
        amount1Out: amount1_out,
        to: owner.address,
    };

    let multicallArgs = [
        params_send_tokens.encode(),
        params_swap.encode(),
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