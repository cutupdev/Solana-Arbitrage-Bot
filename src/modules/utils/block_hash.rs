use solana_sdk::{commitment_config::CommitmentConfig, hash::Hash};
use tokio::time::{Duration, sleep};

use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::RPC_CLIENT;

static GLOBAL_CURRENT_HASH: Lazy<Mutex<Hash>> = Lazy::new(|| Mutex::new(Hash::default()));

fn set_slot(value: Hash) {
    let mut slot = GLOBAL_CURRENT_HASH.lock().unwrap();
    *slot = value;
}

pub fn get_slot() -> Hash {
    let slot = GLOBAL_CURRENT_HASH.lock().unwrap();
    *slot
}

/// Continuously retries fetching the latest blockhash until successful.
pub async fn recent_blockhash_handler() {
    loop {
        match RPC_CLIENT
            .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
            .await
        {
            Ok((latest_blockhash, _)) => {
                set_slot(latest_blockhash);
                break; // Exit the loop once we get a valid blockhash
            }
            Err(e) => {
                println!(
                    "[BLOCKHASH ERROR]\n\t* ERR MSG : {}\n\t* Retrying in 200ms...",
                    e
                );
                sleep(Duration::from_millis(200)).await;
            }
        }
    }

    sleep(Duration::from_millis(500)).await;
}
