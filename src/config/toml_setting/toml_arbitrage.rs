//! Arbitrage configuration structures
//!
//! This module defines the configuration structures for arbitrage trading,
//! including mother token settings and nonce account configuration.

use serde::Deserialize;

/// Arbitrage trading configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ArbitrageConfig {
    /// List of mother tokens to monitor for arbitrage opportunities
    pub mother_token: Vec<MotherTokenConfig>,
    /// Nonce account address for faster transaction processing
    pub nonce_addr: String,
}

/// Configuration for a single mother token to monitor
#[derive(Debug, Deserialize, Clone)]
pub struct MotherTokenConfig {
    /// Token mint address
    pub token_addr: String,
    /// Minimum trade size threshold to trigger arbitrage detection
    pub threshold: f64,
    /// Minimum profit amount required to execute arbitrage
    pub min_profit_amount: f64,
    /// Range of input amounts to simulate [min, max]
    pub input_amount_range: [f64; 2],
    /// Number of steps to simulate within the input amount range
    pub input_amount_steps: u64,
}
