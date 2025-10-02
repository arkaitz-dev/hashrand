//! Validation logic for refresh token requests

use spin_sdk::http::{Method, Request, Response};

use super::super::types::{RefreshPayload, RefreshSignedRequest};
use super::utilities::{create_error_response, extract_refresh_token_from_cookies};
use crate::utils::jwt::types::RefreshTokenClaims;
use crate::utils::{JwtUtils, SignedRequestValidator};

/// Validate HTTP method (must be POST)
///
/// # Arguments
/// * `request` - HTTP request
///
/// # Returns
/// * `Result<(), Response>` - Ok if POST, Err with error response otherwise
pub fn validate_http_method(request: &Request) -> Result<(), Response> {
    if *request.method() != Method::Post {
        return Err(Response::builder()
            .status(405)
            .header("allow", "POST")
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&super::super::types::ErrorResponse {
                    error: "Method not allowed".to_string(),
                })
                .expect("Failed to serialize error"),
            )
            .build());
    }
    Ok(())
}

/// Extract and validate refresh token from cookies
///
/// # Arguments
/// * `request` - HTTP request with cookie header
///
/// # Returns
/// * `Result<RefreshTokenClaims, Response>` - Validated claims or error response
pub fn extract_and_validate_refresh_token(
    request: &Request,
) -> Result<RefreshTokenClaims, Response> {
    // Extract refresh token from cookies
    let refresh_token = match request.header("cookie") {
        Some(cookie_header) => {
            let cookie_str = cookie_header.as_str().unwrap_or("");
            extract_refresh_token_from_cookies(cookie_str)
        }
        None => None,
    };

    let refresh_token = match refresh_token {
        Some(token) => token,
        None => {
            return Err(create_error_response(401, "Refresh token not found")
                .expect("Failed to create error response"));
        }
    };

    // Validate refresh token
    let claims = match JwtUtils::validate_refresh_token(&refresh_token) {
        Ok(claims) => claims,
        Err(e) => {
            println!("❌ Refresh: Token validation failed: {}", e);
            return Err(
                create_error_response(401, &format!("Invalid refresh token: {}", e))
                    .expect("Failed to create error response"),
            );
        }
    };

    Ok(claims)
}

/// Validate SignedRequest from request body
///
/// Parses SignedRequest from body and validates Ed25519 signature
///
/// # Arguments
/// * `request` - HTTP request with SignedRequest body
/// * `pub_key_hex` - Public key hex string for signature validation
///
/// # Returns
/// * `Result<RefreshSignedRequest, Response>` - Validated SignedRequest or error response
pub fn validate_signed_request(
    request: &Request,
    pub_key_hex: &str,
) -> Result<RefreshSignedRequest, Response> {
    // Parse SignedRequest from body
    let body_bytes = request.body();
    let signed_request: RefreshSignedRequest = match serde_json::from_slice(body_bytes) {
        Ok(req) => req,
        Err(e) => {
            println!("❌ Refresh: Failed to parse SignedRequest: {}", e);
            return Err(
                create_error_response(400, "Invalid SignedRequest structure")
                    .expect("Failed to create error response"),
            );
        }
    };

    // Validate Ed25519 signature using pub_key from refresh token JWT
    if let Err(e) = SignedRequestValidator::validate_base64_payload(
        &signed_request.payload,
        &signed_request.signature,
        pub_key_hex,
    ) {
        println!("❌ Refresh: Signature validation failed: {}", e);
        return Err(
            create_error_response(401, &format!("Invalid signature: {}", e))
                .expect("Failed to create error response"),
        );
    }

    Ok(signed_request)
}

/// Parse and validate refresh payload from SignedRequest
///
/// # Arguments
/// * `signed_request` - Validated SignedRequest
///
/// # Returns
/// * `Result<RefreshPayload, Response>` - Parsed payload or error response
pub fn parse_refresh_payload(
    signed_request: &RefreshSignedRequest,
) -> Result<RefreshPayload, Response> {
    let refresh_payload: RefreshPayload =
        match SignedRequestValidator::deserialize_base64_payload(&signed_request.payload) {
            Ok(payload) => payload,
            Err(e) => {
                println!("❌ Refresh: Failed to deserialize payload: {}", e);
                return Err(create_error_response(400, "Invalid payload format")
                    .expect("Failed to create error response"));
            }
        };

    println!(
        "✅ Refresh: SignedRequest validated, new_pub_key received: {}",
        &refresh_payload.new_pub_key[..16.min(refresh_payload.new_pub_key.len())]
    );

    Ok(refresh_payload)
}
