//! Lazy-loaded fee configuration and lending accounts
//!
//! This module provides lazy initialization of fee settings and
//! Kamino lending account structures.

use once_cell::sync::Lazy;

use crate::*;

/// Priority fee and compute unit configuration
pub static FEES: Lazy<PriorityFeeConfig> = Lazy::new(|| CONFIG.fee.clone());

/// Kamino lending account instance for flash loan operations
pub static KAMINO_ACCOUNTS: Lazy<KaminoLendAccount> = Lazy::new(|| KaminoLendAccount::new());
