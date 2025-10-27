//! JWT utilities implementation
//!
//! Provides backwards compatibility wrapper for the modularized JWT functionality.

use super::{
    crypto, custom_token_api, magic_links, tokens,
    types::{AccessTokenClaims, RefreshTokenClaims},
};

/// JWT token generation and validation utilities
///
/// This struct maintains backwards compatibility with the original monolithic implementation
/// while providing access to the modularized functionality.
pub struct JwtUtils;

#[allow(dead_code)]
impl JwtUtils {
    // Re-export crypto functions for backwards compatibility
    pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
        crypto::derive_user_id(email)
    }

    pub fn user_id_to_username(user_id: &[u8; 16]) -> String {
        crypto::user_id_to_username(user_id)
    }

    pub fn email_to_username(email: &str) -> Result<String, String> {
        crypto::email_to_username(email)
    }

    // Custom token functions for backwards compatibility
    pub fn create_access_token(
        email: &str,
        ed25519_pub_key: &[u8; 32],
        x25519_pub_key: &[u8; 32],
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        custom_token_api::create_custom_access_token(email, ed25519_pub_key, x25519_pub_key)
    }

    pub fn create_access_token_from_username(
        username: &str,
        ed25519_pub_key: &[u8; 32],
        x25519_pub_key: &[u8; 32],
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        custom_token_api::create_custom_access_token_from_username(
            username,
            ed25519_pub_key,
            x25519_pub_key,
        )
    }

    pub fn create_access_token_from_username_with_refresh_context(
        username: &str,
        refresh_expires_at: chrono::DateTime<chrono::Utc>,
        ed25519_pub_key: &[u8; 32],
        x25519_pub_key: &[u8; 32],
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        custom_token_api::create_custom_access_token_from_username_with_refresh_context(
            username,
            refresh_expires_at,
            ed25519_pub_key,
            x25519_pub_key,
        )
    }

    pub fn create_refresh_token(
        email: &str,
        session_id: i64,
        ed25519_pub_key: &[u8; 32],
        x25519_pub_key: &[u8; 32],
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        tokens::create_refresh_token(email, session_id, ed25519_pub_key, x25519_pub_key)
    }

    pub fn create_refresh_token_from_username(
        username: &str,
        ed25519_pub_key: &[u8; 32],
        x25519_pub_key: &[u8; 32],
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        tokens::create_refresh_token_from_username(username, ed25519_pub_key, x25519_pub_key)
    }

    pub fn validate_access_token(token: &str) -> Result<AccessTokenClaims, String> {
        custom_token_api::validate_custom_access_token(token)
    }

    pub fn validate_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
        tokens::validate_refresh_token(token)
    }

    // Re-export magic link functions for backwards compatibility
    pub fn generate_magic_token_encrypted(
        email: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(String, [u8; 44], i64, [u8; 16]), String> {
        magic_links::generate_magic_token_encrypted(email, expires_at)
    }

    pub fn validate_magic_token_encrypted(
        encrypted_token: &str,
        nonce: &[u8; 12],
        secret_key: &[u8; 32],
    ) -> Result<([u8; 16], chrono::DateTime<chrono::Utc>), String> {
        magic_links::validate_magic_token_encrypted(encrypted_token, nonce, secret_key)
    }

    pub fn create_magic_link_url(host_url: &str, magic_token: &str) -> String {
        magic_links::create_magic_link_url(host_url, magic_token)
    }
}
