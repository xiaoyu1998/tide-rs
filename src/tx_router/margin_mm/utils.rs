
use std::sync::Arc;
use anyhow::{Context, Result};
use alloy::{
    transports::Transport, 
    providers::{
        Provider,
    }, 
    network::{
        TransactionBuilder,
        Network,
        //ReceiptResponse, 
        Ethereum
    },
};
use alloy_primitives::{
    Log,
    LogData,
    FixedBytes,
    Keccak256,
    Address,
    U256,
};

use alloy::rpc::types::{
    TransactionReceipt,
    // Log,
};

use alloy::consensus::{
    ReceiptWithBloom,
    ReceiptEnvelope,
    Receipt,
    // TxReceipt
};

use alloy::sol_types::{
    SolValue,
    SolEvent,
};

use margin_mm::{
    eventemitter::{
        EventEmitter::Swap,
    },
};

type Bytes32 = FixedBytes<32>;
pub const PRICE_DECIMALS: u32 = 27;


pub async fn send_transaction<T, P, N>(
    client: Arc<P>,
    account: Address,
    mut tx: <N as Network>::TransactionRequest,
    chain_id: u64,
) -> Result<(), String>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    tx.set_chain_id(chain_id);
    tx.set_from(account);

    let gas_usage_result = client
        .estimate_gas(&tx)
        .await
        // .context("Error estimating gas usage: {}")
        // .map_err(|e| e.to_string())?;
        .map_err(|e| format!("Error estimating gas usage: {}", e))?;

    let bid_gas_price: u128 = client
        .get_gas_price()
        .await
        // .context("Error getting gas price")
        // .map_err(|e| e.to_string())?;
        .map_err(|e| format!("Error getting gas price: {}", e))?;

    tx.set_gas_price(bid_gas_price);
    tx.set_gas_limit(gas_usage_result);

    let receipt = client
        .send_transaction(tx) // Changed from `action.tx` to `tx`
        .await
        .map_err(|e| format!("Error sending transaction: {}", e))?
        .get_receipt()
        .await
        .map_err(|e| format!("Error getting receipt: {}", e))?;

    // // Force cast (unsafe if not sure of the type)
    // let tx_receipt: &TransactionReceipt<ReceiptEnvelope<Log>> = unsafe {
    //     &*(&receipt as *const _ as *const TransactionReceipt<ReceiptEnvelope<Log>>)
    // };

    // dbg!(tx_receipt);
    // let receipt: &TransactionReceipt<ReceiptEnvelope<Log>> = &receipt;
    // let logs = get_logs(receipt);
    // //dbg!(logs);
    // println!("Receipt type: {}", std::any::type_name_of_val(&receipt));
    // // let swap_log = Swap::decode_log(&logs[2], false).unwrap();
    // // println!("swap_log account: {}", swap_log.account);
    // // println!("swap_log amountIn: {}", swap_log.amountIn);
    // // println!("swap_log amountOut: {}", swap_log.amountOut);
    // if let Some(tx_receipt) = receipt.as_ref() {
    //     let logs = get_logs(tx_receipt);
    // } else {
    //     eprintln!("Receipt is None");
    // }


    Ok(())
}

pub fn get_logs(receipt: &TransactionReceipt<ReceiptEnvelope<Log>>) -> Vec<Log> {
    // Match the ReceiptEnvelope variant to access the logs
    match &receipt.inner {
        ReceiptEnvelope::Legacy(ref receipt_with_bloom) 
        | ReceiptEnvelope::Eip2930(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip1559(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip4844(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip7702(ref receipt_with_bloom) => {

            println!("xiaoyu1998 {}", receipt_with_bloom.receipt.logs[0].address);
            println!("xiaoyu1998 {}", receipt_with_bloom.receipt.logs[0].data.data);
            // Return the logs from the Receipt
            //let logs = receipt_with_bloom.receipt.logs.clone();
            // println!("xiaoyu1998 {}", receipt_with_bloom.receipt.logs[0].address);

            // logs
            receipt_with_bloom.receipt.logs.clone()


        },
        // Handle any unknown or future variants with a wildcard
        _ => {
            // You can return an empty Vec<Log> or handle it differently if needed
            eprintln!("Unexpected ReceiptEnvelope variant");
            Vec::new() // Returning an empty vector in case of unexpected variant
        }
    }
}

pub fn hash_data(data: Vec<Vec<u8>>) -> Vec<u8> {
    // Concatenate all byte arrays in the data
    let mut result = Vec::new();
    for item in data {
        result.extend(item);
    }
    result
}

pub fn keccak256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn hash_pool_key(token0: Address, token1: Address) -> Bytes32 {
    let pool_bytes = "POOL".to_string().abi_encode();
    let pool_hash = keccak256_hash(&pool_bytes);

    // Ensure the tokens are in lexicographical order (lowercase comparison)
    let (token0, token1) = if token0 < token1 {
        (token0, token1)
    } else {
        (token1, token0)
    };

    let token0_bytes = token0.abi_encode();
    let token1_bytes = token1.abi_encode();

    // Concatenate POOL (bytes32), token0, and token1 (addresses)
    let encoded_data = vec![pool_hash, token0_bytes, token1_bytes];

    // // Return the hash of the concatenated data
    let concatenated = hash_data(encoded_data);
    
    // Perform Keccak256 hashing on the concatenated bytes
    let mut hasher = Keccak256::new();
    hasher.update(concatenated);
    let hash_result = hasher.finalize();

    FixedBytes::try_from(hash_result.as_slice()).expect("Hash should be 32 bytes")
}

fn hash_position_key(account: Address, position_id: U256) -> Bytes32 {

    let bytes1: [u8; 20] = account.into();  // Convert Address to a 20-byte array
    let bytes2: [u8; 32] = position_id.to_be_bytes();  // Convert Address to a 20-byte array

    // Create a Keccak256 hasher
    let mut hasher = Keccak256::new();

    // Concatenate the addresses in the determined order
    hasher.update(&bytes1);
    hasher.update(&bytes2);

    // Finalize the hash and convert to Bytes32
    let hash_result = hasher.finalize();

    // Convert the result to Bytes32
    FixedBytes::try_from(hash_result.as_slice()).expect("Hash should be 32 bytes")

}

pub fn mul_pow_2_half(value: f64, decimal: u32) -> U256{
    let first_half = decimal / 2;
    let rest_half = decimal - first_half;
    U256::from(value * (10u64.pow(first_half)) as f64) * U256::from(10u64.pow(rest_half))
}

pub fn mul_pow(price: U256, decimals: U256) -> U256{
    price*U256::from(10).pow(decimals)
}

pub fn div_pow(price: U256, decimals: U256) -> U256{
    price/U256::from(10).pow(decimals)
}

pub fn get_base_units_from_tokens(amount: U256, price: U256, decimal_delta: U256) -> U256{
    amount*price/U256::from(10).pow(decimal_delta)/U256::from(10).pow(U256::from(PRICE_DECIMALS))
}

