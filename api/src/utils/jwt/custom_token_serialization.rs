//! Custom token serialization operations - Binary serialization and deserialization of token claims

use crate::utils::pseudonimizer::blake3_keyed_variable;
use chrono::DateTime;

use super::custom_token_types::{CustomTokenClaims, TokenType};

/// Serialize claims to bytes: user_id(16) + expires_at(4) + refresh_expires_at(4) + pub_key(32) + blake3_keyed(8) = 64 bytes
pub fn claims_to_bytes(
    claims: &CustomTokenClaims,
    hmac_key: &[u8; 64],
) -> Result<[u8; 64], String> {
    // Timestamps as seconds since Unix epoch (4 bytes each, big-endian u32)
    let expires_timestamp = claims.expires_at.timestamp() as u32;
    let refresh_expires_timestamp = claims.refresh_expires_at.timestamp() as u32;
    let expires_bytes = expires_timestamp.to_be_bytes();
    let refresh_expires_bytes = refresh_expires_timestamp.to_be_bytes();

    // Prepare data for HMAC: user_id + expires_at + refresh_expires_at + pub_key
    let mut hmac_data = Vec::with_capacity(56);
    hmac_data.extend_from_slice(&claims.user_id);
    hmac_data.extend_from_slice(&expires_bytes);
    hmac_data.extend_from_slice(&refresh_expires_bytes);
    hmac_data.extend_from_slice(&claims.pub_key);

    // Generate Blake3 keyed hash for integrity (direct 8 bytes)
    let compressed_hmac = blake3_keyed_variable(hmac_key, &hmac_data, 8);

    // Create final payload: user_id + expires_at + refresh_expires_at + pub_key + compressed_hmac (64 bytes)
    let mut payload = [0u8; 64];
    payload[..16].copy_from_slice(&claims.user_id);
    payload[16..20].copy_from_slice(&expires_bytes);
    payload[20..24].copy_from_slice(&refresh_expires_bytes);
    payload[24..56].copy_from_slice(&claims.pub_key);
    payload[56..64].copy_from_slice(&compressed_hmac);

    Ok(payload)
}

/// Deserialize claims from bytes and validate integrity
pub fn claims_from_bytes(
    payload: &[u8; 64],
    hmac_key: &[u8; 64],
) -> Result<CustomTokenClaims, String> {
    if payload.len() != 64 {
        return Err("Invalid payload length".to_string());
    }

    // Extract components
    let user_id_bytes = &payload[0..16];
    let expires_bytes = &payload[16..20];
    let refresh_expires_bytes = &payload[20..24];
    let pub_key_bytes = &payload[24..56];
    let provided_compressed_hmac = &payload[56..64];

    // Verify Blake3 keyed hash integrity
    let mut verification_data = Vec::with_capacity(56);
    verification_data.extend_from_slice(user_id_bytes);
    verification_data.extend_from_slice(expires_bytes);
    verification_data.extend_from_slice(refresh_expires_bytes);
    verification_data.extend_from_slice(pub_key_bytes);

    // Generate Blake3 keyed hash for verification (direct 8 bytes)
    let expected_compressed_hmac = blake3_keyed_variable(hmac_key, &verification_data, 8);

    // Verify HMAC integrity
    if provided_compressed_hmac != expected_compressed_hmac {
        return Err("Token integrity verification failed - corrupted or wrong key".to_string());
    }

    // Extract timestamps (4 bytes each, u32 seconds since Unix epoch)
    let expires_timestamp = u32::from_be_bytes(
        expires_bytes
            .try_into()
            .map_err(|_| "Invalid expires timestamp format")?,
    );
    let refresh_expires_timestamp = u32::from_be_bytes(
        refresh_expires_bytes
            .try_into()
            .map_err(|_| "Invalid refresh expires timestamp format")?,
    );

    let expires_at =
        DateTime::from_timestamp(expires_timestamp as i64, 0).ok_or("Invalid expires timestamp")?;
    let refresh_expires_at = DateTime::from_timestamp(refresh_expires_timestamp as i64, 0)
        .ok_or("Invalid refresh expires timestamp")?;

    // Convert user_id bytes to array
    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(user_id_bytes);

    // Convert pub_key bytes to array
    let mut pub_key = [0u8; 32];
    pub_key.copy_from_slice(pub_key_bytes);

    // Token type will be determined by validation context
    Ok(CustomTokenClaims {
        user_id,
        expires_at,
        refresh_expires_at,
        token_type: TokenType::Access, // Will be overridden by caller
        pub_key,
    })
}
