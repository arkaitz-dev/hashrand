//! Magic Link Token Processor - Token Validation Operations
//!
//! Single Responsibility: Handle magic link token validation, decryption and data extraction
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;
use tracing::{debug, error, warn};

use super::types::ErrorResponse;
use crate::database::operations::MagicLinkOperations;

/// Validation result containing extracted data from magic link token
pub struct TokenValidationResult {
    pub next_param: Option<String>,
    pub user_id_bytes: [u8; 16],
    pub ed25519_pub_key_bytes: [u8; 32],
    pub x25519_pub_key_bytes: [u8; 32],
    pub ui_host: Option<String>,
    pub encrypted_privkey_context: String,
}

/// Validate and decrypt magic link token, extracting embedded data
///
/// # Arguments
/// * `magic_token` - Base58-encoded encrypted magic link token
///
/// # Returns
/// * `Result<TokenValidationResult, Response>` - Extracted data or error response
pub fn validate_and_extract_token_data(
    magic_token: &str,
) -> Result<TokenValidationResult, Response> {
    // Validate and consume encrypted magic token, extract next parameter, user_id, Ed25519 and X25519 pub_keys, ui_host, and privkey_context
    let (
        is_valid,
        next_param,
        user_id_bytes,
        ed25519_pub_key_bytes,
        x25519_pub_key_bytes,
        ui_host,
        privkey_context_decrypted,
    ) = match MagicLinkOperations::validate_and_consume_magic_link_encrypted(magic_token) {
        Ok((valid, next, user_id, ed25519_pub_key, x25519_pub_key, ui_host, privkey_context)) => (
            valid,
            next,
            user_id,
            ed25519_pub_key,
            x25519_pub_key,
            ui_host,
            privkey_context,
        ),
        Err(error) => {
            return Err(categorize_token_validation_error(error.into()));
        }
    };

    // Check if token validation passed
    if !is_valid {
        error!("❌ Magic token validation failed or expired");
        return Err(create_token_invalid_response());
    }

    // Extract and validate user_id
    let user_id_array = match user_id_bytes {
        Some(user_id) => user_id,
        None => {
            error!("❌ No user_id returned from magic link validation");
            return Err(create_missing_user_id_response());
        }
    };

    // Extract and validate Ed25519 public key
    let ed25519_pub_key_array = match ed25519_pub_key_bytes {
        Some(key) => key,
        None => {
            error!("❌ No Ed25519 public key found in magic link payload");
            return Err(create_missing_public_key_response("Ed25519"));
        }
    };

    // Extract and validate X25519 public key
    let x25519_pub_key_array = match x25519_pub_key_bytes {
        Some(key) => key,
        None => {
            error!("❌ No X25519 public key found in magic link payload");
            return Err(create_missing_public_key_response("X25519"));
        }
    };

    debug!("✅ Magic token validation and data extraction successful");

    // Log ui_host extraction
    if let Some(ref host) = ui_host {
        debug!(
            "🔒 [SECURITY] ui_host extracted for cookie Domain: '{}'",
            host
        );
    } else {
        warn!("⚠️ [COMPAT] No ui_host in magic link (old format) - will need fallback for Domain");
    }

    // Encrypt privkey_context with X25519 for client
    let encrypted_privkey_context = encrypt_privkey_context_for_client(
        &privkey_context_decrypted,
        &user_id_array,
        &x25519_pub_key_array,
    )
    .map_err(|e| {
        error!("Failed to encrypt privkey_context with X25519: {}", e);
        create_error_response(500, "Failed to encrypt private key context")
    })?;

    Ok(TokenValidationResult {
        next_param,
        user_id_bytes: user_id_array,
        ed25519_pub_key_bytes: ed25519_pub_key_array,
        x25519_pub_key_bytes: x25519_pub_key_array,
        ui_host,
        encrypted_privkey_context,
    })
}

/// Categorize token validation errors and return appropriate HTTP response
fn categorize_token_validation_error(error: anyhow::Error) -> Response {
    let error_msg = error.to_string();
    error!("❌ Magic token validation error: {}", error_msg);

    // Client validation errors (400 Bad Request)
    if error_msg.contains("Invalid Base58") || error_msg.contains("must be 32 bytes") {
        return create_error_response(400, "Invalid magic link token format");
    }

    if error_msg.contains("ChaCha20-Poly1305 decryption error") {
        return create_error_response(400, "Invalid or corrupted magic link");
    }

    // Server configuration errors (500 Internal Server Error)
    if error_msg.contains("Missing MLINK_CONTENT")
        || error_msg.contains("Invalid MLINK_CONTENT")
        || error_msg.contains("Argon2 params error")
        || error_msg.contains("Invalid nonce key")
        || error_msg.contains("Invalid cipher key")
    {
        return create_error_response(500, "Server configuration error");
    }

    // Default: Database or system errors (500 Internal Server Error)
    create_error_response(500, "Database error")
}

/// Create error response for invalid/expired token
fn create_token_invalid_response() -> Response {
    create_error_response(400, "Invalid or expired magic link")
}

/// Create error response for missing user ID
fn create_missing_user_id_response() -> Response {
    create_error_response(400, "Invalid magic link data")
}

/// Create error response for missing public key
fn create_missing_public_key_response(key_type: &str) -> Response {
    create_error_response(
        400,
        &format!("Invalid magic link: missing {} public key", key_type),
    )
}

/// Create standardized error response
fn create_error_response(status: u16, error_message: &str) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ErrorResponse {
                error: error_message.to_string(),
            })
            .unwrap_or_default(),
        )
        .build()
}

/// Encrypt private key context for client using X25519 ECDH
///
/// # Arguments
/// * `privkey_context` - 64-byte decrypted private key context
/// * `user_id` - 16-byte user identifier
/// * `client_x25519_pub_key` - Client's X25519 public key (32 bytes)
///
/// # Returns
/// * `Result<String, String>` - Base64-encoded encrypted context or error
fn encrypt_privkey_context_for_client(
    privkey_context: &[u8; 64],
    user_id: &[u8; 16],
    client_x25519_pub_key: &[u8; 32],
) -> Result<String, String> {
    use crate::utils::crypto::{encrypt_with_ecdh, get_backend_x25519_private_key};
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
    use x25519_dalek::PublicKey as X25519PublicKey;

    // Convert client's X25519 public key to hex for backend key derivation
    let client_x25519_hex = hex::encode(client_x25519_pub_key);

    // Get backend's per-user X25519 private key
    let backend_x25519_private = get_backend_x25519_private_key(user_id, &client_x25519_hex)
        .map_err(|e| format!("Failed to derive backend X25519 private key: {}", e))?;

    // Convert client's X25519 public key bytes to X25519PublicKey type
    let client_x25519_public = X25519PublicKey::from(*client_x25519_pub_key);

    // Encrypt privkey_context with ECDH
    let encrypted = encrypt_with_ecdh(
        privkey_context,
        &backend_x25519_private,
        &client_x25519_public,
    )
    .map_err(|e| format!("ECDH encryption failed: {}", e))?;

    // Encode to base64
    Ok(BASE64.encode(&encrypted))
}
