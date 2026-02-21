//! Fee configuration structures
//!
//! This module defines the configuration structures for transaction fees,
//! including compute units and priority fees.

use serde::Deserialize;

/// Priority fee and compute unit configuration
#[derive(Debug, Deserialize, Clone)]
pub struct PriorityFeeConfig {
    /// Compute units (CU) to request for transactions
    pub cu: u64,
    /// Priority fee in micro-lamports (1 micro-lamport = 0.000001 lamports)
    pub priority_fee_micro_lamport: u64,
    /// Third-party fee percentage (e.g., 0.0001 = 0.01%)
    pub third_party_fee: f64,
}