//! Magic Link JWT Generator - JWT Token Generation Operations
//!
//! Single Responsibility: Handle JWT access and refresh token generation
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use chrono::{DateTime, Utc};
use spin_sdk::http::Response;
use tracing::{debug, error};

use super::types::ErrorResponse;
use crate::database::operations::MagicLinkOperations;
use crate::utils::JwtUtils;

/// JWT token generation result
pub struct JwtTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub username: String,
}

/// Generate JWT access and refresh tokens for authenticated user
///
/// # Arguments
/// * `user_id_bytes` - User ID bytes from magic link token
/// * `pub_key_bytes` - Ed25519 public key bytes for JWT claims
///
/// # Returns
/// * `Result<JwtTokens, Response>` - Generated tokens or error response
pub fn generate_jwt_tokens(
    user_id_bytes: &[u8; 16],
    pub_key_bytes: &[u8; 32],
) -> Result<JwtTokens, Response> {
    // Convert user_id to Base58 username
    let username = JwtUtils::user_id_to_username(user_id_bytes);

    //     "🔐 User {} authenticated successfully with Ed25519 verification",
    //     username
    // );
    debug!(
        "🔐 User {} authenticated successfully with Ed25519 verification",
        username
    );

    // Ensure user exists in users table
    let _ = MagicLinkOperations::ensure_user_exists(user_id_bytes);

    // Generate access token with Ed25519 public key
    let (access_token, _access_expires) = create_access_token(&username, pub_key_bytes)?;

    // Generate refresh token with Ed25519 public key for /api/refresh signature validation
    let refresh_token = create_refresh_token(&username, pub_key_bytes)?;

    debug!("✅ JWT tokens generated successfully for user {}", username);

    Ok(JwtTokens {
        access_token,
        refresh_token,
        username,
    })
}

/// Create access token with Ed25519 public key embedded
fn create_access_token(
    username: &str,
    pub_key_bytes: &[u8; 32],
) -> Result<(String, DateTime<Utc>), Response> {
    match JwtUtils::create_access_token_from_username(username, pub_key_bytes) {
        Ok((token, expires)) => {
            debug!("✅ Access token created successfully");
            Ok((token, expires))
        }
        Err(e) => {
            error!("❌ Failed to create access token: {}", e);
            Err(create_jwt_error_response("Failed to create access token"))
        }
    }
}

/// Create refresh token for session persistence with Ed25519 public key
fn create_refresh_token(username: &str, pub_key_bytes: &[u8; 32]) -> Result<String, Response> {
    match JwtUtils::create_refresh_token_from_username(username, pub_key_bytes) {
        Ok((token, _expires)) => {
            debug!("✅ Refresh token created successfully");
            Ok(token)
        }
        Err(e) => {
            error!("❌ Failed to create refresh token: {}", e);
            Err(create_jwt_error_response("Failed to create refresh token"))
        }
    }
}

/// Create standardized error response for JWT generation failures
fn create_jwt_error_response(error_message: &str) -> Response {
    Response::builder()
        .status(500)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ErrorResponse {
                error: error_message.to_string(),
            })
            .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
        )
        .build()
}
