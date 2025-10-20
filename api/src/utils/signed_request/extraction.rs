//! Public key extraction from different authentication sources

use serde_json::Value;
use spin_sdk::http::Request;

use super::errors::SignedRequestError;
use crate::database::operations::magic_link_ops::MagicLinkOperations;
use crate::utils::jwt::utils::JwtUtils;

/// Extract pub_key from Bearer token (JWT)
///
/// Method 1: Used when Authorization header contains a JWT Bearer token
pub fn extract_pub_key_from_bearer(request: &Request) -> Result<String, SignedRequestError> {
    let auth_header = request
        .header("authorization")
        .and_then(|h| h.as_str())
        .ok_or_else(|| {
            SignedRequestError::MissingPublicKey("No Authorization header".to_string())
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        SignedRequestError::MissingPublicKey("Invalid Bearer token format".to_string())
    })?;

    let claims = JwtUtils::validate_access_token(token).map_err(|e| {
        SignedRequestError::InvalidSignature(format!("JWT validation failed: {}", e))
    })?;

    Ok(hex::encode(claims.ed25519_pub_key))
}

/// Extract Ed25519 pub_key directly from payload
///
/// Method 2: Used when payload contains ed25519_pub_key field (initial login requests)
pub fn extract_pub_key_from_payload(payload: &Value) -> Result<String, SignedRequestError> {
    payload
        .get("ed25519_pub_key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            SignedRequestError::MissingPublicKey("ed25519_pub_key not found in payload".to_string())
        })
}

/// Extract pub_key from magiclink via database lookup
///
/// Method 3: Used when payload contains magiclink token (magic link validation)
pub fn extract_pub_key_from_magiclink(payload: &Value) -> Result<String, SignedRequestError> {
    let magiclink = payload
        .get("magiclink")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            SignedRequestError::MissingPublicKey("magiclink not found in payload".to_string())
        })?;

    // Validate magiclink and extract Ed25519 pub_key from database
    let (_is_valid, _next_param, _user_id, ed25519_pub_key_bytes, _x25519_pub_key_bytes, _ui_host) =
        MagicLinkOperations::validate_and_consume_magic_link_encrypted(magiclink).map_err(|e| {
            SignedRequestError::InvalidSignature(format!("Magiclink validation failed: {}", e))
        })?;

    let ed25519_pub_key_array = ed25519_pub_key_bytes.ok_or_else(|| {
        SignedRequestError::MissingPublicKey("No Ed25519 pub_key found in magiclink data".to_string())
    })?;

    Ok(hex::encode(ed25519_pub_key_array))
}
