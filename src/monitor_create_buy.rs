use std::thread;
use tokio::time::Duration;
use rand::random;


fn check_create() -> bool {
    // Simulated status check (replace with actual logic)
    println!("Checking status...");
    rand::random::<bool>() // Random true/false for demonstration
}

fn buy() {
    println!("Performing action...");
}

pub fn execute(sol_amount: u64) {
    loop {
        if check_create() {
            buy();
        }
        thread::sleep(Duration::from_secs(1)); // Wait for 1 second
    }
}

