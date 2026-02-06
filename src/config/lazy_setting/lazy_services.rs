use crate::CONFIG;
use jupiter_swap_api_client::JupiterSwapApiClient;
use once_cell::sync::Lazy;
use solana_relayer_adapter_rust::{
    Astralane, BlockRazor, BloxRoute, Helius, Jito, NextBlock, Nozomi, NozomiRegionsType, ZeroSlot
};
use std::str::FromStr;
use tokio::sync::OnceCell;

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

pub static SUBMIT_TYPE: Lazy<ConfirmServices> = Lazy::new(|| {
    CONFIG
        .services
        .confirm_service
        .parse::<ConfirmServices>()
        .expect("Invalid confirm_service in config")
});

pub static JUPITER_ENDPOINT: Lazy<String> = Lazy::new(|| CONFIG.services.jupiter_endpoint.clone());
pub static JUPITER_API_KEY: Lazy<Option<String>> = Lazy::new(|| {
    if CONFIG.services.jito_api_key.clone() == "" {
        None
    } else {
        Some(CONFIG.services.jito_api_key.clone())
    }
});

pub static JUPITER_CLIENT: Lazy<JupiterSwapApiClient> =
    Lazy::new(|| JupiterSwapApiClient::new(JUPITER_ENDPOINT.clone(), JUPITER_API_KEY.clone()));

pub static NOZOMI_CLIENT: OnceCell<Nozomi> = OnceCell::const_new();
pub static ZSLOT_CLIENT: OnceCell<ZeroSlot> = OnceCell::const_new();
pub static HELIUS_CLIENT: OnceCell<Helius> = OnceCell::const_new();
pub static JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();
pub static LIL_JITO_CLIENT: OnceCell<Jito> = OnceCell::const_new();
pub static BRAZOR_CLIENT: OnceCell<BlockRazor> = OnceCell::const_new();
pub static ASTRA_CLIENT: OnceCell<Astralane> = OnceCell::const_new();

pub async fn init_nozomi() {
    let nozomi = Nozomi::new_with_region(
        NozomiRegionsType::EwrSecure,
        CONFIG.services.nozomi_api_key.clone(),
    )
    .await;
    nozomi.health_check(50);
    NOZOMI_CLIENT.set(nozomi).unwrap();
}

pub async fn init_jito() {
    let jito = Jito::new_auto(Some(CONFIG.services.jito_api_key.clone())).await;
    JITO_CLIENT.set(jito).unwrap();
}

pub async fn init_helius() {
    let helius = Helius::new_auto(CONFIG.credential.laser_token.clone()).await;
    HELIUS_CLIENT.set(helius).unwrap();
}

pub async fn init_lil_jit() {
    let jito = Jito::new_with_liljit(CONFIG.services.liljit_endpoint.clone()).await;
    LIL_JITO_CLIENT.set(jito).unwrap();
}

pub async fn init_zslot() {
    let zslot = ZeroSlot::new_auto(CONFIG.services.zero_slot_key.clone()).await;
    zslot.health_check(50);
    ZSLOT_CLIENT.set(zslot).unwrap();
}


pub async fn init_astra() {
    let astralane_key = CONFIG.services.astralane_key.clone();

    let astralane = Astralane::new_auto(astralane_key).await;
    ASTRA_CLIENT.set(astralane).unwrap();
}

pub async fn init_blockrazor() {
    let blockrazor_key = CONFIG.services.blockrazor_key.clone();

    let blockrazor = BlockRazor::new_auto(blockrazor_key).await;
    BRAZOR_CLIENT.set(blockrazor).unwrap();
}
