//! Configuration module for Solana Arbitrage Bot
//!
//! This module handles all configuration management, including:
//! - TOML configuration file parsing
//! - Lazy-loaded configuration values
//! - Service initialization

pub mod lazy_setting;
pub mod toml_setting;

pub use lazy_setting::*;
pub use toml_setting::*;
