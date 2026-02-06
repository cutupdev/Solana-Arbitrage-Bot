use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationConfig {
    pub telegram_bot_key: String,
    pub group_channel_id: isize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProtectionConfig {
    pub protect_key: bool,
}