//! Custom token operations
//!
//! Implements secure custom tokens with Blake2b-keyed HMAC and ChaCha20 encryption.
//! Uses embedded prehashseed system similar to web UI parameter encryption.

use blake2::{
    Blake2bMac, Blake2bVar,
    digest::{KeyInit as Blake2KeyInit, Mac, Update, VariableOutput},
};
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20poly1305::consts::U32;
use chrono::{DateTime, Duration, Utc};

use super::config::{
    get_access_token_cipher_key, get_access_token_duration_minutes, get_access_token_hmac_key,
    get_access_token_nonce_key, get_prehash_cipher_key, get_prehash_hmac_key,
    get_prehash_nonce_key, get_refresh_token_duration_minutes,
};
use super::crypto::{derive_user_id, user_id_to_username};
use super::types::AccessTokenClaims;

/// Token type enum
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum TokenType {
    Access,
    Refresh,
}

/// Custom token configuration for a specific token type
pub struct CustomTokenConfig {
    pub cipher_key: Vec<u8>,
    pub nonce_key: Vec<u8>,
    pub hmac_key: Vec<u8>,
    pub duration: Duration,
}

impl CustomTokenConfig {
    /// Get configuration for access tokens
    pub fn access_token() -> Result<Self, String> {
        Ok(CustomTokenConfig {
            cipher_key: get_access_token_cipher_key()?,
            nonce_key: get_access_token_nonce_key()?,
            hmac_key: get_access_token_hmac_key()?,
            duration: Duration::minutes(get_access_token_duration_minutes()? as i64),
        })
    }

    /// Get configuration for refresh tokens (same keys as access, different duration)
    pub fn refresh_token() -> Result<Self, String> {
        Ok(CustomTokenConfig {
            cipher_key: get_access_token_cipher_key()?, // Same keys as access token
            nonce_key: get_access_token_nonce_key()?,   // Same keys as access token
            hmac_key: get_access_token_hmac_key()?,     // Same keys as access token
            duration: Duration::minutes(get_refresh_token_duration_minutes()? as i64), // Only duration differs
        })
    }
}

/// Custom token claims (internal payload)
#[derive(Debug)]
pub struct CustomTokenClaims {
    pub user_id: [u8; 16],
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    pub token_type: TokenType,
    /// Ed25519 public key (32 bytes) for cryptographic operations
    pub pub_key: [u8; 32],
}

impl CustomTokenClaims {
    /// Create new claims from email, token type, and Ed25519 public key
    pub fn new(email: &str, token_type: TokenType, pub_key: &[u8; 32]) -> Result<Self, String> {
        let user_id = derive_user_id(email)?;
        let config = match token_type {
            TokenType::Access => CustomTokenConfig::access_token()?,
            TokenType::Refresh => CustomTokenConfig::refresh_token()?,
        };

        let now = Utc::now();
        let expires_at = now + config.duration;

        // All tokens share the same refresh expiration time for proactive renewal
        let refresh_config = CustomTokenConfig::refresh_token()?;
        let refresh_expires_at = now + refresh_config.duration;

        Ok(CustomTokenClaims {
            user_id,
            expires_at,
            refresh_expires_at,
            token_type,
            pub_key: *pub_key, // Ed25519 public key integration
        })
    }

    /// Create claims directly from user_id and Ed25519 public key (for username-based token creation)
    pub fn new_from_user_id(
        user_id: &[u8; 16],
        token_type: TokenType,
        pub_key: &[u8; 32],
    ) -> Result<Self, String> {
        let config = match token_type {
            TokenType::Access => CustomTokenConfig::access_token()?,
            TokenType::Refresh => CustomTokenConfig::refresh_token()?,
        };
        let now = Utc::now();
        let expires_at = now + config.duration;
        // All tokens share the same refresh expiration time for proactive renewal
        let refresh_config = CustomTokenConfig::refresh_token()?;
        let refresh_expires_at = now + refresh_config.duration;

        // DEBUG: Log token creation details (commented out for production)
        // println!("🔍 DEBUG new_from_user_id: token_type={:?}, duration_minutes={}, expires_at={}",
        //          token_type, config.duration.num_minutes(), expires_at);

        Ok(CustomTokenClaims {
            user_id: *user_id,
            expires_at,
            refresh_expires_at,
            token_type,
            pub_key: *pub_key, // Ed25519 public key integration
        })
    }

    /// Serialize claims to bytes: user_id(16) + expires_at(4) + refresh_expires_at(4) + pub_key(32) + blake2b_keyed(8) = 64 bytes
    pub fn to_bytes(&self, hmac_key: &[u8]) -> Result<[u8; 64], String> {
        // Timestamps as seconds since Unix epoch (4 bytes each, big-endian u32)
        let expires_timestamp = self.expires_at.timestamp() as u32;
        let refresh_expires_timestamp = self.refresh_expires_at.timestamp() as u32;
        let expires_bytes = expires_timestamp.to_be_bytes();
        let refresh_expires_bytes = refresh_expires_timestamp.to_be_bytes();

        // Prepare data for HMAC: user_id + expires_at + refresh_expires_at + pub_key
        let mut hmac_data = Vec::with_capacity(56);
        hmac_data.extend_from_slice(&self.user_id);
        hmac_data.extend_from_slice(&expires_bytes);
        hmac_data.extend_from_slice(&refresh_expires_bytes);
        hmac_data.extend_from_slice(&self.pub_key);

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
        payload[..16].copy_from_slice(&self.user_id);
        payload[16..20].copy_from_slice(&expires_bytes);
        payload[20..24].copy_from_slice(&refresh_expires_bytes);
        payload[24..56].copy_from_slice(&self.pub_key);
        payload[56..64].copy_from_slice(&compressed_hmac);

        Ok(payload)
    }

    /// Deserialize claims from bytes and validate integrity
    pub fn from_bytes(payload: &[u8; 64], hmac_key: &[u8]) -> Result<Self, String> {
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

        let expires_at = DateTime::from_timestamp(expires_timestamp as i64, 0)
            .ok_or("Invalid expires timestamp")?;
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
}

/// Generate cryptographically secure prehash seed (32 bytes)
pub fn generate_prehash_seed() -> [u8; 32] {
    use rand::RngCore;
    let mut rng = rand::rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    seed
}

/// Generate prehash using Blake2b-keyed (similar to web UI cryptoHashGen)
pub fn generate_prehash(seed: &[u8; 32], hmac_key: &[u8]) -> Result<[u8; 32], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(hmac_key)
        .map_err(|_| "Invalid HMAC key format for prehash".to_string())?;
    Mac::update(&mut keyed_hasher, seed);
    let result = keyed_hasher.finalize().into_bytes();

    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate 32-byte hash from 64-byte encrypted payload for key derivation
pub fn hash_encrypted_payload(encrypted_payload: &[u8; 64]) -> [u8; 32] {
    use blake2::{Blake2bVar, digest::VariableOutput};
    let mut hasher = Blake2bVar::new(32).expect("Blake2b initialization should not fail");
    hasher.update(encrypted_payload);
    let mut result = [0u8; 32];
    hasher
        .finalize_variable(&mut result)
        .expect("Blake2b finalization should not fail");
    result
}

/// Generate cipher key from base key and prehash (similar to web UI generateCipherKey)
pub fn generate_cipher_key(base_key: &[u8], prehash: &[u8; 32]) -> Result<[u8; 32], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(base_key)
        .map_err(|_| "Invalid base key format for cipher".to_string())?;
    Mac::update(&mut keyed_hasher, prehash);
    let result = keyed_hasher.finalize().into_bytes();

    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate nonce from base key and prehash (similar to web UI generateCipherNonce)
pub fn generate_cipher_nonce(base_key: &[u8], prehash: &[u8; 32]) -> Result<[u8; 12], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(base_key)
        .map_err(|_| "Invalid base key format for nonce".to_string())?;
    Mac::update(&mut keyed_hasher, prehash);
    let result = keyed_hasher.finalize().into_bytes();

    let mut output = [0u8; 12];
    output.copy_from_slice(&result[..12]);
    Ok(output)
}

/// Encrypt prehash seed with ChaCha20 (32 bytes)
pub fn encrypt_prehash_seed_data(
    seed: &[u8; 32],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 32], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut ciphertext = *seed;
    cipher.apply_keystream(&mut ciphertext);
    Ok(ciphertext)
}

/// Decrypt prehash seed with ChaCha20 (32 bytes)
pub fn decrypt_prehash_seed_data(
    ciphertext: &[u8; 32],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 32], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut plaintext = *ciphertext;
    cipher.apply_keystream(&mut plaintext);
    Ok(plaintext)
}

/// Encrypt payload with ChaCha20 (64 bytes for tokens)
pub fn encrypt_payload(
    payload: &[u8; 64],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 64], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut ciphertext = *payload;
    cipher.apply_keystream(&mut ciphertext);
    Ok(ciphertext)
}

/// Decrypt payload with ChaCha20 (64 bytes for tokens)
pub fn decrypt_payload(
    ciphertext: &[u8; 64],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 64], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut plaintext = *ciphertext;
    cipher.apply_keystream(&mut plaintext);
    Ok(plaintext)
}

/// Type alias for prehash encryption keys (cipher_key, nonce_key, hmac_key)
type PrehashKeys = (Vec<u8>, Vec<u8>, Vec<u8>);

/// Generate prehash seed encryption keys from encrypted payload (circular interdependence)
pub fn generate_prehash_encryption_keys(
    encrypted_payload: &[u8; 64],
) -> Result<PrehashKeys, String> {
    // Get base keys from environment
    let base_cipher_key = get_prehash_cipher_key()?;
    let base_nonce_key = get_prehash_nonce_key()?;
    let base_hmac_key = get_prehash_hmac_key()?;

    // Hash encrypted_payload to 32 bytes for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);

    // Use payload_hash as prehash to derive actual encryption keys
    let cipher_key = generate_cipher_key(&base_cipher_key, &payload_hash)?;
    let nonce_key = generate_cipher_key(&base_nonce_key, &payload_hash)?;
    let hmac_key = generate_cipher_key(&base_hmac_key, &payload_hash)?;

    Ok((cipher_key.to_vec(), nonce_key.to_vec(), hmac_key.to_vec()))
}

/// Encrypt prehash seed using circular interdependent encryption
pub fn encrypt_prehash_seed(
    prehash_seed: &[u8; 32],
    encrypted_payload: &[u8; 64],
) -> Result<[u8; 32], String> {
    // Generate encryption keys from encrypted_payload (circular dependency)
    let (cipher_key, nonce_key, hmac_key) = generate_prehash_encryption_keys(encrypted_payload)?;

    // Generate prehash from encrypted_payload hash for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);
    let prehash = generate_prehash(&payload_hash, &hmac_key)?;

    // Generate actual cipher key and nonce
    let final_cipher_key = generate_cipher_key(&cipher_key, &prehash)?;
    let final_cipher_nonce = generate_cipher_nonce(&nonce_key, &prehash)?;

    // Encrypt prehash_seed with ChaCha20
    encrypt_prehash_seed_data(prehash_seed, &final_cipher_key, &final_cipher_nonce)
}

/// Decrypt prehash seed using circular interdependent decryption
pub fn decrypt_prehash_seed(
    encrypted_prehash_seed: &[u8; 32],
    encrypted_payload: &[u8; 64],
) -> Result<[u8; 32], String> {
    // Generate decryption keys from encrypted_payload (same as encryption)
    let (cipher_key, nonce_key, hmac_key) = generate_prehash_encryption_keys(encrypted_payload)?;

    // Generate prehash from encrypted_payload hash for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);
    let prehash = generate_prehash(&payload_hash, &hmac_key)?;

    // Generate actual cipher key and nonce (same as encryption)
    let final_cipher_key = generate_cipher_key(&cipher_key, &prehash)?;
    let final_cipher_nonce = generate_cipher_nonce(&nonce_key, &prehash)?;

    // Decrypt encrypted_prehash_seed with ChaCha20
    decrypt_prehash_seed_data(
        encrypted_prehash_seed,
        &final_cipher_key,
        &final_cipher_nonce,
    )
}

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

/// Convert CustomTokenClaims to AccessTokenClaims for backwards compatibility
impl CustomTokenClaims {
    /// Convert to AccessTokenClaims structure expected by existing code
    pub fn to_access_token_claims(&self) -> AccessTokenClaims {
        let username = user_id_to_username(&self.user_id);
        let exp = self.expires_at.timestamp();
        let iat = (self.expires_at - match self.token_type {
            TokenType::Access => {
                Duration::minutes(get_access_token_duration_minutes().expect(
                    "CRITICAL: SPIN_VARIABLE_ACCESS_TOKEN_DURATION_MINUTES must be set in .env",
                ) as i64)
            }
            TokenType::Refresh => Duration::minutes(get_refresh_token_duration_minutes().expect(
                "CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env",
            ) as i64),
        })
        .timestamp();

        AccessTokenClaims {
            sub: username,
            exp,
            iat,
            token_type: match self.token_type {
                TokenType::Access => "access".to_string(),
                TokenType::Refresh => "refresh".to_string(),
            },
            refresh_expires_at: self.refresh_expires_at.timestamp(),
            pub_key: self.pub_key,
        }
    }
}

/// High-level API functions that maintain compatibility with existing JWT system
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
