use std::sync::Arc;
use std::str::FromStr;
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    sol_types::private::{Address},
    providers::{ProviderBuilder}, 
};
use alloy::sol_types::SolValue;
use alloy::sol_types::SolCall;
use alloy::transports::http::reqwest::Url;

use alloy_primitives::{
    Bytes,
    FixedBytes,
    U256,
};
use margin_mm::{
    reader::Reader,
    exchangerouter::{
        ExchangeRouter,
        SwapUtils,
    },
    erc20::{
        ERC20,
    },
};
use crate::tx_router::margin_mm::contracts;
use crate::tx_router::margin_mm::utils;
use crate::utils::keypair;

//pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";
//pub const chain_id: u64 = 84532;
pub const BASE_SEPOLIA : &str = "http://127.0.0.1:8545";
pub const chain_id: u64 = 31337;
pub const usdt_decimals: u32 = 6;

pub async fn buy(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {
   let power = 10u64.pow(usdt_decimals);
   let amount0_in = U256::from((amount * power as f64) as f64);
   let amount1_out = if U256::from(price_limit as u64) == U256::ZERO {
        U256::ZERO
   } else {
        U256::from((amount / price_limit) as u64)
   };

   return swap(meme, amount0_in, U256::ZERO, U256::ZERO, amount1_out).await;
}

pub async fn sell(
    meme: String,
    amount: f64,
    price_limit: f64,
) -> Result<(), String> {

    let amount1_in = U256::from(amount as u64);
    let amount0_out = if U256::from(price_limit as u64) == U256::ZERO {
        U256::ZERO
    } else {
        U256::from((amount * price_limit) as u64)
    }; 

    return swap(meme, U256::ZERO, amount1_in, amount0_out, U256::ZERO).await;
}

pub async fn swap(
    meme: String,
    amount0_in: U256,
    amount1_in: U256,
    amount0_out: U256,
    amount1_out: U256,
) -> Result<(), String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.default_signer().address();
   // let chain_id: u64 = 84532;
   //let chain_id: u64 = 31337;

   //let rpc = (BASE_SEPOLIA).parse().map_err(|e| e.to_string())?;
   let rpc = Url::parse(BASE_SEPOLIA).map_err(|e| e.to_string())?;
   let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("deployments/contracts.json");
   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let router_address = contracts::get_contract_address(&contracts, "Router").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();

   dbg!(owner, data_store_address, exchange_router_address, reader_address, base_address);

   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), client.clone());
   let pook_key = utils::hash_pool_key(base_address, meme_address);

   dbg!(meme_address, pook_key);

   let pools = reader.getPools2(data_store_address,  vec![pook_key]).call().await.unwrap();
   let pools = pools._0;

   let (erc20_address, amount_in) = if amount0_in != U256::ZERO && amount1_in == U256::ZERO {
       (base_address, amount0_in)
    } else if amount1_in != U256::ZERO && amount0_in == U256::ZERO {
       (meme_address, amount1_in)
    } else {
       return Err("Invalid amounts: both amount0_in and amount1_in are non-zero".to_string());
    };

    println!("approve {} {}", erc20_address, amount_in);
    let token = ERC20::new(erc20_address, client.clone());
    let call_build_approve = token.approve(router_address, amount_in);
    let mut tx_approve = call_build_approve.into_transaction_request();
    let _ = utils::send_transaction(
        Arc::new(client.clone()),
        owner,
        tx_approve, 
        chain_id,
    ).await?; 

   println!("swap exchange_router {} {}", exchange_router_address, amount0_in);
   let params_send_tokens = if amount0_in != U256::ZERO {
        ExchangeRouter::sendTokensCall {
            token: base_address,
            receiver: pools[0].bank,
            amount: amount0_in,
        }
   } else {
         ExchangeRouter::sendTokensCall {
            token: meme_address,
            receiver: pools[0].bank,
            amount: amount1_in,
        }   
   };

   let params_swap = SwapUtils::SwapParams{
        token0: base_address,
        token1: meme_address,
        amount0In: amount0_in,
        amount1In: amount1_in,
        amount0Out: amount0_out,
        amount1Out: amount1_out,
        to: owner,
    };

    let params_execute_swap = ExchangeRouter::executeSwapCall{
        params:params_swap,
    };

    let multicallArgs = vec![
        Bytes::from(params_send_tokens.abi_encode()),
        Bytes::from(params_execute_swap.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));
    let call_build = exchange_router.multicall(multicallArgs);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        Arc::new(client.clone()),
        owner,
        tx, 
        chain_id,
    ).await;

    if let Ok(_) = result {
        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    Ok(()) 

}

// pub async fn buy(
//     meme: String,
//     amount: f64,
//     price_limit: f64,
// ) -> Result<(), String> {

//    let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
//    let wallet = EthereumWallet::from(signer.clone());
//    let owner = wallet.default_signer().address();
//    // let chain_id: u64 = 84532;
//    //let chain_id: u64 = 31337;

//    //let rpc = (BASE_SEPOLIA).parse().map_err(|e| e.to_string())?;
//    let rpc = Url::parse(BASE_SEPOLIA).map_err(|e| e.to_string())?;
//    let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);
//    let contracts = contracts::load_contracts("deployments/contracts.json");
//    let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
//    let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
//    let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
//    let router_address = contracts::get_contract_address(&contracts, "Router").unwrap();
//    let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();

//    dbg!(owner, data_store_address, exchange_router_address, reader_address, base_address);

//    let meme_address = Address::from_str(meme.as_str()).unwrap();
//    let reader = Reader::new(reader_address.clone(), client.clone());
//    let pook_key = utils::hash_pool_key(base_address, meme_address);

//    dbg!(meme_address, pook_key);

//    let pools = reader.getPools2(data_store_address,  vec![pook_key]).call().await.unwrap();
//    let pools = pools._0;

//    //let base_decimals = pools[0].assets[1].decimals;
//    //let base_decimals = U256::from(6);//usdt
//    let amount0_in = U256::from((amount * 1000000 as f64) as u64);
//    let amount1_out = if U256::from(price_limit as u64) == U256::ZERO {
//         U256::ZERO
//    } else {
//         U256::from((amount / price_limit) as u64)
//    };


//    println!("aporove usdt {}", amount);
//    let usdt = ERC20::new(base_address, client.clone());
//    //usdt.approve(router_address, amount0_in).await.unwrap;
//    let call_build_approve = usdt.approve(router_address, amount0_in);
//     let mut tx_approve = call_build_approve.into_transaction_request();
//     let _ = utils::send_transaction(
//         Arc::new(client.clone()),
//         owner,
//         tx_approve, 
//         chain_id,
//     ).await?;  

//    println!("swap exchange_router {} {}", exchange_router_address, amount0_in);
//    let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));
//    let params_send_tokens = ExchangeRouter::sendTokensCall {
//         token: base_address,
//         receiver: pools[0].bank,
//         amount: amount0_in,
//     };

//    let params_swap = SwapUtils::SwapParams{
//         token0: base_address,
//         token1: meme_address,
//         amount0In: amount0_in,
//         amount1In: U256::ZERO,
//         amount0Out: U256::ZERO,
//         amount1Out: amount1_out,
//         to: owner,
//     };

//     let params_execute_swap = ExchangeRouter::executeSwapCall{
//         params:params_swap,
//     };

//     let multicallArgs = vec![
//         Bytes::from(params_send_tokens.abi_encode()),
//         Bytes::from(params_execute_swap.abi_encode()),
//     ];

//     let call_build = exchange_router.multicall(multicallArgs);
//     let mut tx = call_build.into_transaction_request();
//     let _ = utils::send_transaction(
//         Arc::new(client.clone()),
//         owner,
//         tx, 
//         chain_id,
//     ).await?;

//     Ok(()) 

// }