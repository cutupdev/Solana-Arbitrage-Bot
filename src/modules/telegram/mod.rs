use reqwest::Client;
use serde::Serialize;

use crate::*; // Your constants like TELEGRAM_BOT_KEY, TG_GROUP_CHANNEL, USERNAME, LOCAL_IP, PUBKEY

#[derive(Serialize)]
struct SendMessageRequest {
    chat_id: String,
    text: String,
    parse_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

/// Send a single message to Telegram via bot API
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

/// Format and send a message to the single chat
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

/// Helper to send a formatted message with title and link
pub async fn tg_msg(title: &str, text: String) {
    let log = format!(
        "{}\n\n<a href=\"{}\" target=\"_blank\">{}</a>",
        title, text, text
    );
    send_messages(log).await;
}

/// Abbreviate a public key or long address
pub fn abbreviate_address(address: String) -> String {
    if address.len() <= 8 {
        return address;
    }
    let start = &address[..4];
    let end = &address[address.len() - 4..];
    format!("{}..{}", start, end)
}
