//! Logout handler with Ed25519 signature validation
//!
//! Handles DELETE /api/login/ - Clear refresh token cookie (logout)
//! SECURITY: Validates Ed25519 signature to prevent unauthorized logout (DoS protection)

use spin_sdk::http::{Request, Response};

use super::utilities::{
    create_error_response, extract_refresh_token_from_cookies, parse_query_params,
};
use crate::utils::jwt::types::RefreshTokenClaims;
use crate::utils::{CryptoMaterial, SignedRequestValidator};

/// Handle DELETE /api/login/ - Clear refresh token cookie (logout)
///
/// Validates Ed25519 signature to prevent unauthorized logout
/// Returns a SignedResponse with an expired cookie to clear the refresh token
pub fn handle_logout(req: Request) -> anyhow::Result<Response> {
    // Extract and validate refresh token
    let refresh_token = match extract_and_validate_token(&req) {
        Ok(token) => token,
        Err(response) => return Ok(response),
    };

    // Validate JWT and extract pub_key
    let claims = match validate_jwt_and_extract_claims(&refresh_token) {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let pub_key_hex = hex::encode(claims.pub_key);

    // Validate Ed25519 signature from query parameters
    if let Err(response) = validate_logout_signature(&req, &pub_key_hex) {
        return Ok(response);
    }

    println!("✅ Logout: Signature validated, clearing refresh token");

    // Create and return logout response with expired cookie
    create_logout_response(&claims.sub, &pub_key_hex)
}

/// Extract refresh token from cookie header and validate presence
fn extract_and_validate_token(req: &Request) -> Result<String, Response> {
    let refresh_token = match req.header("cookie") {
        Some(cookie_header) => {
            let cookie_str = cookie_header.as_str().unwrap_or("");
            extract_refresh_token_from_cookies(cookie_str)
        }
        None => None,
    };

    match refresh_token {
        Some(token) => Ok(token),
        None => Err(create_error_response(401, "Not authenticated")
            .expect("Failed to create error response")),
    }
}

/// Validate JWT refresh token and extract claims
fn validate_jwt_and_extract_claims(refresh_token: &str) -> Result<RefreshTokenClaims, Response> {
    match crate::utils::JwtUtils::validate_refresh_token(refresh_token) {
        Ok(claims) => Ok(claims),
        Err(e) => {
            println!("❌ Logout: JWT validation failed: {}", e);
            Err(create_error_response(401, "Invalid authentication")
                .expect("Failed to create error response"))
        }
    }
}

/// Validate Ed25519 signature from query parameters
fn validate_logout_signature(req: &Request, pub_key_hex: &str) -> Result<(), Response> {
    let uri_str = req.uri().to_string();
    let mut query_params = parse_query_params(&uri_str);

    // Validate signature (DELETE requests have empty params except signature)
    if let Err(e) = SignedRequestValidator::validate_query_params(&mut query_params, pub_key_hex) {
        println!("❌ Logout: Signature validation failed: {}", e);
        return Err(create_error_response(401, "Invalid signature")
            .expect("Failed to create error response"));
    }

    Ok(())
}

/// Create logout response with SignedResponse and expired cookie
fn create_logout_response(user_id: &str, pub_key_hex: &str) -> anyhow::Result<Response> {
    // Prepare crypto material for SignedResponse
    let crypto_material = CryptoMaterial {
        user_id: user_id.as_bytes().to_vec(),
        pub_key_hex: pub_key_hex.to_string(),
    };

    // Create logout success payload
    #[derive(serde::Serialize)]
    struct LogoutResponse {
        message: String,
    }

    let logout_payload = LogoutResponse {
        message: "Logged out successfully".to_string(),
    };

    // Generate SignedResponse
    let signed_response =
        match crate::utils::create_signed_endpoint_response(logout_payload, &crypto_material) {
            Ok(response) => response,
            Err(e) => {
                println!("❌ Failed to create SignedResponse for logout: {}", e);
                return create_error_response(500, "Failed to generate signed response");
            }
        };

    // Create expired cookie to clear the refresh token
    let expired_cookie = "refresh_token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/";

    // Add cookie to SignedResponse
    Ok(Response::builder()
        .status(*signed_response.status())
        .header("content-type", "application/json")
        .header("set-cookie", expired_cookie)
        .body(signed_response.body().to_vec())
        .build())
}
