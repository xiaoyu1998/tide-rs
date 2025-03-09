use std::sync::Arc;
use std::str::FromStr;
use alloy::{
    network::{EthereumWallet, Ethereum},
    signers::local::PrivateKeySigner,
    sol_types::private::{Address},
    providers::{Provider, ProviderBuilder}, 
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
use crate::tx_router::types::RouterState;

pub async fn buy<P>(
    state: Arc<RouterState<P>>, 
    meme: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
   let amount0_in = amount;
   let amount1_out = if price_limit == U256::ZERO {
        U256::ZERO
   } else {
        utils::mul_pow(amount, U256::from(constants::PRICE_DECIMALS))  / price_limit
   };

   dbg!(amount0_in, amount1_out);

   return swap(state, meme, amount0_in, U256::ZERO, U256::ZERO, amount1_out).await;
}

pub async fn sell<P>(
    state: Arc<RouterState<P>>, 
    meme: String,
    amount: U256,
    price_limit: U256,
) -> Result<(U256, U256), String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    let amount1_in = amount;
    let amount0_out = if price_limit == U256::ZERO {
        U256::ZERO
    } else {
        utils::div_pow(amount * price_limit, U256::from(constants::PRICE_DECIMALS))
    }; 

    dbg!(amount1_in, amount0_out);

    return swap(state, meme, U256::ZERO, amount1_in, amount0_out, U256::ZERO).await;
}

pub async fn add<P>(
    state: Arc<RouterState<P>>, 
    meme: String,
    liquidity: U256,
) -> Result<(U256,U256,U256), String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{

   let owner = state.owner;
   let data_store_address = state.data_store_address;
   let exchange_router_address = state.exchange_router_address;
   let reader_address = state.reader_address;
   let router_address = state.router_address;
   let base_address = state.base_address;
   let meme_address = Address::from_str(meme.as_str()).unwrap();
   let reader = Reader::new(reader_address.clone(), state.client.clone());

   //approve
   let pairOut = reader.calcTokenPairOut(data_store_address,  base_address, meme_address, liquidity).call().await.unwrap();
   let amount0 = pairOut._0;
   let amount1 = pairOut._1;
   utils::approve(state.client.clone(), router_address, owner, base_address, amount0).await?;
   utils::approve(state.client.clone(), router_address, owner, meme_address, amount1).await?;

   dbg!(amount0, amount1);

   //add
   let pook_key = utils::hash_pool_key(base_address, meme_address);
   let pools = reader.getPools2(data_store_address,  vec![pook_key]).call().await.unwrap();
   let pools = pools._0;
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

   let params_execute_add = ExchangeRouter::executeAddCall{
        params:params_add,
    };

    let multicall_args = vec![
        Bytes::from(params_send_tokens_base.abi_encode()),
        Bytes::from(params_send_tokens_meme.abi_encode()),
        Bytes::from(params_execute_add.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, state.client.clone());
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        state.client.clone(),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        //dbg!(logs.len());
        if logs.len() == 4 {
            let add_log:Add = utils::get_log(&logs).unwrap();
            // Return the swap instance
            return Ok((add_log.liquidity, amount0, amount1));
        }

        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    // If you reach here, it means no swap was found or an error occurred
    Err("No valid swap log found.".to_string())    

}

pub async fn remove<P>(
    state: Arc<RouterState<P>>, 
    meme: String,
    liquidity: U256,
) -> Result<(U256, U256), String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{

   let owner = state.owner;
   let data_store_address = state.data_store_address;
   let exchange_router_address = state.exchange_router_address;
   let reader_address = state.reader_address;
   let router_address = state.router_address;
   let base_address = state.base_address;
   let meme_address = Address::from_str(meme.as_str()).unwrap();

   let params_remove = LiquidityUtils::RemoveParams{
        token0: base_address,
        token1: meme_address,
        liquidity: liquidity,
        to: owner,
    };

   let params_execute_remove = ExchangeRouter::executeRemoveCall{
        params:params_remove,
    };

    let multicall_args = vec![
        Bytes::from(params_execute_remove.abi_encode()),
    ];

    let exchange_router = ExchangeRouter::new(exchange_router_address, state.client.clone());
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        state.client.clone(),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        dbg!(logs.len());
        // if logs.len() == 4 {
            let remove_log:Remove = utils::get_log(&logs).unwrap();
            // Return the swap instance
            return Ok((remove_log.baseAmount, remove_log.memeAmount));
        // }

        println!("Transaction sent successfully!");
    } else if let Err(e) = result {
        eprintln!("Error sending transaction: {}", e);
    }

    // If you reach here, it means no swap was found or an error occurred
    Err("No valid swap log found.".to_string())    

}


pub async fn swap<P>(
    state: Arc<RouterState<P>>, 
    meme: String,
    amount0_in: U256,
    amount1_in: U256,
    amount0_out: U256,
    amount1_out: U256,
) -> Result<(U256, U256), String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
    let owner = state.owner;
    let data_store_address = state.data_store_address;
    let exchange_router_address = state.exchange_router_address;
    let reader_address = state.reader_address;
    let router_address = state.router_address;
    let base_address = state.base_address;
    let meme_address = Address::from_str(meme.as_str()).unwrap();
    let reader = Reader::new(reader_address.clone(), state.client.clone());
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
    utils::approve(state.client.clone(), router_address, owner, erc20_address, amount_in).await?;

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

    let exchange_router = ExchangeRouter::new(exchange_router_address, state.client.clone());
    let call_build = exchange_router.multicall(multicall_args);
    let mut tx = call_build.into_transaction_request();
    let result = utils::send_transaction(
        state.client.clone(),
        owner,
        tx, 
        constants::CHAIN_ID,
    ).await;

    if let Ok(logs) = result {
        if logs.len() == 3 {
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

pub async fn get_pool<P>(
    state: Arc<RouterState<P>>,
    meme: String,
) -> Result<ReaderPoolUtils::GetPool, String> 
where
    P: Provider<Ethereum> + Send + Sync + 'static, // Ensure the correct provider is passed
{
   let data_store_address = state.data_store_address;
   let reader_address = state.reader_address;
   let base_address = state.base_address; 
   let meme_address = Address::from_str(meme.as_str()).unwrap(); 
   let reader = Reader::new(reader_address.clone(), state.client.clone());
   let pook_key = utils::hash_pool_key(base_address, meme_address);

   let pools = reader.getPools2(data_store_address, vec![pook_key]).call().await.unwrap();
   let pools = pools._0;

   Ok(pools[0].clone())
}


