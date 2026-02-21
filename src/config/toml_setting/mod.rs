//! TOML configuration parsing and management
//!
//! This module handles parsing of the Config.toml file and provides
//! structured access to all configuration values.

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

pub mod toml_arbitrage;
pub mod toml_credential;
pub mod toml_fee;
pub mod toml_telegram;

pub use toml_arbitrage::*;
pub use toml_credential::*;
pub use toml_fee::*;
pub use toml_telegram::*;

/// Main configuration structure containing all bot settings
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Arbitrage trading configuration
    pub arbitrage: ArbitrageConfig,
    /// Credential and endpoint configuration
    pub credential: CredentialConfig,
    /// Fee and priority fee configuration
    pub fee: PriorityFeeConfig,
    /// External service configuration (Jito, Nozomi, etc.)
    pub services: ServicesConfig,
    /// Telegram notification configuration
    pub telegram: NotificationConfig,
    /// Security and protection settings
    pub protect: ProtectionConfig,
}

/// Global configuration instance loaded from Config.toml
/// This is lazily initialized on first access
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let content = fs::read_to_string("Config.toml")
        .expect("Failed to read Config.toml file");
    toml::from_str(&content)
        .expect("Failed to parse Config.toml - check file format")
});
