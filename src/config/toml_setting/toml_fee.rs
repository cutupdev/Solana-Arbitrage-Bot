use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PriorityFeeConfig {
    pub cu: u64,
    pub priority_fee_micro_lamport: u64,
    pub third_party_fee: f64,
}