use solana_rpc_client_nonce_utils::data_from_account;
use solana_sdk::{commitment_config::CommitmentConfig, nonce::state::Data as NonceData};
use tokio::time::{Duration, sleep};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::{NONCE_ADDR, RPC_CLIENT};

pub static GLOBAL_CURRENT_NONCE: Lazy<Mutex<NonceData>> =
    Lazy::new(|| Mutex::new(NonceData::default()));

pub fn set_nonce(new_nonce: NonceData) {
    let mut nonce = GLOBAL_CURRENT_NONCE.lock().unwrap();
    *nonce = new_nonce;
}

pub fn get_nonce() -> NonceData {
    let nonce = GLOBAL_CURRENT_NONCE.lock().unwrap();
    nonce.clone()
}

/// Continuously fetches nonce account data to maintain fresh blockhash for fast transactions.
pub async fn fetch_nonce() {
    // Polls RPC for nonce account updates and stores the latest nonce data in global state
    todo!("Implementation removed for NDA compliance")
}
