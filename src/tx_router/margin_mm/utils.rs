
use std::sync::Arc;
use anyhow::{Context, Result};
use alloy::{
    contract::private::{
        Transport, 
        Provider, 
        Network
    },
    network::TransactionBuilder,
};
use alloy_primitives::{
    FixedBytes,
    Keccak256,
    Address,
    U256,
};

use alloy::sol_types::SolValue;

type Bytes32 = FixedBytes<32>;

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
        .context("Error estimating gas usage: {}")
        .map_err(|e| e.to_string())?;

    let bid_gas_price: u128 = client
        .get_gas_price()
        .await
        .context("Error getting gas price")
        .map_err(|e| e.to_string())?;

    tx.set_gas_price(bid_gas_price);
    tx.set_gas_limit(gas_usage_result);

    client
        .send_transaction(tx) // Changed from `action.tx` to `tx`
        .await
        .map_err(|e| format!("Error sending transaction: {}", e))?
        .get_receipt()
        .await
        .map_err(|e| format!("Error getting receipt: {}", e))?;

    Ok(())
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