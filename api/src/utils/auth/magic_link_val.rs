//! Magic link validation business logic

use hex;
use spin_sdk::http::Response;
use std::collections::HashMap;

use super::types::{ErrorResponse, MagicLinkValidationRequest};
use crate::database::operations::MagicLinkOperations;
use crate::utils::{
    JwtUtils,
    ed25519::{Ed25519Utils, SignatureVerificationResult},
};

/// Validate magic link and generate JWT tokens
///
/// This function handles the business logic for magic link validation:
/// - Extracts magic link token from query parameters
/// - Validates and consumes the encrypted magic token
/// - Generates new access and refresh tokens
/// - Returns JWT response with secure HttpOnly cookie
pub fn validate_magic_link(query_params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Get magic link from query parameters
    let magic_token = match query_params.get("magiclink") {
        Some(token) if !token.is_empty() => token,
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Missing magiclink parameter".to_string(),
                })?)
                .build());
        }
    };

    println!("Magic token received: '{}'", magic_token);

    // Validate and consume encrypted magic token, extract next parameter, user_id, and Ed25519 pub_key
    let (is_valid, next_param, user_id_bytes, pub_key_bytes) =
        match MagicLinkOperations::validate_and_consume_magic_link_encrypted(magic_token) {
            Ok((valid, next, user_id, pub_key)) => (valid, next, user_id, pub_key),
            Err(error) => {
                let error_msg = error.to_string();
                println!("Magic token validation error: {}", error_msg);

                // Categorize error types for appropriate HTTP status codes
                if error_msg.contains("Invalid Base58") || error_msg.contains("must be 32 bytes") {
                    // Client validation error - malformed token format
                    return Ok(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Invalid magic link token format".to_string(),
                        })?)
                        .build());
                } else if error_msg.contains("ChaCha20-Poly1305 decryption error") {
                    // Client validation error - corrupted or tampered token
                    return Ok(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Invalid or corrupted magic link".to_string(),
                        })?)
                        .build());
                } else if error_msg.contains("Missing MLINK_CONTENT")
                    || error_msg.contains("Invalid MLINK_CONTENT")
                    || error_msg.contains("Argon2 params error")
                    || error_msg.contains("Invalid nonce key")
                    || error_msg.contains("Invalid cipher key")
                {
                    // System configuration/crypto errors - return 500 Internal Server Error
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Server configuration error".to_string(),
                        })?)
                        .build());
                } else {
                    // Database connection or other system errors - return 500 Internal Server Error
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Database error".to_string(),
                        })?)
                        .build());
                }
            }
        };

    if !is_valid {
        println!("Magic token validation failed or expired");
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Invalid or expired magic link".to_string(),
            })?)
            .build());
    }

    // Extract user_id from the decrypted magic link
    let user_id_array = match user_id_bytes {
        Some(user_id) => user_id,
        None => {
            println!("No user_id returned from magic link validation");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid magic link data".to_string(),
                })?)
                .build());
        }
    };

    // Convert user_id to Base58 username
    let username = JwtUtils::user_id_to_username(&user_id_array);

    println!("User {} authenticated successfully", username);

    // Ensure user exists in users table
    let _ = MagicLinkOperations::ensure_user_exists(&user_id_array);

    // Use Ed25519 public key extracted from the encrypted magic link payload
    let pub_key_array = match pub_key_bytes {
        Some(key) => key,
        None => {
            println!("No Ed25519 public key found in magic link payload");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid magic link: missing Ed25519 public key".to_string(),
                })?)
                .build());
        }
    };

    // Generate new access and refresh tokens with extracted Ed25519 public key
    let (access_token, _access_expires) =
        match JwtUtils::create_access_token_from_username(&username, &pub_key_array) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!("Failed to create access token: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Failed to create access token".to_string(),
                    })?)
                    .build());
            }
        };

    let (refresh_token, _) = match JwtUtils::create_refresh_token_from_username(&username, None) {
        Ok((token, exp)) => (token, exp),
        Err(e) => {
            println!("Failed to create refresh token: {}", e);
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Failed to create refresh token".to_string(),
                })?)
                .build());
        }
    };

    // Create response with access token, user_id, next parameter, and secure refresh token cookie
    let mut auth_response = serde_json::json!({
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": (_access_expires.timestamp() - chrono::Utc::now().timestamp()),
        "user_id": username
    });

    // Add next parameter if present
    if let Some(next) = next_param {
        auth_response["next"] = serde_json::Value::String(next);
    }

    // Set refresh token as HttpOnly, Secure, SameSite cookie with correct duration
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        refresh_token,
        refresh_duration_minutes * 60 // Convert minutes to seconds
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(auth_response.to_string())
        .build())
}

/// Validate magic link with Ed25519 signature verification (secure POST endpoint)
///
/// This function handles secure magic link validation with cryptographic signature verification:
/// - Parses JSON request body containing magic link token and Ed25519 signature
/// - Validates and consumes the encrypted magic token (same as validate_magic_link)
/// - Extracts Ed25519 public key from decrypted magic link payload
/// - Verifies the provided signature against the magic link token using extracted public key
/// - Only issues JWT tokens if signature verification succeeds
/// - Returns JWT response with secure HttpOnly cookie (same as validate_magic_link)
pub fn validate_magic_link_secure(request_body: &[u8]) -> anyhow::Result<Response> {
    // Parse JSON request body
    let validation_request: MagicLinkValidationRequest = match serde_json::from_slice(request_body)
    {
        Ok(req) => req,
        Err(e) => {
            println!("Failed to parse magic link validation request: {}", e);
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid JSON request body".to_string(),
                })?)
                .build());
        }
    };

    let magic_token = &validation_request.magiclink;
    let signature_hex = &validation_request.signature;

    println!(
        "Magic token received for secure validation: '{}'",
        magic_token
    );
    println!(
        "🔍 DEBUG Ed25519: Received signature for validation: {}",
        signature_hex
    );

    // Validate and consume encrypted magic token, extract next parameter, user_id, and Ed25519 pub_key
    let (is_valid, next_param, user_id_bytes, pub_key_bytes) =
        match MagicLinkOperations::validate_and_consume_magic_link_encrypted(magic_token) {
            Ok((valid, next, user_id, pub_key)) => (valid, next, user_id, pub_key),
            Err(error) => {
                let error_msg = error.to_string();
                println!("Magic token validation error: {}", error_msg);

                // Categorize error types for appropriate HTTP status codes
                if error_msg.contains("Invalid Base58") || error_msg.contains("must be 32 bytes") {
                    // Client validation error - malformed token format
                    return Ok(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Invalid magic link token format".to_string(),
                        })?)
                        .build());
                } else if error_msg.contains("ChaCha20-Poly1305 decryption error") {
                    // Client validation error - corrupted or tampered token
                    return Ok(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Invalid or corrupted magic link".to_string(),
                        })?)
                        .build());
                } else if error_msg.contains("Missing MLINK_CONTENT")
                    || error_msg.contains("Invalid MLINK_CONTENT")
                    || error_msg.contains("Argon2 params error")
                    || error_msg.contains("Invalid nonce key")
                    || error_msg.contains("Invalid cipher key")
                {
                    // System configuration/crypto errors - return 500 Internal Server Error
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Server configuration error".to_string(),
                        })?)
                        .build());
                } else {
                    // Database connection or other system errors - return 500 Internal Server Error
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Database error".to_string(),
                        })?)
                        .build());
                }
            }
        };

    if !is_valid {
        println!("Magic token validation failed or expired");
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Invalid or expired magic link".to_string(),
            })?)
            .build());
    }

    // Extract user_id from the decrypted magic link
    let user_id_array = match user_id_bytes {
        Some(user_id) => user_id,
        None => {
            println!("No user_id returned from magic link validation");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid magic link data".to_string(),
                })?)
                .build());
        }
    };

    // Extract Ed25519 public key from decrypted magic link payload
    let pub_key_array = match pub_key_bytes {
        Some(key) => key,
        None => {
            println!("No Ed25519 public key found in magic link payload");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid magic link: missing Ed25519 public key".to_string(),
                })?)
                .build());
        }
    };

    // 🔐 CRITICAL SECURITY STEP: Verify Ed25519 signature before issuing credentials
    println!(
        "🔍 DEBUG Ed25519: Verifying signature for magic link token: {}",
        magic_token
    );

    // Convert pub_key_array to hex string for verification
    let pub_key_hex = hex::encode(pub_key_array);

    // The message that was signed is the magic link token itself
    let message_to_verify = magic_token.as_bytes();

    let signature_verification_result =
        Ed25519Utils::verify_signature(message_to_verify, signature_hex, &pub_key_hex);

    match signature_verification_result {
        SignatureVerificationResult::Valid => {
            println!("✅ Ed25519 signature verification successful");
        }
        SignatureVerificationResult::Invalid => {
            println!("❌ Ed25519 signature verification failed - invalid signature");
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Ed25519 signature verification failed".to_string(),
                })?)
                .build());
        }
        SignatureVerificationResult::MalformedPublicKey => {
            println!("❌ Ed25519 signature verification error: malformed public key");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Ed25519 malformed public key".to_string(),
                })?)
                .build());
        }
        SignatureVerificationResult::MalformedSignature => {
            println!("❌ Ed25519 signature verification error: malformed signature");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Ed25519 malformed signature".to_string(),
                })?)
                .build());
        }
        SignatureVerificationResult::MalformedMessage => {
            println!("❌ Ed25519 signature verification error: malformed message");
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Ed25519 malformed message".to_string(),
                })?)
                .build());
        }
    }

    // Convert user_id to Base58 username
    let username = JwtUtils::user_id_to_username(&user_id_array);

    println!(
        "User {} authenticated successfully with Ed25519 verification",
        username
    );

    // Ensure user exists in users table
    let _ = MagicLinkOperations::ensure_user_exists(&user_id_array);

    // Generate new access and refresh tokens with extracted Ed25519 public key
    let (access_token, _access_expires) =
        match JwtUtils::create_access_token_from_username(&username, &pub_key_array) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!("Failed to create access token: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Failed to create access token".to_string(),
                    })?)
                    .build());
            }
        };

    let (refresh_token, _) = match JwtUtils::create_refresh_token_from_username(&username, None) {
        Ok((token, exp)) => (token, exp),
        Err(e) => {
            println!("Failed to create refresh token: {}", e);
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Failed to create refresh token".to_string(),
                })?)
                .build());
        }
    };

    // Create response with access token, user_id, next parameter, and secure refresh token cookie
    let mut auth_response = serde_json::json!({
        "access_token": access_token,
        "token_type": "Bearer",
        "expires_in": (_access_expires.timestamp() - chrono::Utc::now().timestamp()),
        "user_id": username
    });

    // Add next parameter if present
    if let Some(next) = next_param {
        auth_response["next"] = serde_json::Value::String(next);
    }

    // Set refresh token as HttpOnly, Secure, SameSite cookie with correct duration
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        refresh_token,
        refresh_duration_minutes * 60 // Convert minutes to seconds
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(auth_response.to_string())
        .build())
}
