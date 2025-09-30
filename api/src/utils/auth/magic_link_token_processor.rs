//! Magic Link Token Processor - Token Validation Operations
//!
//! Single Responsibility: Handle magic link token validation, decryption and data extraction
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;

use super::types::ErrorResponse;
use crate::database::operations::MagicLinkOperations;

/// Validation result containing extracted data from magic link token
pub struct TokenValidationResult {
    pub next_param: Option<String>,
    pub user_id_bytes: [u8; 16],
    pub pub_key_bytes: [u8; 32],
    pub ui_host: Option<String>,
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
    // Validate and consume encrypted magic token, extract next parameter, user_id, Ed25519 pub_key, and ui_host
    let (is_valid, next_param, user_id_bytes, pub_key_bytes, ui_host) =
        match MagicLinkOperations::validate_and_consume_magic_link_encrypted(magic_token) {
            Ok((valid, next, user_id, pub_key, ui_host)) => (valid, next, user_id, pub_key, ui_host),
            Err(error) => {
                return Err(categorize_token_validation_error(error.into()));
            }
        };

    // Check if token validation passed
    if !is_valid {
        println!("❌ Magic token validation failed or expired");
        return Err(create_token_invalid_response());
    }

    // Extract and validate user_id
    let user_id_array = match user_id_bytes {
        Some(user_id) => user_id,
        None => {
            println!("❌ No user_id returned from magic link validation");
            return Err(create_missing_user_id_response());
        }
    };

    // Extract and validate Ed25519 public key
    let pub_key_array = match pub_key_bytes {
        Some(key) => key,
        None => {
            println!("❌ No Ed25519 public key found in magic link payload");
            return Err(create_missing_public_key_response());
        }
    };

    println!("✅ Magic token validation and data extraction successful");

    // Log ui_host extraction
    if let Some(ref host) = ui_host {
        println!("🔒 [SECURITY] ui_host extracted for cookie Domain: '{}'", host);
    } else {
        println!("⚠️ [COMPAT] No ui_host in magic link (old format) - will need fallback for Domain");
    }

    Ok(TokenValidationResult {
        next_param,
        user_id_bytes: user_id_array,
        pub_key_bytes: pub_key_array,
        ui_host,
    })
}

/// Categorize token validation errors and return appropriate HTTP response
fn categorize_token_validation_error(error: anyhow::Error) -> Response {
    let error_msg = error.to_string();
    println!("❌ Magic token validation error: {}", error_msg);

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
fn create_missing_public_key_response() -> Response {
    create_error_response(400, "Invalid magic link: missing Ed25519 public key")
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
