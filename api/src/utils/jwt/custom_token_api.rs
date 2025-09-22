//! Custom token API operations - High-level API functions for token creation and validation

use super::custom_token_crypto::{
    encrypt_payload, generate_cipher_key, generate_cipher_nonce, generate_prehash,
    generate_prehash_seed,
};
use super::custom_token_encryption::encrypt_prehash_seed;
use super::custom_token_types::{CustomTokenClaims, CustomTokenConfig, TokenType};
use super::custom_tokens::{generate_custom_token, validate_custom_token};
use super::types::AccessTokenClaims;
use chrono::{DateTime, Utc};
// Import compatibility module to make trait implementations available
#[allow(unused_imports)]
use super::custom_token_compat;

/// Create access token using custom token system with Ed25519 public key
pub fn create_custom_access_token(
    email: &str,
    pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let token = generate_custom_token(email, TokenType::Access, pub_key)?;
    let claims = CustomTokenClaims::new(email, TokenType::Access, pub_key)?;
    Ok((token, claims.expires_at))
}
/// Create refresh token using custom token system with Ed25519 public key
pub fn create_custom_refresh_token(
    email: &str,
    pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let token = generate_custom_token(email, TokenType::Refresh, pub_key)?;
    let claims = CustomTokenClaims::new(email, TokenType::Refresh, pub_key)?;
    Ok((token, claims.expires_at))
}
/// Create refresh token from username using custom token system with optional Ed25519 public key
pub fn create_custom_refresh_token_from_username(
    username: &str,
    pub_key: Option<&[u8; 32]>,
) -> Result<(String, DateTime<Utc>), String> {
    // Convert username back to user_id bytes
    let user_id_bytes = bs58::decode(username)
        .into_vec()
        .map_err(|_| "Invalid username format")?;

    if user_id_bytes.len() != 16 {
        return Err("Invalid username length".to_string());
    }

    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_bytes);

    // Create claims with proper user_id and Ed25519 public key
    let default_pub_key = [0u8; 32]; // Fallback for compatibility
    let pub_key_to_use = pub_key.unwrap_or(&default_pub_key);
    let claims = CustomTokenClaims::new_from_user_id(&user_id, TokenType::Refresh, pub_key_to_use)?;

    // Generate token manually using the same logic as generate_custom_token()
    let config = CustomTokenConfig::refresh_token()?;
    let prehash_seed = generate_prehash_seed();
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;
    let payload = claims.to_bytes(&config.hmac_key)?;
    let encrypted_payload = encrypt_payload(&payload, &cipher_key, &cipher_nonce)?;
    let encrypted_prehash_seed = encrypt_prehash_seed(&prehash_seed, &encrypted_payload)?;

    let mut combined = [0u8; 96];
    combined[..32].copy_from_slice(&encrypted_prehash_seed);
    combined[32..96].copy_from_slice(&encrypted_payload);
    let token = bs58::encode(&combined).into_string();

    Ok((token, claims.expires_at))
}

/// Create access token from username using custom token system (compatible with existing API)
pub fn create_custom_access_token_from_username(
    username: &str,
    pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    // Convert username back to user_id, then derive email (simplified approach)
    // For now, we'll use the username as a pseudo-email since we have the conversion functions
    // This is a temporary bridge - in real implementation, we'd need to store email/username mapping

    // Extract user_id bytes from username (Base58 decoded)
    let user_id_bytes = bs58::decode(username)
        .into_vec()
        .map_err(|_| "Invalid username format")?;

    if user_id_bytes.len() != 16 {
        return Err("Invalid username length".to_string());
    }

    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_bytes);

    // Create claims directly from user_id
    let config = CustomTokenConfig::access_token()?;
    let now = Utc::now();
    let expires_at = now + config.duration;

    // Calculate refresh token expiration for proactive renewal
    let refresh_config = CustomTokenConfig::refresh_token()?;
    let refresh_expires_at = now + refresh_config.duration;

    let claims = CustomTokenClaims {
        user_id,
        expires_at,
        refresh_expires_at,
        token_type: TokenType::Access,
        pub_key: *pub_key, // Ed25519 public key integration
    };

    // Generate token using same secure method as generate_custom_token()
    let prehash_seed = generate_prehash_seed();
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;
    let payload = claims.to_bytes(&config.hmac_key)?;
    let encrypted_payload = encrypt_payload(&payload, &cipher_key, &cipher_nonce)?;

    // ULTRA-SECURE: Encrypt prehash_seed using encrypted_payload as circular dependency
    let encrypted_prehash_seed = encrypt_prehash_seed(&prehash_seed, &encrypted_payload)?;

    let mut combined = [0u8; 96];
    combined[..32].copy_from_slice(&encrypted_prehash_seed);
    combined[32..96].copy_from_slice(&encrypted_payload);

    let token = bs58::encode(&combined).into_string();
    Ok((token, expires_at))
}

/// Validate any token using custom token system (validation logic is same: expiration_timestamp < now)
pub fn validate_custom_access_token(token: &str) -> Result<AccessTokenClaims, String> {
    // Since both token types now use the same keys, try access first (most common)
    let access_result = validate_custom_token(token, TokenType::Access);
    if let Ok(claims) = access_result {
        return Ok(claims.to_access_token_claims());
    }

    // Fallback to refresh (though should work with either due to same keys)
    let refresh_result = validate_custom_token(token, TokenType::Refresh);
    if let Ok(claims) = refresh_result {
        return Ok(claims.to_access_token_claims());
    }

    // ENHANCED ERROR DETECTION: Check if token is expired even if other validations fail
    // This allows middleware to detect true expiration vs corruption/invalidity
    let access_error = access_result.unwrap_err();
    let refresh_error = refresh_result.unwrap_err();

    // If either validation reached expiration check, prefer that error
    if access_error.contains("expired") {
        Err(access_error)
    } else if refresh_error.contains("expired") {
        Err(refresh_error)
    } else {
        // No expiration detected in either validation - token is invalid for other reasons
        Err("Invalid token - corrupted, malformed, or wrong key".to_string())
    }
}

/// Validate custom refresh token specifically (uses refresh token configuration only)
pub fn validate_custom_refresh_token(token: &str) -> Result<AccessTokenClaims, String> {
    let claims = validate_custom_token(token, TokenType::Refresh)?;
    Ok(claims.to_access_token_claims())
}

/// Create access token from username preserving refresh context (for system 2/3)
///
/// This function creates a new access token while preserving the refresh_expires_at
/// from the original refresh token context. This is essential for the 2/3 system
/// to work correctly - when renewing only the access token, the new access token
/// must maintain the original refresh expiration time for proper proactive renewal.
///
/// # Arguments
/// * `username` - Base58 encoded user ID
/// * `refresh_expires_at` - Original refresh token expiration to preserve
///
/// # Returns
/// * `Result<(String, DateTime<Utc>), String>` - New access token and its expiration
pub fn create_custom_access_token_from_username_with_refresh_context(
    username: &str,
    refresh_expires_at: DateTime<Utc>,
    pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    // Convert username back to user_id bytes
    let user_id_bytes = bs58::decode(username)
        .into_vec()
        .map_err(|_| "Invalid username format")?;

    if user_id_bytes.len() != 16 {
        return Err("Invalid username length".to_string());
    }

    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_bytes);

    // Create claims directly from user_id
    let config = CustomTokenConfig::access_token()?;
    let now = Utc::now();
    let expires_at = now + config.duration;

    // CRITICAL: Use provided refresh_expires_at instead of calculating new one
    // This preserves the original refresh token timeline for 2/3 system
    let claims = CustomTokenClaims {
        user_id,
        expires_at,
        refresh_expires_at, // ← FIXED: Use original refresh_expires_at
        token_type: TokenType::Access,
        pub_key: *pub_key, // Ed25519 public key integration
    };

    // Generate token using same secure method as generate_custom_token()
    let prehash_seed = generate_prehash_seed();
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;
    let payload = claims.to_bytes(&config.hmac_key)?;
    let encrypted_payload = encrypt_payload(&payload, &cipher_key, &cipher_nonce)?;

    // ULTRA-SECURE: Encrypt prehash_seed using encrypted_payload as circular dependency
    let encrypted_prehash_seed = encrypt_prehash_seed(&prehash_seed, &encrypted_payload)?;

    let mut combined = [0u8; 96];
    combined[..32].copy_from_slice(&encrypted_prehash_seed);
    combined[32..96].copy_from_slice(&encrypted_payload);

    let token = bs58::encode(&combined).into_string();
    Ok((token, expires_at))
}
