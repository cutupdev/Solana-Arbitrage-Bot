//! Lazy-loaded service configurations and clients
//!
//! This module provides lazy initialization of external service clients
//! including Jupiter, Jito, Nozomi, Helius, and other relayer services.

use crate::CONFIG;
use jupiter_swap_api_client::JupiterSwapApiClient;
use once_cell::sync::Lazy;
use solana_relayer_adapter_rust::{
    Astralane, BlockRazor, BloxRoute, Helius, Jito, NextBlock, Nozomi, NozomiRegionsType, ZeroSlot
};
use std::str::FromStr;
use tokio::sync::OnceCell;

/// Available transaction confirmation services
#[derive(Debug, Clone, Copy)]
pub enum ConfirmServices {
    Jito,
    LilJit,
    ZeroSlot,
    Nozomi,
    Helius,
    Rpc,
}

impl FromStr for ConfirmServices {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "JITO" => Ok(ConfirmServices::Jito),
            "LIL_JIT" => Ok(ConfirmServices::LilJit),
            "ZERO_SLOT" => Ok(ConfirmServices::ZeroSlot),
            "HELIUS" => Ok(ConfirmServices::Helius),
            "NOZOMI" => Ok(ConfirmServices::Nozomi),
            "RPC" => Ok(ConfirmServices::Rpc),
            other => Err(format!("Invalid confirm_service value: {}", other)),
        }
    }
}

/// Selected transaction confirmation service type
pub static SUBMIT_TYPE: Lazy<ConfirmServices> = Lazy::new(|| {
    CONFIG
        .services
        .confirm_service
        .parse::<ConfirmServices>()
        .expect("Invalid confirm_service in config")
});

/// Jupiter API endpoint URL
pub static JUPITER_ENDPOINT: Lazy<String> = Lazy::new(|| CONFIG.services.jupiter_endpoint.clone());
/// Jupiter API key (optional)
pub static JUPITER_API_KEY: Lazy<Option<String>> = Lazy::new(|| {
    if CONFIG.services.jupiter_api_key.is_empty() {
        None
    } else {
        Some(CONFIG.services.jupiter_api_key.clone())
    }
});

/// Jupiter swap API client instance
pub static JUPITER_CLIENT: Lazy<JupiterSwapApiClient> =
    Lazy::new(|| JupiterSwapApiClient::new(JUPITER_ENDPOINT.clone(), JUPITER_API_KEY.clone()));

/// Nozomi relayer client (initialized on first use)
pub static NOZOMI_CLIENT: OnceCell<Nozomi> = OnceCell::const_new();

/// ZeroSlot relayer client (initialized on first use)
pub static ZSLOT_CLIENT: OnceCell<ZeroSlot> = OnceCell::const_new();

/// Helius relayer client (initialized on first use)
pub static HELIUS_CLIENT: OnceCell<Helius> = OnceCell::const_new();

/// Jito relayer client (initialized on first use)
pub static JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();

/// Lil Jito relayer client (initialized on first use)
pub static LIL_JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();

/// BlockRazor relayer client (initialized on first use)
pub static BRAZOR_CLIENT: OnceCell<BlockRazor> = OnceCell::const_new();

/// Astralane relayer client (initialized on first use)
pub static ASTRA_CLIENT: OnceCell<Astralane> = OnceCell::const_new();

/// Initialize the Nozomi relayer client
pub async fn init_nozomi() {
    let nozomi = Nozomi::new_with_region(
        NozomiRegionsType::EwrSecure,
        CONFIG.services.nozomi_api_key.clone(),
    )
    .await;
    nozomi.health_check(50);
    NOZOMI_CLIENT.set(nozomi).unwrap();
}

/// Initialize the Jito relayer client
pub async fn init_jito() {
    let jito = Jito::new_auto(Some(CONFIG.services.jito_api_key.clone())).await;
    JITO_CLIENT.set(jito).unwrap();
}

/// Initialize the Helius relayer client
pub async fn init_helius() {
    let helius = Helius::new_auto(CONFIG.credential.laser_token.clone()).await;
    HELIUS_CLIENT.set(helius).unwrap();
}

/// Initialize the Lil Jito relayer client
pub async fn init_lil_jit() {
    let jito = Jito::new_with_liljit(CONFIG.services.liljit_endpoint.clone()).await;
    LIL_JITO_CLIENT.set(jito).unwrap();
}

/// Initialize the ZeroSlot relayer client
pub async fn init_zslot() {
    let zslot = ZeroSlot::new_auto(CONFIG.services.zero_slot_key.clone()).await;
    zslot.health_check(50);
    ZSLOT_CLIENT.set(zslot).unwrap();
}


/// Initialize the Astralane relayer client
pub async fn init_astra() {
    let astralane_key = CONFIG.services.astralane_key.clone();

    let astralane = Astralane::new_auto(astralane_key).await;
    ASTRA_CLIENT.set(astralane).unwrap();
}

/// Initialize the BlockRazor relayer client
pub async fn init_blockrazor() {
    let blockrazor_key = CONFIG.services.blockrazor_key.clone();

    let blockrazor = BlockRazor::new_auto(blockrazor_key).await;
    BRAZOR_CLIENT.set(blockrazor).unwrap();
}
