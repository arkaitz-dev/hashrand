//! JWT utilities implementation
//!
//! Provides backwards compatibility wrapper for the modularized JWT functionality.

use super::{
    crypto, custom_tokens, magic_links, tokens,
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
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        custom_tokens::create_custom_access_token(email)
    }

    pub fn create_access_token_from_username(
        username: &str,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        custom_tokens::create_custom_access_token_from_username(username)
    }

    pub fn create_refresh_token(
        email: &str,
        session_id: i64,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        tokens::create_refresh_token(email, session_id)
    }

    pub fn create_refresh_token_from_username(
        username: &str,
        session_id: Option<i64>,
    ) -> Result<(String, chrono::DateTime<chrono::Utc>), String> {
        tokens::create_refresh_token_from_username(username, session_id)
    }

    pub fn validate_access_token(token: &str) -> Result<AccessTokenClaims, String> {
        custom_tokens::validate_custom_access_token(token)
    }

    pub fn validate_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
        tokens::validate_refresh_token(token)
    }

    // Re-export magic link functions for backwards compatibility
    pub fn generate_magic_token_encrypted(
        email: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(String, [u8; 44], i64), String> {
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

    pub fn get_host_url_from_request(req: &spin_sdk::http::Request) -> String {
        magic_links::get_host_url_from_request(req)
    }
}
