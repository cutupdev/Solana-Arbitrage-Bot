//! Constants module for Solana Arbitrage Bot
//!
//! This module contains all constant values used throughout the application,
//! including program addresses, token information, and transaction fees.

pub mod alt;
pub mod keys;

pub use alt::*;
pub use keys::*;

/// Base transaction fee in lamports (5000 lamports = 0.000005 SOL)
pub const TRANSACTION_FEE: u64 = 5000;