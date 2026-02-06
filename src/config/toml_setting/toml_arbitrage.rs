use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ArbitrageConfig {
    pub mother_token: Vec<MotherTokenConfig>,
    pub nonce_addr: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MotherTokenConfig {
    pub token_addr: String,
    pub threshold: f64,
    pub min_profit_amount: f64,
    pub input_amount_range: [f64; 2],
    pub input_amount_steps: u64,
}
