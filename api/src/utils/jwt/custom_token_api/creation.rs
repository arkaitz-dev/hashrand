use super::super::custom_token_crypto::{
    encrypt_payload, generate_cipher_key, generate_cipher_nonce, generate_prehash,
    generate_prehash_seed,
};
use super::super::custom_token_encryption::encrypt_prehash_seed;
use super::super::custom_token_types::{CustomTokenClaims, CustomTokenConfig, TokenType};
use super::super::custom_tokens::generate_custom_token;
use super::conversion::username_to_user_id;
use chrono::{DateTime, Utc};

/// Create access token using custom token system with Ed25519 and X25519 public keys
pub fn create_custom_access_token(
    email: &str,
    ed25519_pub_key: &[u8; 32],
    x25519_pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let token = generate_custom_token(email, TokenType::Access, ed25519_pub_key, x25519_pub_key)?;
    let claims = CustomTokenClaims::new(email, TokenType::Access, ed25519_pub_key, x25519_pub_key)?;
    Ok((token, claims.expires_at))
}

/// Create refresh token using custom token system with Ed25519 and X25519 public keys
pub fn create_custom_refresh_token(
    email: &str,
    ed25519_pub_key: &[u8; 32],
    x25519_pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let token = generate_custom_token(email, TokenType::Refresh, ed25519_pub_key, x25519_pub_key)?;
    let claims =
        CustomTokenClaims::new(email, TokenType::Refresh, ed25519_pub_key, x25519_pub_key)?;
    Ok((token, claims.expires_at))
}

/// Generate token from claims (DRY utility for token generation)
///
/// # Arguments
/// * `claims` - CustomTokenClaims to encode
/// * `config` - Token configuration (access or refresh)
///
/// # Returns
/// * `Result<String, String>` - Base58 encoded token or error
fn generate_token_from_claims(
    claims: &CustomTokenClaims,
    config: &CustomTokenConfig,
) -> Result<String, String> {
    let prehash_seed = generate_prehash_seed();
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;
    let payload = claims.to_bytes(&config.hmac_key)?;
    let encrypted_payload = encrypt_payload(&payload, &cipher_key, &cipher_nonce)?;
    let encrypted_prehash_seed = encrypt_prehash_seed(&prehash_seed, &encrypted_payload)?;

    let mut combined = [0u8; 128];
    combined[..32].copy_from_slice(&encrypted_prehash_seed);
    combined[32..128].copy_from_slice(&encrypted_payload);
    let token = bs58::encode(&combined).into_string();

    Ok(token)
}

/// Create refresh token from username using custom token system with Ed25519 and X25519 public keys
pub fn create_custom_refresh_token_from_username(
    username: &str,
    ed25519_pub_key: &[u8; 32],
    x25519_pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let user_id = username_to_user_id(username)?;

    // Create claims with proper user_id, Ed25519 and X25519 public keys
    let claims = CustomTokenClaims::new_from_user_id(
        &user_id,
        TokenType::Refresh,
        ed25519_pub_key,
        x25519_pub_key,
    )?;

    // Generate token using DRY utility
    let config = CustomTokenConfig::refresh_token()?;
    let token = generate_token_from_claims(&claims, &config)?;

    Ok((token, claims.expires_at))
}

/// Create access token from username using custom token system (compatible with existing API)
pub fn create_custom_access_token_from_username(
    username: &str,
    ed25519_pub_key: &[u8; 32],
    x25519_pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let user_id = username_to_user_id(username)?;

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
        ed25519_pub_key: *ed25519_pub_key,
        x25519_pub_key: *x25519_pub_key,
    };

    // Generate token using DRY utility
    let token = generate_token_from_claims(&claims, &config)?;
    Ok((token, expires_at))
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
/// * `ed25519_pub_key` - Ed25519 public key for signature verification
/// * `x25519_pub_key` - X25519 public key for ECDH E2E encryption
///
/// # Returns
/// * `Result<(String, DateTime<Utc>), String>` - New access token and its expiration
pub fn create_custom_access_token_from_username_with_refresh_context(
    username: &str,
    refresh_expires_at: DateTime<Utc>,
    ed25519_pub_key: &[u8; 32],
    x25519_pub_key: &[u8; 32],
) -> Result<(String, DateTime<Utc>), String> {
    let user_id = username_to_user_id(username)?;

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
        ed25519_pub_key: *ed25519_pub_key,
        x25519_pub_key: *x25519_pub_key,
    };

    // Generate token using DRY utility
    let token = generate_token_from_claims(&claims, &config)?;
    Ok((token, expires_at))
}
