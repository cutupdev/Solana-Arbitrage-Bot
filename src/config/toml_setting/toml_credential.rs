//! Credential and service configuration structures
//!
//! This module defines the configuration structures for credentials,
//! RPC endpoints, and external service API keys.

use serde::Deserialize;

/// Credential and endpoint configuration
#[derive(Debug, Deserialize, Clone)]
pub struct CredentialConfig {
    /// Path to the wallet keypair file
    pub wallet_path: String,
    /// RPC endpoint URL for Solana network access
    pub rpc_endpoint: String,
    /// Endpoint URL for transaction submission
    pub submit_endpoint: String,
    /// Helius Laserstream endpoint URL
    pub laser_endpoint: String,
    /// Helius Laserstream API token
    pub laser_token: String,
}

/// External service configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ServicesConfig {
    /// Jito API key
    pub jito_api_key: String,
    /// Lil Jito endpoint URL
    pub liljit_endpoint: String,
    /// Transaction confirmation service to use (JITO, NOZOMI, RPC, etc.)
    pub confirm_service: String,
    /// Nozomi API key
    pub nozomi_api_key: String,
    /// ZeroSlot API key
    pub zero_slot_key: String,
    /// Jupiter aggregator API endpoint URL
    pub jupiter_endpoint: String,
    /// BloxRoute API key
    pub bloxroute_key: String,
    /// Astralane API key
    pub astralane_key: String,
    /// BlockRazor API key
    pub blockrazor_key: String,
    /// NextBlock API key
    pub nextblock_key: String,
    /// Jupiter API key (optional)
    pub jupiter_api_key: String,
}
