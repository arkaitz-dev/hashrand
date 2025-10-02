//! Magic link operations
//!
//! Handles generation and validation of encrypted magic links for authentication.

use chrono::{DateTime, Utc};

use super::config::get_magic_link_hmac_key;
use super::crypto::{
    decrypt_magic_link, derive_user_id, encrypt_magic_link, generate_chacha_nonce_and_key,
};
use crate::utils::pseudonimizer::blake3_keyed_variable;

/// Generate secure magic token with ChaCha20 encryption
///
/// Enhanced Process:
/// 1. Create raw_magic_link: user_id (16) + timestamp (8) + Blake3-keyed-variable[8] (hmac) = 32 bytes
/// 2. Generate nonce[12] + secret_key[32] from Blake3-keyed-variable(chacha_key[64], raw_magic_link, 44)
/// 3. Encrypt raw_magic_link with ChaCha20 → encrypted_raw_magic_link
/// 4. Return Base58(encrypted_raw_magic_link) for transmission + encryption_blob + timestamp for database
///
/// # Arguments
/// * `email` - User email to derive user_id
/// * `expires_at` - Magic link expiration timestamp
///
/// # Returns
/// * `Result<(String, [u8; 44], i64), String>` - (Base58 token, encryption_blob, timestamp) or error
pub fn generate_magic_token_encrypted(
    email: &str,
    expires_at: DateTime<Utc>,
) -> Result<(String, [u8; 44], i64), String> {
    // Derive deterministic user_id from email
    let user_id = derive_user_id(email)?;

    // Timestamp as nanoseconds since Unix epoch (8 bytes, big-endian u64)
    let timestamp_nanos = expires_at
        .timestamp_nanos_opt()
        .ok_or("Timestamp overflow in nanoseconds conversion")?;
    let timestamp_bytes = timestamp_nanos.to_be_bytes();

    // Prepare data for HMAC: user_id + timestamp
    let mut data = Vec::with_capacity(24);
    data.extend_from_slice(&user_id);
    data.extend_from_slice(&timestamp_bytes);

    // Generate 8-byte HMAC using Blake3 pseudonimizer
    let hmac_key = get_magic_link_hmac_key().map_err(|e| format!("HMAC key error: {}", e))?;
    let hmac_output = blake3_keyed_variable(&hmac_key, &data, 8);
    let mut compressed_hmac = [0u8; 8];
    compressed_hmac.copy_from_slice(&hmac_output);

    // Create raw_magic_link: user_id + timestamp + compressed_hmac (32 bytes)
    let mut raw_magic_link = [0u8; 32];
    raw_magic_link[..16].copy_from_slice(&user_id);
    raw_magic_link[16..24].copy_from_slice(&timestamp_bytes);
    raw_magic_link[24..32].copy_from_slice(&compressed_hmac);

    // Generate nonce and secret key from raw_magic_link
    let (nonce, secret_key) = generate_chacha_nonce_and_key(&raw_magic_link)?;

    // Encrypt raw_magic_link with ChaCha20
    let encrypted_data = encrypt_magic_link(&raw_magic_link, &nonce, &secret_key)?;

    if encrypted_data.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", encrypted_data.len()));
    }

    // Create encryption_blob: nonce[12] + secret_key[32] = 44 bytes
    let mut encryption_blob = [0u8; 44];
    encryption_blob[..12].copy_from_slice(&nonce);
    encryption_blob[12..44].copy_from_slice(&secret_key);

    // Return encrypted data as Base58 token, encryption_blob, and original timestamp
    Ok((
        bs58::encode(&encrypted_data).into_string(),
        encryption_blob,
        timestamp_nanos,
    ))
}

/// Validate encrypted magic token using ChaCha20-Poly1305 decryption
///
/// Process:
/// 1. Decode Base58 encrypted token
/// 2. Decrypt with ChaCha20-Poly1305 using nonce + secret_key → raw_magic_link
/// 3. Extract and validate HMAC integrity
/// 4. Return user_id and timestamp from decrypted data
///
/// # Arguments
/// * `encrypted_token` - Base58 encoded encrypted magic token
/// * `nonce` - 12-byte nonce from encryption_blob
/// * `secret_key` - 32-byte secret key from encryption_blob
///
/// # Returns
/// * `Result<([u8; 16], DateTime<Utc>), String>` - (user_id, expiration) or validation error
pub fn validate_magic_token_encrypted(
    encrypted_token: &str,
    nonce: &[u8; 12],
    secret_key: &[u8; 32],
) -> Result<([u8; 16], DateTime<Utc>), String> {
    // Decode Base58 encrypted token
    let encrypted_data = bs58::decode(encrypted_token)
        .into_vec()
        .map_err(|_| "Invalid Base58 encoding")?;

    // Decrypt with ChaCha20-Poly1305
    let raw_magic_link = decrypt_magic_link(&encrypted_data, nonce, secret_key)?;

    // Extract components from decrypted raw_magic_link
    let user_id_bytes = &raw_magic_link[0..16];
    let timestamp_bytes = &raw_magic_link[16..24];
    let provided_compressed_hmac = &raw_magic_link[24..32];

    // Verify Blake3 HMAC integrity (same as generation)
    let hmac_key = get_magic_link_hmac_key().map_err(|e| format!("HMAC key error: {}", e))?;

    // Prepare data for verification (same as generation)
    let mut verification_data = Vec::with_capacity(24);
    verification_data.extend_from_slice(user_id_bytes);
    verification_data.extend_from_slice(timestamp_bytes);

    let hmac_output = blake3_keyed_variable(&hmac_key, &verification_data, 8);
    let mut expected_compressed_hmac = [0u8; 8];
    expected_compressed_hmac.copy_from_slice(&hmac_output);

    // Compare compressed HMAC values
    if provided_compressed_hmac == expected_compressed_hmac {
        // Extract timestamp
        let timestamp = u64::from_be_bytes(
            timestamp_bytes
                .try_into()
                .map_err(|_| "Invalid timestamp format")?,
        );

        let expires_at = DateTime::from_timestamp_nanos(timestamp as i64);

        // Convert user_id bytes to array
        let mut user_id = [0u8; 16];
        user_id.copy_from_slice(user_id_bytes);

        Ok((user_id, expires_at))
    } else {
        Err("Token integrity verification failed".to_string())
    }
}

/// Create magic link URL with automatic protocol detection
///
/// Automatically adds the appropriate protocol if not present:
/// - localhost/127.0.0.1 → http://
/// - Any other domain → https://
///
/// # Arguments
/// * `host_url` - Base URL from request (with or without protocol)
/// * `magic_token` - Magic token to include in URL
///
/// # Returns
/// * `String` - Complete magic link URL with protocol
pub fn create_magic_link_url(host_url: &str, magic_token: &str) -> String {
    let base_url = host_url.trim_end_matches('/');

    // Check if protocol is already present
    let url_with_protocol = if base_url.starts_with("http://") || base_url.starts_with("https://") {
        // Protocol already present - use as is
        base_url.to_string()
    } else {
        // No protocol - add appropriate one based on host
        if base_url.contains("localhost") || base_url.contains("127.0.0.1") {
            // Development: use http://
            format!("http://{}", base_url)
        } else {
            // Production/remote: use https://
            format!("https://{}", base_url)
        }
    };

    format!("{}/?magiclink={}", url_with_protocol, magic_token)
}
