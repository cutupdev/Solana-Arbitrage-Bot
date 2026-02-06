use std::time::Instant;

use futures::future::join_all;
use jupiter_swap_api_client::quote::QuoteResponse;

use crate::*;

/// Simulates arbitrage opportunities across multiple input amounts to find profitable paths.
pub async fn simulate_amount_in(
    mother_token: String,
    mother_token_decimal: u8,
    mother_token_symbol: String,
    unique_tokens: Vec<String>,
    from_f: f64,
    to_f: f64,
    steps: usize,
    min_profit_amount: f64,
) -> Vec<(u64, u64, QuoteResponse, QuoteResponse, u128)> {
    // Tests different trade sizes in parallel and filters for profitable arbitrage opportunities
    todo!("Implementation removed for NDA compliance")
}
