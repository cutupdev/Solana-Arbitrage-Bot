use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CredentialConfig {
    pub wallet_path: String,
    pub rpc_endpoint: String,
    pub submit_endpoint: String,
    pub laser_endpoint: String,
    pub laser_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServicesConfig {
    pub jito_api_key: String,
    pub liljit_endpoint: String,
    pub confirm_service: String,
    pub nozomi_api_key: String,
    pub zero_slot_key: String,
    pub jupiter_endpoint: String,
    pub bloxroute_key: String,
    pub astralane_key: String,
    pub blockrazor_key: String,
    pub nextblock_key: String,
    pub jupiter_api_key: String,
}
