use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

pub mod toml_arbitrage;
pub mod toml_credendial;
pub mod toml_fee;
pub mod toml_telegram;
pub use toml_arbitrage::*;
pub use toml_credendial::*;
pub use toml_fee::*;
pub use toml_telegram::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub arbitrage: ArbitrageConfig,
    pub credential: CredentialConfig,
    pub fee: PriorityFeeConfig,
    pub services: ServicesConfig,
    pub telegram: NotificationConfig,
    pub protect: ProtectionConfig,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let content = fs::read_to_string("Config.toml").expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
});
