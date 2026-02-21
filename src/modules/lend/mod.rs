//! Lending protocol integration module
//!
//! This module handles interactions with lending protocols like Kamino,
//! enabling flash loans and other lending operations for arbitrage strategies.

pub mod kamino;

pub use kamino::*;
