//! Telegram notification and protection configuration
//!
//! This module defines the configuration structures for Telegram notifications
//! and security protection settings.

use serde::{Deserialize, Serialize};

/// Telegram notification configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    /// Telegram bot API token
    pub telegram_bot_key: String,
    /// Telegram group/channel ID for notifications
    pub group_channel_id: isize,
}

/// Security and protection configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtectionConfig {
    /// Whether to enable key protection/encryption
    pub protect_key: bool,
}