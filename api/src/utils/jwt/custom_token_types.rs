//! Custom token types and configuration
//!
//! Single Responsibility: Define token types, configuration structures, and basic operations

use chrono::{DateTime, Duration, Utc};

use super::config::{
    get_access_token_cipher_key, get_access_token_duration_minutes, get_access_token_hmac_key,
    get_access_token_nonce_key, get_refresh_token_duration_minutes,
};
use super::crypto::derive_user_id;
use super::custom_token_serialization::{claims_to_bytes, claims_from_bytes};

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

    /// Serialize claims to bytes using dedicated serialization module
    pub fn to_bytes(&self, hmac_key: &[u8]) -> Result<[u8; 64], String> {
        claims_to_bytes(self, hmac_key)
    }

    /// Deserialize claims from bytes using dedicated serialization module
    pub fn from_bytes(payload: &[u8; 64], hmac_key: &[u8]) -> Result<Self, String> {
        claims_from_bytes(payload, hmac_key)
    }
}

// Implementation of to_access_token_claims is now in custom_token_compat module