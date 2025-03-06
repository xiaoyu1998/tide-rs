
use std::sync::Arc;
use std::error::Error;
use anyhow::{Context, Result};
use alloy::{
    transports::Transport, 
    providers::{
        Provider,
    }, 
    network::{
        TransactionBuilder,
        Network,
        ReceiptResponse, 
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
    //Log,
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
    erc20::{
        ERC20,
    },
};
use crate::tx_router::margin_mm::constants;

type Bytes32 = FixedBytes<32>;

pub async fn approve<P>(
    client: Arc<P>,
    router_address: Address,
    owner: Address,
    token: Address,
    amount: U256,
) -> Result<(), String>
where
    P: Provider<Ethereum>,
{
    let token = ERC20::new(token, client.clone());
    let call_build_approve = token.approve(router_address, amount);
    let mut tx_approve = call_build_approve.into_transaction_request();
    let _ = send_transaction(
        Arc::new(client.clone()),
        owner,
        tx_approve, 
        constants::CHAIN_ID,
    ).await?; 

    Ok(())
}

pub async fn send_transaction<P>(
    client: Arc<P>,
    account: Address,
    mut tx: <Ethereum as Network>::TransactionRequest,
    chain_id: u64,
) -> Result<Vec<Log>, String>
where
    P: Provider<Ethereum>,
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

    //println!("Receipt type: {}", std::any::type_name_of_val(&receipt));
    let receipt = receipt.into_primitives_receipt();
    let logs = get_logs(&receipt);

    Ok(logs)
}

pub fn get_log<T>(
    logs: &Vec<Log>,
    // target_address: Option<Address>,
) -> Result<T, String>
where
    T: SolEvent,
{
    // Loop through each log to find the matching one
    for log in logs {
        // // Check if the log matches the provided address (if given)
        // if let Some(address) = target_address {
        //     if log.address != address {
        //         continue; // Skip logs from different addresses
        //     }
        // }

        // Check if the log's first topic matches the event signature hash of the given type `T`
        if log.data.topics()[0] != T::SIGNATURE_HASH {
            continue; // Skip logs that don't match the event signature
        }

        // Attempt to decode the log into the target type (T)
        let decoded_log: T = T::decode_log(log, false).unwrap().data; // Use false for non-indexed parameters
        return Ok(decoded_log); // Return the decoded log
    }

    // Return None if no matching log is found
    Err("No valid log found.".to_string())
}


pub fn get_logs(receipt: &TransactionReceipt<ReceiptEnvelope<Log>>) -> Vec<Log> {
    // Match the ReceiptEnvelope variant to access the logs
    match &receipt.inner {
        ReceiptEnvelope::Legacy(ref receipt_with_bloom) 
        | ReceiptEnvelope::Eip2930(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip1559(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip4844(ref receipt_with_bloom)
        | ReceiptEnvelope::Eip7702(ref receipt_with_bloom) => {

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
    amount*price/U256::from(10).pow(decimal_delta)/U256::from(10).pow(U256::from(constants::PRICE_DECIMALS))
}


pub fn format_to_f32(amount: U256, decimals: U256) -> f32 {
    // Calculate 10^decimals as a U256
    let divisor = U256::from(10).pow(decimals);

    // Perform the division to get the integer part
    let integer_part = amount / divisor;
    let integer_part_f32: f32 = integer_part.try_into().unwrap();

    // Perform the modulus to get the fractional part
    let remainder = amount % divisor;

    // Convert the remainder into a fraction and divide by the divisor again
    let fractional_part = remainder * U256::from(10).pow(U256::from(6)) / divisor; // Here we multiply by 10^6 to get more precision
    let fractional_part_f32 :f32 = fractional_part.try_into().unwrap();

    // Combine integer and fractional parts and return as f32
    integer_part_f32 + fractional_part_f32 / 1_000_000.0
}

pub fn calc_price(base_amount: U256, base_decimals: U256, meme_amount: U256, meme_decimals: U256) -> f32 {
    // Calculate scaling factors for base and meme amounts
    let base_scale = U256::from(10).pow(base_decimals);
    let meme_scale = U256::from(10).pow(meme_decimals);

    // Scale both amounts first
    let scaled_base_amount = base_amount * meme_scale;
    let scaled_meme_amount = meme_amount * base_scale;

    // Perform the division to get the integer part
    let integer_part = scaled_base_amount / scaled_meme_amount;
    let integer_part_f32: f32 = integer_part.try_into().unwrap();

    // Calculate the remainder for the fractional part
    let remainder = scaled_base_amount % scaled_meme_amount;

    // Scale the remainder to get more precision in the fractional part
    let fractional_part = remainder * U256::from(10).pow(U256::from(6)) / scaled_meme_amount; // Using 10^6 for 6 decimal precision
    let fractional_part_f32 :f32 = fractional_part.try_into().unwrap();
    
    // Combine integer and fractional parts and return as f32
    integer_part_f32 + fractional_part_f32 / 1_000_000.0
}




