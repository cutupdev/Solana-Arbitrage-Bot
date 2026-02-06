use once_cell::sync::Lazy;
use solana_sdk::pubkey::Pubkey;

use crate::*;

pub static MOTHER_TOKEN: Lazy<Vec<MotherTokenConfig>> =
    Lazy::new(|| CONFIG.arbitrage.mother_token.clone());

    pub static NONCE_ADDR: Lazy<Pubkey> =
    Lazy::new(|| Pubkey::from_str_const(&CONFIG.arbitrage.nonce_addr));

