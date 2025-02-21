use std::thread;
use tokio::time::Duration;
use tokio::sync::{mpsc, broadcast};
use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Cluster,
};
use pumpfun::{
    // accounts::BondingCurveAccount, 
    // utils::CreateTokenMetadata, 
    // PriorityFee, 
    // PumpFun,
    // error::ClientError,
    cpi
};
use rand::random;

use crate::strategies::monitor;
use crate::utils::tx_parser;
use crate::tx_router::client_apis::buy;

pub async fn execute(
    network: String,
    contract: String,
    amount: u64
) {

    // Create an async channel for logs
    let (instr_tx, instr_rx) = mpsc::channel::<Vec<tx_parser::Instruction>>(100);
    //let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let (stop_tx, stop_rx) = broadcast::channel::<()>(1);


    let monitor_stop_rx = stop_rx.resubscribe();
    let instr_stop_rx = stop_rx.resubscribe();

    // Spawn monitor thread
    let monitor_handle = tokio::spawn(monitor::execute(
        instr_tx.clone(),
        // Cluster::Devnet,
        cpi::ID,
        "Create".to_string(),
        monitor_stop_rx,
    ));

    // Spawn a separate task to process instrs
    let instr_handle = tokio::spawn(async move {
        process(network.clone(), contract.clone(), amount, instr_rx, instr_stop_rx).await;
    });

    // // Wait for Ctrl+C signal
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C detected. Stopping...");

    let _ = stop_tx.send(());
    monitor_handle.await.unwrap(); 
    instr_handle.await.unwrap();

}

// Function to process in a separate thread
async fn process(
    network: String,
    contract: String,
    amount: u64, 
    mut instr_rx: mpsc::Receiver<Vec<tx_parser::Instruction>>, 
    mut stop_rx: broadcast::Receiver<()>
) {
    tokio::select! {
        // Process instrs
        Some(instrs) = instr_rx.recv() => {
            dbg!("instrs", &instrs);
            //println!("Received Log: {}", instr);
            //buy
            for instr in &instrs {
                if instr.action == "create_buy".to_string() {
                    // Call the buy function and handle errors
                    match buy(
                        network.clone(),
                        contract.clone(),
                        instr.params.clone(),
                        amount
                    ).await {
                        Ok(_) => {
                            println!("Buy successful for action: {}", instr.action);
                        }
                        Err(e) => {
                            // Print the error if the buy fails
                            println!("Error occurred during buy: {}", e);
                        }
                    }
                }

            }
        }

        // Stop the thread when receiving stop signal
        _ = stop_rx.recv() => {
            println!("Shutting down log listener...");
        }
    }
}