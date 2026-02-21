//! Lazy-loaded arbitrage configuration
//!
//! This module provides lazy initialization of arbitrage-related
//! configuration values from the TOML config file.

use once_cell::sync::Lazy;
use solana_sdk::pubkey::Pubkey;

use crate::*;

/// List of mother tokens to monitor for arbitrage opportunities
pub static MOTHER_TOKEN: Lazy<Vec<MotherTokenConfig>> =
    Lazy::new(|| CONFIG.arbitrage.mother_token.clone());

/// Nonce account address for faster transaction processing
pub static NONCE_ADDR: Lazy<Pubkey> =
    Lazy::new(|| Pubkey::from_str_const(&CONFIG.arbitrage.nonce_addr));

