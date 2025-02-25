
use std::sync::Arc;
use alloy_contract::private::{Transport, Provider, Network};
use alloy_primitives::Address;

pub const BASE_SEPOLIA : &str = "https://base-sepolia.g.alchemy.com/v2/78EX3W8zQaMXPMs1RPt3nhTDefmKkuEB";


// fn hex_to_pubkey(mint_str: &String) -> Result<Pubkey, String> {
//     // Convert the string to a 32-byte array
//     let mint_bytes = match hex::decode(mint_str.to_string()) {
//         Ok(bytes) if bytes.len() == 32 => bytes,
//         Ok(_) => return Err("Invalid pubkey length".to_string()),
//         Err(err) => return Err(format!("Failed to decode pubkey string: {}", err)),
//     };

//     // Create Pubkey from the 32-byte array
//     let pubkey = Pubkey::new_from_array(mint_bytes.try_into().unwrap());
//     Ok(pubkey)
// }

pub async fn send_transaction<P, N>(
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

    let bid_gas_price: u128 = client
        .get_gas_price()
        .await
        .context("Error getting gas price")?
        .as_u128(); // Assuming gas price is returned as a type that can be converted

    tx.set_gas_price(bid_gas_price);

    client
        .send_transaction(tx) // Changed from `action.tx` to `tx`
        .await
        .map_err(|e| format!("Error sending transaction: {}", e))?
        .get_receipt()
        .await
        .map_err(|e| format!("Error getting receipt: {}", e))?;

    Ok(())
}