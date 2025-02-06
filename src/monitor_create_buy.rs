use std::thread;
use tokio::time::Duration;
use rand::random;


fn check_create() -> bool {
    // // Simulated status check (replace with actual logic)
    // println!("Checking status...");
    // rand::random::<bool>() // Random true/false for demonstration
}

fn buy() {
    println!("Performing action...");
}

fn parse_logs(msg: &str) -> Option<Vec<String>> {
    serde_json::from_str::<RpcTransactionLogs>(msg)
        .ok()
        .map(|logs| logs.value)
}

async pub fn execute(sol_amount: u64) {

    let rpc = RpcClient::new(Cluster::Devnet.url());
    dbg!(rpc.get_balance(&public_key).unwrap());

    let client: PumpFun<'_> = PumpFun::new(Cluster::Devnet, &payer, None, None);
    dbg!(Cluster::Devnet.url());


    // Create an async channel for logs
    let (log_tx, mut log_rx) = mpsc::channel::<String>(100);
    let (stop_tx, stop_rx) = oneshot::channel::<()>();

    // Store the listener in an Option to allow unsubscription
    let mut listener = Some(program.on(move |log: String| {
        let _ = log_tx.try_send(log);
    })?);

    // Spawn a separate task to process logs
    let handle = tokio::spawn(async move {
        process_logs(log_rx, stop_rx).await;
    });

    // Wait for Ctrl+C signal
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("Ctrl+C detected. Stopping...");

    let _ = stop_tx.send(());
    listener.take(); 
    handle.await?;

}

// Function to process logs in a separate thread
async fn process_logs(mut log_rx: mpsc::Receiver<String>, stop_rx: oneshot::Receiver<()>) {
    tokio::select! {
        // Process logs
        Some(log) = log_rx.recv() => {
            println!("Received Log: {}", log);
        }

        // Stop the thread when receiving stop signal
        _ = stop_rx => {
            println!("Shutting down log listener...");
        }
    }
}