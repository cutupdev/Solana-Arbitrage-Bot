//! Encryption utilities for sensitive data
//!
//! This module provides XOR-based encoding/decoding functions
//! for protecting sensitive configuration data.

/// Encode a string using XOR cipher with the provided key
pub fn encode_with_xor(data: &str, key: &str) -> String {
    let data_bytes = data.as_bytes();
    let key_bytes = key.as_bytes();
    let mut encoded = Vec::new();

    for (i, &byte) in data_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        encoded.push(byte ^ key_byte);
    }

    // Convert to hex string for better readability
    encoded.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Decode a hex-encoded string using XOR cipher with the provided key
pub fn decode_with_xor(hex_data: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Convert hex string back to bytes
    let mut data_bytes = Vec::new();
    for chunk in hex_data.as_bytes().chunks(2) {
        let hex_str = std::str::from_utf8(chunk)?;
        let byte = u8::from_str_radix(hex_str, 16)?;
        data_bytes.push(byte);
    }

    let key_bytes = key.as_bytes();
    let mut decoded = Vec::new();

    for (i, &byte) in data_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        decoded.push(byte ^ key_byte);
    }

    // Convert back to string
    Ok(String::from_utf8(decoded)?)
}
