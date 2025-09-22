//! Custom token serialization operations - Binary serialization and deserialization of token claims

use blake2::{
    Blake2bMac, Blake2bVar,
    digest::{KeyInit as Blake2KeyInit, Mac, Update, VariableOutput},
};
use chacha20poly1305::consts::U32;
use chrono::DateTime;

use super::custom_token_types::{CustomTokenClaims, TokenType};

/// Serialize claims to bytes: user_id(16) + expires_at(4) + refresh_expires_at(4) + pub_key(32) + blake2b_keyed(8) = 64 bytes
pub fn claims_to_bytes(claims: &CustomTokenClaims, hmac_key: &[u8]) -> Result<[u8; 64], String> {
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

    // Generate Blake2b keyed hash for integrity
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(hmac_key)
        .map_err(|_| "Invalid HMAC key format".to_string())?;
    Mac::update(&mut keyed_hasher, &hmac_data);
    let hmac_result = keyed_hasher.finalize().into_bytes();

    // Compress to 8 bytes using Blake2b variable output
    let mut compressor =
        Blake2bVar::new(8).map_err(|_| "Blake2b initialization failed".to_string())?;
    compressor.update(&hmac_result);
    let mut compressed_hmac = [0u8; 8];
    compressor
        .finalize_variable(&mut compressed_hmac)
        .map_err(|_| "Blake2b finalization failed".to_string())?;

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
pub fn claims_from_bytes(payload: &[u8; 64], hmac_key: &[u8]) -> Result<CustomTokenClaims, String> {
    if payload.len() != 64 {
        return Err("Invalid payload length".to_string());
    }

    // Extract components
    let user_id_bytes = &payload[0..16];
    let expires_bytes = &payload[16..20];
    let refresh_expires_bytes = &payload[20..24];
    let pub_key_bytes = &payload[24..56];
    let provided_compressed_hmac = &payload[56..64];

    // Verify Blake2b keyed hash integrity
    let mut verification_data = Vec::with_capacity(56);
    verification_data.extend_from_slice(user_id_bytes);
    verification_data.extend_from_slice(expires_bytes);
    verification_data.extend_from_slice(refresh_expires_bytes);
    verification_data.extend_from_slice(pub_key_bytes);

    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(hmac_key)
        .map_err(|_| "Invalid HMAC key format".to_string())?;
    Mac::update(&mut keyed_hasher, &verification_data);
    let hmac_result = keyed_hasher.finalize().into_bytes();

    // Compress to 8 bytes using Blake2b variable output
    let mut compressor =
        Blake2bVar::new(8).map_err(|_| "Blake2b initialization failed".to_string())?;
    compressor.update(&hmac_result);
    let mut expected_compressed_hmac = [0u8; 8];
    compressor
        .finalize_variable(&mut expected_compressed_hmac)
        .map_err(|_| "Blake2b finalization failed".to_string())?;

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
