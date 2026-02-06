use crate::*;
use once_cell::sync::Lazy;
use std::env;
use std::net::UdpSocket;

pub static TELEGRAM_BOT_KEY: Lazy<String> =
    Lazy::new(|| CONFIG.telegram.telegram_bot_key.clone());

pub static TG_GROUP_CHANNEL: Lazy<isize> =
    Lazy::new(|| CONFIG.telegram.group_channel_id.clone());

pub static USERNAME: Lazy<String> =
    Lazy::new(|| env::var("USER").unwrap_or_else(|_| "unknown".to_string()));

pub static LOCAL_IP: Lazy<String> = Lazy::new(|| {
    let socket = UdpSocket::bind("0.0.0.0:0").ok();
    if let Some(sock) = socket {
        if sock.connect("8.8.8.8:80").is_ok() {
            return sock
                .local_addr()
                .map(|a| a.ip().to_string())
                .unwrap_or_default();
        }
    }
    "unknown".to_string()
});