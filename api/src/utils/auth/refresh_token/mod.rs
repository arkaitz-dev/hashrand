//! Refresh token business logic
//!
//! Handles token refresh with optional Ed25519 key rotation using 2/3 threshold system

mod threshold;
mod period_1_3;
mod period_2_3;
mod utilities;
mod validation;

use spin_sdk::http::{Request, Response};

use utilities::extract_hostname_from_host_header;
use validation::{
    extract_and_validate_refresh_token, parse_refresh_payload, validate_http_method,
    validate_signed_request,
};

/// Handle refresh token request and generate new access token
///
/// This function orchestrates the complete refresh token flow:
/// 1. Validate HTTP method (POST only)
/// 2. Extract hostname from Host header
/// 3. Extract and validate refresh token from cookies
/// 4. Validate SignedRequest body with Ed25519 signature
/// 5. Parse refresh payload to get new_pub_key
/// 6. Calculate if in 2/3 renewal window
/// 7. Route to PERIOD 2/3 (key rotation) or PERIOD 1/3 (simple refresh)
///
/// # Arguments
/// * `req` - HTTP POST request with refresh token cookie and SignedRequest body
///
/// # Returns
/// * `anyhow::Result<Response>` - Signed response with new access token (+ optional refresh cookie)
pub async fn handle_refresh_token(req: Request) -> anyhow::Result<Response> {
    // Step 1: Validate HTTP method
    if let Err(response) = validate_http_method(&req) {
        return Ok(response);
    }

    // Step 2: Extract hostname from Host header for cookie Domain attribute
    let domain = req
        .header("host")
        .and_then(|h| h.as_str())
        .and_then(extract_hostname_from_host_header);

    // Step 3: Extract and validate refresh token
    let claims = match extract_and_validate_refresh_token(&req) {
        Ok(claims) => claims,
        Err(response) => return Ok(response),
    };

    let username = &claims.sub;
    let ed25519_pub_key = &claims.ed25519_pub_key;
    let x25519_pub_key = &claims.x25519_pub_key;
    let ed25519_pub_key_hex = hex::encode(ed25519_pub_key);

    // Step 4: Validate SignedRequest from body (using Ed25519 for signature validation)
    let signed_request = match validate_signed_request(&req, &ed25519_pub_key_hex) {
        Ok(request) => request,
        Err(response) => return Ok(response),
    };

    // Step 5: Parse refresh payload (contains new_ed25519_pub_key and new_x25519_pub_key)
    let refresh_payload = match parse_refresh_payload(&signed_request) {
        Ok(payload) => payload,
        Err(response) => return Ok(response),
    };

    // Step 6: Calculate if we're in 2/3 renewal window
    let is_in_renewal_window = threshold::is_in_renewal_window(&claims);

    // Step 7: Route to appropriate handler
    if is_in_renewal_window {
        // PERIOD 2/3: Complete key rotation with both Ed25519 and X25519
        period_2_3::handle_key_rotation(
            username,
            &ed25519_pub_key_hex,
            &hex::encode(x25519_pub_key),
            &refresh_payload.new_ed25519_pub_key,
            &refresh_payload.new_x25519_pub_key,
            domain,
        )
    } else {
        // PERIOD 1/3: Simple token refresh (no rotation)
        period_1_3::handle_no_rotation(username, ed25519_pub_key, x25519_pub_key)
    }
}
