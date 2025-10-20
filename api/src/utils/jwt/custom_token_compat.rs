//! Custom token backwards compatibility operations
//!
//! Single Responsibility: Conversion functions for backwards compatibility with existing APIs

use chrono::Duration;

use super::config::{get_access_token_duration_minutes, get_refresh_token_duration_minutes};
use super::crypto::user_id_to_username;
use super::custom_token_types::{CustomTokenClaims, TokenType};
use super::types::AccessTokenClaims;

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
            ed25519_pub_key: self.ed25519_pub_key,
            x25519_pub_key: self.x25519_pub_key,
        }
    }
}
