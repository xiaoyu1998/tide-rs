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
    accounts::BondingCurveAccount, 
    utils::CreateTokenMetadata, 
    PriorityFee, 
    PumpFun,
    error::ClientError,
    cpi
};
use rand::random;

use crate::monitor;

fn buy() {
    println!("Performing action...");
}

pub async fn execute(sol_amount: u64) {

    // Create an async channel for logs
    let (instr_tx, instr_rx) = mpsc::channel::<String>(100);
    //let (stop_tx, stop_rx) = oneshot::channel::<()>();
    let (stop_tx, stop_rx) = broadcast::channel::<()>(1);


    let monitor_stop_rx = stop_rx.resubscribe();
    let instr_stop_rx = stop_rx.resubscribe();

    // Spawn monitor thread
    let monitor_handle = tokio::spawn(monitor::execute(
        instr_tx.clone(),
        Cluster::Devnet,
        cpi::ID,
        "Create".to_string(),
        monitor_stop_rx,
    ));

    // Spawn a separate task to process instrs
    let instr_handle = tokio::spawn(async move {
        process_instr(instr_rx, instr_stop_rx).await;
    });

    // // Wait for Ctrl+C signal
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C detected. Stopping...");

    let _ = stop_tx.send(());
    monitor_handle.await.unwrap(); 
    instr_handle.await.unwrap();

}

// Function to process instr in a separate thread
async fn process_instr(mut instr_rx: mpsc::Receiver<String>, mut stop_rx: broadcast::Receiver<()>) {
    // let payer: Keypair = Keypair::new();
    // let public_key = payer.pubkey();
    // println!("Loaded Solana Address: {}", public_key);

    // let rpc = RpcClient::new(Cluster::Devnet.url());
    // dbg!(rpc.get_balance(&public_key).unwrap());

    // let client: PumpFun<'_> = PumpFun::new(Cluster::Devnet, &payer, None, None);
    // dbg!(Cluster::Devnet.url());

    // // Request airdrop of 2 SOL
    // let amount = 2_000_000_000; // 2 SOL (in lamports)
    // match client.client.request_airdrop(&public_key, amount) {
    //     Ok(sig) => println!("Airdrop requested. Transaction Signature: {}", sig),
    //     Err(err) => eprintln!("Airdrop failed: {:?}", err),
    // }
    tokio::select! {
        // Process instrs
        Some(instr) = instr_rx.recv() => {
            println!("Received Log: {}", instr);
            //buy
        }

        // Stop the thread when receiving stop signal
        _ = stop_rx.recv() => {
            println!("Shutting down log listener...");
        }
    }
}