//! Solana Arbitrage Bot Library
//!
//! A high-performance arbitrage trading bot for the Solana blockchain
//! that monitors large trades in real-time and executes profitable
//! arbitrage opportunities using Jupiter aggregator.

pub mod config;
pub mod constant;
pub mod modules;

// Re-export commonly used items for convenience
pub use config::*;
pub use constant::*;
pub use modules::*;
