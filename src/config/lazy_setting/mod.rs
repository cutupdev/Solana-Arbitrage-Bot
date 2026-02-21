//! Lazy-loaded configuration values
//!
//! This module provides lazy initialization of configuration values
//! from the TOML config file, ensuring values are only loaded when needed.

pub mod lazy_arbitrage;
pub mod lazy_credential;
pub mod lazy_fee;
pub mod lazy_services;
pub mod lazy_telegram;

pub use lazy_arbitrage::*;
pub use lazy_credential::*;
pub use lazy_fee::*;
pub use lazy_services::*;
pub use lazy_telegram::*;
