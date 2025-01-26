use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Cluster,
};

pub const NUMBER_PER_WORKER: u64 = 100;
pub const ACCOUNTS_TXT: &str = "accounts.txt";

pub fn create_account(numbers: u64) {
    //println!("numbers, {}!", numbers);
    let (tx, rx) = mpsc::channel();
    // let accounts_file = "accounts.txt";


    let workers = numbers/NUMBER_PER_WORKER;
    for id in 0..workers {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            let result = process_task(id, NUMBER_PER_WORKER); // Process the task
            thread_tx.send(result).expect("Failed to send result");
        });        
    }

    drop(tx);

    save_results(rx, ACCOUNTS_TXT);

    //return accounts_file;
}

fn process_task(worker_id: u64, number: u64) -> String {
    // Simulate task processing
    //println!("Worker {} is processing task: {}", worker_id, number);
    let mut accounts:Vec<String> = vec![];
    for i in 0..number{
        let account: Keypair = Keypair::new();
        //accounts.push(account);
        //let public_key_str = account.pubkey().to_string();
        let private_key_str = account.to_base58_string();
        accounts.push(private_key_str);
    }
    format!("Worker {} completed task: {:?}", worker_id, accounts)
}

fn save_results(rx: mpsc::Receiver<String>, output_file: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)
        .expect("Failed to open file");

    // Listen for results and write to file
    for result in rx {
        //println!("Received result: {}", result); // Optional: Print result
        writeln!(file, "{}", result).expect("Failed to write to file");
    }

    println!("All results saved to {}", output_file);
}