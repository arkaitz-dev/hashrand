//! Magic link validation business logic

use spin_sdk::http::Response;
use std::collections::HashMap;

use super::types::ErrorResponse;
use crate::database::{connection::DatabaseEnvironment, operations::MagicLinkOperations};
use crate::utils::JwtUtils;

/// Validate magic link and generate JWT tokens
///
/// This function handles the business logic for magic link validation:
/// - Extracts magic link token from query parameters
/// - Validates and consumes the encrypted magic token
/// - Generates new access and refresh tokens
/// - Returns JWT response with secure HttpOnly cookie
pub fn validate_magic_link(
    query_params: HashMap<String, String>,
    env: DatabaseEnvironment,
) -> anyhow::Result<Response> {
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

    // Validate and consume encrypted magic token, get next parameter and user_id
    // Get random_hash from query params (needed for validation)
    let random_hash = query_params.get("hash").map(|s| s.as_str());

    let (is_valid, next_param, user_id_bytes) =
        match MagicLinkOperations::validate_and_consume_magic_link_encrypted(
            env.clone(),
            magic_token,
            random_hash,
        ) {
            Ok((valid, next, user_id)) => (valid, next, user_id),
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
    let _ = MagicLinkOperations::ensure_user_exists(env.clone(), &user_id_array);

    // Generate new access and refresh tokens
    let (access_token, _access_expires) = match JwtUtils::create_access_token(&username) {
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

    let (refresh_token, _) = match JwtUtils::create_refresh_token(&username, 0) {
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
        "expires_in": 180, // 3 minutes
        "user_id": username
    });

    // Add next parameter if present
    if let Some(next) = next_param {
        auth_response["next"] = serde_json::Value::String(next);
    }

    // Set refresh token as HttpOnly, Secure, SameSite cookie
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        refresh_token,
        15 * 60 // 15 minutes in seconds
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(auth_response.to_string())
        .build())
}
