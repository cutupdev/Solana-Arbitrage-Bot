use once_cell::sync::Lazy;

use crate::*;

pub static FEES: Lazy<PriorityFeeConfig> = Lazy::new(|| CONFIG.fee.clone());
pub static KAMINO_ACCOUNTS: Lazy<KaminoLendAccount> = Lazy::new(|| KaminoLendAccount::new());
