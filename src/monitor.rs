use anchor_client::{
    solana_client::rpc_client::{RpcClient,GetConfirmedSignaturesForAddress2Config},
    solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature,
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

use crate::tx_parser;

async fn get_newest_slot(rpc_client: &RpcClient) -> Result<u64, Box<dyn Error>> {
    // Fetch the latest slot
    let latest_slot = rpc_client.get_slot()?;
    Ok(latest_slot)
}

async fn get_signatures_by_program_id(
    client: &RpcClient,
    program_id: &Pubkey,
    start_slot: u64,
    end_slot: u64,
) -> Result<Vec<RpcConfirmedTransactionStatusWithSignature>, Box<dyn Error>> {

    let mut all_signatures = Vec::new();
    let mut before = None;

    loop {
        // Fetch signatures for program_id using `get_signatures_for_address_with_config`
        let config = GetConfirmedSignaturesForAddress2Config {
            before,
            until: None,
            limit: Some(100), // Adjust the batch size as needed
            commitment: Some(CommitmentConfig::confirmed()),
        };
        
        let signatures = client
            .get_signatures_for_address_with_config(program_id, config)?;

        // Filter signatures within the slot range
        let filtered_signatures: Vec<RpcConfirmedTransactionStatusWithSignature> = signatures
            .iter()
            .filter(|sig| sig.slot >= start_slot && sig.slot <= end_slot && sig.err.is_none())
            .cloned() // Clone the signature to avoid borrowing issues
            .collect();

        all_signatures.extend(filtered_signatures);

        // If there are more signatures, fetch the next page
        if let Some(last_signature) = signatures.last() {
            // If the last signature's slot is below the start_slot, we break the loop
            if last_signature.slot < start_slot {
                break;
            }
            before = Some(Signature::from_str(&last_signature.signature)?);
        } else {
            break;
        }
    }

    Ok(all_signatures)
}


pub async fn execute(
    log_tx: mpsc::Sender<Vec<tx_parser::Instruction>>,
    cluster: Cluster,
    program_id: Pubkey,
    instr: String,
    mut stop_rx: broadcast::Receiver<()>,  // ✅ Added stop signal receiver
) {
    let solana_devnet_url = "https://solana-devnet.g.alchemy.com/v2/Vlen2KsFpIkGNdoGIQynPL828MV-MqeS".to_string();
    let rpc_client = RpcClient::new(solana_devnet_url);

    let mut start_slot = 360083993;
    //dbg!("solana_url", cluster.url());
    // let sig_str = "xYEGeaXDtSZJnhq1uNdu6CsztdYRL44Aaeq6XcYZPU9jPALH8iTLxjNvUq4V7mkdPij4UbgNBuLTj3Rp3o8GL4e".to_string();
    // let mut last_signature: Option<Signature> = Some(Signature::from_str(sig_str.as_str()).unwrap());
    //let mut last_signature: Option<Signature> ;

    loop {
        let end_slot = get_newest_slot(&rpc_client).await.unwrap();

        tokio::select! {
            // Check if we received a stop signal
            _ = stop_rx.recv() => {
                println!("🛑 Stop signal received. Exiting monitor...");
                break;  // ✅ Exit the loop gracefully
            }

            // Fetch latest transactions
            _ = async {
                println!("🔍 Fetching latest transactions...{} {}", start_slot, end_slot);

                let tx_signatures = match get_signatures_by_program_id(
                    &rpc_client,
                    &program_id,
                    start_slot,
                    end_slot
                ).await {
                    Ok(signatures) => signatures,
                    Err(e) => {
                        println!("Error fetching signatures: {}", e);
                        //sleep(Duration::from_secs(5)).await; // Retry after a short delay
                        return; // Skip this iteration and continue
                    }
                };

                start_slot = end_slot;

                if tx_signatures.is_empty() {
                    println!("⚠️ No new transactions.");
                    sleep(Duration::from_secs(5)).await;
                    return;
                }

                //dbg!("tx_signatures", &tx_signatures);
                //last_signature = tx_signatures.first().map(|tx| Signature::from_str(&tx.signature).unwrap());

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
                    let accounts = tx_parser::get_accounts(&tx_details.transaction);

                    if let Some(meta) = &tx_details.transaction.meta {
                        if let OptionSerializer::Some(log_messages) = &meta.log_messages {
                            //dbg!("instrs", tx_parser::get_instrs(log_messages, &accounts));
                            let instrs= tx_parser::get_instrs(log_messages, &accounts);
                            if instrs.len() > 0 {
                                let _ = log_tx.try_send(instrs);
                                //let _ = log_tx.try_send(format!("{:?}", instrs));
                            }  
                        }

                    }
                }
                
                sleep(Duration::from_secs(10)).await;  // Poll every 5 seconds
            } => {}
        }
    }

    println!("🎉 Monitor stopped successfully.");
}
