use std::sync::Arc;
use std::str::FromStr;
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    sol_types::private::{Address},
    providers::{ProviderBuilder}, 
};
use alloy::sol_types::{
    SolValue,
    SolCall,
    SolEvent,
};
use alloy::transports::http::reqwest::Url;

use alloy_primitives::{
    Bytes,
    FixedBytes,
    U256,
};
use margin_mm::{
    reader::{
        Reader,
        ReaderPoolUtils,
    },
    exchangerouter::{
        ExchangeRouter,
        SwapUtils,
        LiquidityUtils,
    },
    eventemitter::{
        EventEmitter::{Swap, Add, Remove},
    },
    erc20::{
        ERC20,
    },
};

use crate::tx_router::margin_mm::contracts;
use crate::tx_router::margin_mm::utils;
use crate::tx_router::margin_mm::constants;
use crate::utils::keypair;

pub async fn buy(
    meme: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> {
   let amount0_in = amount;
   let amount1_out = if price_limit == U256::ZERO {
        U256::ZERO
   } else {
        utils::mul_pow(amount, U256::from(constants::PRICE_DECIMALS))  / price_limit
   };

   dbg!(amount0_in, amount1_out);

   return swap(meme, amount0_in, U256::ZERO, U256::ZERO, amount1_out).await;
}

pub async fn sell(
    meme: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> {

    let amount1_in = amount;
    let amount0_out = if price_limit == U256::ZERO {
        U256::ZERO
    } else {
        utils::div_pow(amount * price_limit, U256::from(constants::PRICE_DECIMALS))
    }; 

    dbg!(amount1_in, amount0_out);

    return swap(meme, U256::ZERO, amount1_in, amount0_out, U256::ZERO).await;
}

pub async fn add(
    meme: String,
    amount0: U256,
    amount1: U256,
) -> Result<U256, String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.default_signer().address();

   let rpc = Url::parse(constants::BASE_SEPOLIA).map_err(|e| e.to_string())?;
   let client = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("deployments/contracts.json");
   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let router_address = contracts::get_contract_address(&contracts, "Router").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();
   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), client.clone());
   let pook_key = utils::hash_pool_key(base_address, meme_address);
   let pools = reader.getPools2(data_store_address,  vec![pook_key]).call().await.unwrap();
   let pools = pools._0;

   utils::approve(Arc::new(client.clone()), router_address, owner, base_address, amount0).await?;
   utils::approve(Arc::new(client.clone()), router_address, owner, meme_address, amount1).await?;

   let params_send_tokens_base = ExchangeRouter::sendTokensCall {
        token: base_address,
        receiver: pools[0].bank,
        amount: amount0,
    };
   let params_send_tokens_meme = ExchangeRouter::sendTokensCall {
        token: meme_address,
        receiver: pools[0].bank,
        amount: amount1,
    };
   let params_add = LiquidityUtils::AddParams{
        token0: base_address,
        token1: meme_address,
        to: owner,
    };

    let multicall_args = vec![
        Bytes::from(params_send_tokens_base.abi_encode()),
        Bytes::from(params_send_tokens_meme.abi_encode()),
        Bytes::from(params_add.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        Arc::new(client.clone()),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        if logs.len() == 5 {
            let add_log = Add::decode_log(&logs[4], false).unwrap();
            // Return the swap instance
            return Ok(add_log.liquidity);
        }

        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    // If you reach here, it means no swap was found or an error occurred
    Err("No valid swap log found.".to_string())    

}

pub async fn remove(
    meme: String,
    liquidity: U256,
) -> Result<(U256, U256), String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.default_signer().address();

   let rpc = Url::parse(constants::BASE_SEPOLIA).map_err(|e| e.to_string())?;
   let client = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("deployments/contracts.json");
   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let router_address = contracts::get_contract_address(&contracts, "Router").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();
   let meme_address = Address::from_str(meme.as_str()).unwrap();
   // let reader = Reader::new(reader_address.clone(), client.clone());
   // let pook_key = utils::hash_pool_key(base_address, meme_address);
   // let pools = reader.getPools2(data_store_address,  vec![pook_key]).call().await.unwrap();
   // let pools = pools._0;


   let params_remove = LiquidityUtils::RemoveParams{
        token0: base_address,
        token1: meme_address,
        liquidity: liquidity,
        to: owner,
    };

    let multicall_args = vec![
        Bytes::from(params_remove.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        Arc::new(client.clone()),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        if logs.len() == 5 {
            let remove_log = Remove::decode_log(&logs[4], false).unwrap();
            // Return the swap instance
            return Ok((remove_log.baseAmount, remove_log.memeAmount));
        }

        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    // If you reach here, it means no swap was found or an error occurred
    Err("No valid swap log found.".to_string())    

}


pub async fn swap(
    meme: String,
    amount0_in: U256,
    amount1_in: U256,
    amount0_out: U256,
    amount1_out: U256,
) -> Result<(U256, U256), String> {

   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.default_signer().address();

   let rpc = Url::parse(constants::BASE_SEPOLIA).map_err(|e| e.to_string())?;
   let client = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("deployments/contracts.json");
   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let exchange_router_address = contracts::get_contract_address(&contracts, "ExchangeRouter").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let router_address = contracts::get_contract_address(&contracts, "Router").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();
   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), client.clone());
   let pook_key = utils::hash_pool_key(base_address, meme_address);


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
    utils::approve(Arc::new(client.clone()), router_address, owner, erc20_address, amount_in).await?;

    // let token = ERC20::new(erc20_address, client.clone());
    // let call_build_approve = token.approve(router_address, amount_in);
    // let mut tx_approve = call_build_approve.into_transaction_request();
    // let _ = utils::send_transaction(
    //     Arc::new(client.clone()),
    //     owner,
    //     tx_approve, 
    //     constants::CHAIN_ID,
    // ).await?; 

   println!("swap exchange_router {} {}", exchange_router_address, amount_in);
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

    let multicall_args = vec![
        Bytes::from(params_send_tokens.abi_encode()),
        Bytes::from(params_execute_swap.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, Arc::new(client.clone()));
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        Arc::new(client.clone()),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        if logs.len() == 3 {
            //let swap_log = Swap::decode_log(&logs[2], false).unwrap();

            let swap_log:Swap = utils::get_log(&logs).unwrap();
            // Return the swap instance
            return Ok((swap_log.amountIn, swap_log.amountOut));
        }

        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    // If you reach here, it means no swap was found or an error occurred
    Err("No valid swap log found.".to_string())

}

pub async fn get_pool(
    meme: String
) -> Result<ReaderPoolUtils::GetPool, String> {
   let signer: PrivateKeySigner = keypair::load_signer_from_file(".env").expect("Failed to load PrivateKeySigner");
   let wallet = EthereumWallet::from(signer.clone());
   let owner = wallet.default_signer().address();
   let rpc = Url::parse(constants::BASE_SEPOLIA).map_err(|e| e.to_string())?;
   //let client = ProviderBuilder::new().with_cached_nonce_management().wallet(wallet.clone()).on_http(rpc);
   let client = ProviderBuilder::new().wallet(wallet.clone()).on_http(rpc);
   let contracts = contracts::load_contracts("deployments/contracts.json");

   let data_store_address = contracts::get_contract_address(&contracts, "DataStore").unwrap();
   let reader_address = contracts::get_contract_address(&contracts, "Reader").unwrap();
   let base_address = contracts::get_contract_address(&contracts, "USDT").unwrap();
   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), client.clone());
   let pook_key = utils::hash_pool_key(base_address, meme_address);

   let pools = reader.getPools2(data_store_address, vec![pook_key]).call().await.unwrap();
   let pools = pools._0;

   Ok(pools[0].clone())
}


