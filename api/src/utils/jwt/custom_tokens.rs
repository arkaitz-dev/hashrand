//! Custom token operations
//!
//! Implements secure custom tokens with Blake2b-keyed HMAC and ChaCha20 encryption.
//! Uses embedded prehashseed system similar to web UI parameter encryption.

use chrono::Utc;
use super::custom_token_crypto::{
    generate_prehash_seed, generate_prehash,
    generate_cipher_key, generate_cipher_nonce, encrypt_payload, decrypt_payload
};
use super::custom_token_encryption::{
    encrypt_prehash_seed, decrypt_prehash_seed
};
use super::custom_token_types::{TokenType, CustomTokenConfig, CustomTokenClaims};

// TokenType is now imported from custom_token_types module

// CustomTokenConfig and CustomTokenClaims are now imported from custom_token_types module

// All cryptographic functions are now imported from custom_token_crypto module

// Prehash encryption functions are now imported from custom_token_encryption module

/// Generate custom token (access or refresh) with ultra-secure circular encryption and Ed25519 public key
pub fn generate_custom_token(
    email: &str,
    token_type: TokenType,
    pub_key: &[u8; 32],
) -> Result<String, String> {
    // 1. Create claims with user_id, expiration, and Ed25519 public key
    let claims = CustomTokenClaims::new(email, token_type, pub_key)?;

    // 2. Get token configuration
    let config = match token_type {
        TokenType::Access => CustomTokenConfig::access_token()?,
        TokenType::Refresh => CustomTokenConfig::refresh_token()?,
    };

    // 3. Generate random prehash seed (completely random as in web UI)
    let prehash_seed = generate_prehash_seed();

    // 4. Generate prehash from seed for payload encryption
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;

    // 5. Generate cipher key and nonce for payload
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;

    // 6. Serialize claims to payload
    let payload = claims.to_bytes(&config.hmac_key)?;

    // 7. Encrypt payload with prehash_seed derived keys
    let encrypted_payload = encrypt_payload(&payload, &cipher_key, &cipher_nonce)?;

    // 8. ULTRA-SECURE: Encrypt prehash_seed using encrypted_payload as circular dependency
    let encrypted_prehash_seed = encrypt_prehash_seed(&prehash_seed, &encrypted_payload)?;

    // 9. Combine encrypted_prehash_seed(32) + encrypted_payload(64) = 96 bytes
    let mut combined = [0u8; 96];
    combined[..32].copy_from_slice(&encrypted_prehash_seed);
    combined[32..96].copy_from_slice(&encrypted_payload);

    // 10. Encode as Base58
    Ok(bs58::encode(&combined).into_string())
}

/// Validate custom token (access or refresh) with ultra-secure circular decryption
pub fn validate_custom_token(
    token: &str,
    token_type: TokenType,
) -> Result<CustomTokenClaims, String> {
    println!(
        "🔍 DEBUG validate_custom_token: Starting validation for token type: {:?}",
        token_type
    );

    // 1. Decode Base58 token
    let combined = bs58::decode(token)
        .into_vec()
        .map_err(|_| "Invalid Base58 token encoding")?;

    println!(
        "🔍 DEBUG validate_custom_token: Token decoded, length: {}",
        combined.len()
    );

    if combined.len() != 96 {
        return Err(format!(
            "Invalid token length: expected 96 bytes, got {}",
            combined.len()
        ));
    }

    // 2. Extract encrypted_prehash_seed(32) + encrypted_payload(64)
    let mut encrypted_prehash_seed = [0u8; 32];
    let mut encrypted_payload = [0u8; 64];
    encrypted_prehash_seed.copy_from_slice(&combined[..32]);
    encrypted_payload.copy_from_slice(&combined[32..96]);

    // 3. ULTRA-SECURE: Decrypt prehash_seed using encrypted_payload as circular dependency
    let prehash_seed = decrypt_prehash_seed(&encrypted_prehash_seed, &encrypted_payload)?;
    println!("🔍 DEBUG validate_custom_token: Prehash seed decrypted successfully");

    // 4. Get token configuration for payload decryption
    let config = match token_type {
        TokenType::Access => CustomTokenConfig::access_token()?,
        TokenType::Refresh => CustomTokenConfig::refresh_token()?,
    };
    println!(
        "🔍 DEBUG validate_custom_token: Got token config for {:?}",
        token_type
    );
    // println!("🔍 DEBUG validate_custom_token: About to generate prehash and keys");

    // 5. Generate prehash from decrypted seed for payload decryption
    let prehash = generate_prehash(&prehash_seed, &config.hmac_key)?;

    // 6. Generate cipher key and nonce for payload decryption
    let cipher_key = generate_cipher_key(&config.cipher_key, &prehash)?;
    let cipher_nonce = generate_cipher_nonce(&config.nonce_key, &prehash)?;

    // 7. Decrypt payload with prehash_seed derived keys
    let payload = decrypt_payload(&encrypted_payload, &cipher_key, &cipher_nonce)?;

    // 8. Deserialize and validate claims
    println!("🔍 DEBUG validate_custom_token: About to deserialize claims");
    let mut claims = CustomTokenClaims::from_bytes(&payload, &config.hmac_key)?;
    claims.token_type = token_type; // Set correct token type
    println!("🔍 DEBUG validate_custom_token: Claims deserialized successfully");

    // 9. Check expiration
    let now = Utc::now();
    println!(
        "🔍 DEBUG validate_custom_token: Token expires at: {}, now: {}",
        claims.expires_at, now
    );
    if now > claims.expires_at {
        println!("🔍 DEBUG validate_custom_token: Token is expired, returning error");
        return Err("Token has expired - please refresh or re-authenticate".to_string());
    }
    // println!("🔍 DEBUG validate_custom_token: Token is valid and not expired, returning success");

    println!("🔍 DEBUG validate_custom_token: Validation completed successfully");
    Ok(claims)
}

// High-level API functions are now in custom_token_api module
