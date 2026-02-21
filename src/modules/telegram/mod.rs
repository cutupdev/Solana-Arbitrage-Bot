//! Telegram notification module
//!
//! This module handles sending notifications to Telegram channels
//! for bot events, trade executions, and status updates.

use reqwest::Client;
use serde::Serialize;

use crate::*;

#[derive(Serialize)]
struct SendMessageRequest {
    chat_id: String,
    text: String,
    parse_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

/// Send a message to Telegram via bot API
///
/// # Arguments
/// * `bot_token` - Telegram bot token
/// * `chat_id` - Target chat/channel ID
/// * `text` - Message text (supports HTML formatting)
pub async fn send_telegram_message(
    bot_token: String,
    chat_id: String,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let payload = SendMessageRequest {
        chat_id,
        text,
        parse_mode: "HTML".to_string(),
        disable_web_page_preview: Some(true), // disable preview
    };

    let client = Client::new();
    let resp = client.post(&url).json(&payload).send().await?;

    if resp.status().is_success() {
    } else {
        let _ = resp.text().await?;
    }

    Ok(())
}

/// Format and send a message with bot metadata to the configured chat
///
/// The message includes username, local IP, and wallet address information.
pub async fn send_messages(text: String) {
    let msg = format!(
        "{}\n{}@{}  <a href=\"https://solscan.io/account/{}\">{}</a>",
        text,
        USERNAME.clone(),
        LOCAL_IP.clone(),
        PUBKEY.to_string(),
        abbreviate_address(PUBKEY.to_string())
    );

    // Single chat, just await the send
    if let Err(_e) = send_telegram_message(
        TELEGRAM_BOT_KEY.clone(),
        TG_GROUP_CHANNEL.to_string(), // single chat ID
        msg.clone(),
    )
    .await
    {};
}

/// Send a formatted message with a title and clickable link
///
/// # Arguments
/// * `title` - Message title/header
/// * `text` - Link text and URL
pub async fn tg_msg(title: &str, text: String) {
    let log = format!(
        "{}\n\n<a href=\"{}\" target=\"_blank\">{}</a>",
        title, text, text
    );
    send_messages(log).await;
}

/// Abbreviate a Solana address for display (first 4 + last 4 characters)
///
/// # Arguments
/// * `address` - Full Solana address string
///
/// # Returns
/// Abbreviated address in format "xxxx..yyyy"
pub fn abbreviate_address(address: String) -> String {
    if address.len() <= 8 {
        return address;
    }
    let start = &address[..4];
    let end = &address[address.len() - 4..];
    format!("{}..{}", start, end)
}
