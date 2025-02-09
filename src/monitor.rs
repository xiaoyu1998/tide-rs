use anchor_client::{
    solana_client::rpc_client::{RpcClient,GetConfirmedSignaturesForAddress2Config},
    solana_client::rpc_config::RpcTransactionConfig,
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature},
};
use solana_transaction_status::{
    option_serializer::OptionSerializer,
    UiMessage, 
    UiInstruction, 
    UiParsedMessage, 
    EncodedTransaction, 
    UiParsedInstruction, 
    UiTransactionEncoding
};
use tokio::sync::{mpsc, broadcast};
use tokio::time::{sleep, Duration};
use anchor_client::Cluster;
use std::str::FromStr;
use std::error::Error;

use crate::utils;

// async fn get_newest_slot(rpc_client: &RpcClient) -> Result<u64, Box<dyn Error>> {
//     // Fetch the latest slot
//     let latest_slot = rpc_client.get_slot()?;
//     Ok(latest_slot)
// }

// async fn get_signatures_for_program(
//     rpc_client: &RpcClient,
//     program_id: &Pubkey,
//     start_slot: u64,
//     end_slot: u64,
// ) -> Result<Vec<Signature>, Box<dyn Error>> {
//     let mut all_signatures = Vec::new();
//     let mut current_slot = start_slot;

//     while current_slot <= end_slot {
//         // Fetch confirmed signatures for the given program ID and slot range
//         match rpc_client.get_confirmed_signatures_for_address2(
//             program_id,
//             Some(current_slot),
//             Some(end_slot), // Fetch signatures from current_slot to end_slot
//             None, // Limit to 1000 signatures (default)
//         ) {
//             Ok(signatures) => {
//                 all_signatures.extend(signatures);

//                 // If fewer than the expected number of results, we have reached the end
//                 if signatures.len() < 1000 {
//                     break;
//                 }

//                 // Set the next slot to fetch
//                 current_slot = signatures.last().unwrap().slot;
//             }
//             Err(e) => {
//                 // Return the error if the RPC call fails
//                 return Err(Box::new(e));
//             }
//         }
//     }

//     Ok(all_signatures)
// }

pub async fn execute(
    log_tx: mpsc::Sender<String>,
    cluster: Cluster,
    program_id: Pubkey,
    instr: String,
    mut stop_rx: broadcast::Receiver<()>,  // ✅ Added stop signal receiver
) {
    let solana_devnet_url = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS".to_string();
    let rpc_client = RpcClient::new(solana_devnet_url);

    //let mut start_slot = 359663424;
    //dbg!("solana_url", cluster.url());

    let sig_str = "2BDahqBc5GR48gfNGiYGDq68wEsEtyLhtg7j2GvcBN1tJGnqeerDmXcZjSpMKqTZmhSYVBiP1ZGVBnMkaE2ten6z".to_string();
    let mut last_signature: Option<Signature> = Some(Signature::from_str(sig_str.as_str()).unwrap());

    loop {
        //let mut end_slot = get_newest_slot(&rpc_client).await.unwrap();

        tokio::select! {
            // Check if we received a stop signal
            _ = stop_rx.recv() => {
                println!("🛑 Stop signal received. Exiting monitor...");
                break;  // ✅ Exit the loop gracefully
            }

            // Fetch latest transactions
            _ = async {
                println!("🔍 Fetching latest transactions...");

                // let tx_signatures = match get_signatures_for_program(
                //     &rpc_client,
                //     &program_id,
                //     start_slot,
                //     end_slot
                // ).await {
                //     Ok(signatures) => signatures,
                //     Err(e) => {
                //         println!("Error fetching signatures: {}", e);
                //         sleep(Duration::from_secs(5)).await; // Retry after a short delay
                //         return; // Skip this iteration and continue
                //     }
                // };

                // Fetch the signatures from the program ID
                let tx_signatures = rpc_client
                    .get_signatures_for_address_with_config(
                        &program_id,
                        GetConfirmedSignaturesForAddress2Config {
                            before: last_signature.clone(), // Get newer signatures
                            until: None,
                            limit: Some(2), // Adjust the limit based on your needs
                            commitment: Some(CommitmentConfig::confirmed()),
                        },
                    )
                    .expect("Failed to fetch transaction signatures");

                //start_slot = end_slot;

                if tx_signatures.is_empty() {
                    println!("⚠️ No new transactions.");
                    sleep(Duration::from_secs(5)).await;
                    return;
                }

                dbg!("tx_signatures", &tx_signatures);
                last_signature = tx_signatures.first().map(|tx| Signature::from_str(&tx.signature).unwrap());

                for tx_info in tx_signatures {
                    let signature = Signature::from_str(&tx_info.signature).unwrap();
                    println!("💡 Processing TX: {}", signature);

                    let config = RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::Json),
                        max_supported_transaction_version: Some(0),  // Add this to support legacy transactions
                        ..Default::default()
                    };

                    let tx_details = rpc_client
                        .get_transaction_with_config(
                            &signature,
                            config,
                        )
                        .expect("Failed to fetch transaction");

                    //dbg!("tx_details", &tx_details);

                    if let Some(meta) = &tx_details.transaction.meta {
                        if let OptionSerializer::Some(log_messages) = &meta.log_messages {
                            dbg!("instrs", utils::get_instrs(log_messages));
                        }

                    }
                }
                
                sleep(Duration::from_secs(10)).await;  // Poll every 5 seconds
            } => {}
        }
    }

    println!("🎉 Monitor stopped successfully.");
}