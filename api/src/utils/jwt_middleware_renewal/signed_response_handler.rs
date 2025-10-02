//! Signed response handler - Add tokens to signed responses

use serde_json::Value;
use spin_sdk::http::Response;

use super::super::jwt_middleware_types::RenewedTokens;
use super::non_signed_handler::add_tokens_to_headers;
use super::response_utilities::create_refresh_cookie;
use crate::utils::{SignedRequestValidator, SignedResponse, SignedResponseGenerator};

/// Add renewed tokens to signed response
///
/// Parses Base64-encoded JSON payload, adds access token, re-signs, and adds refresh cookie
///
/// # Arguments
/// * `response` - Original response
/// * `renewed_tokens` - New tokens to add
/// * `body_str` - Response body as string
///
/// # Returns
/// * `Response` - Response with modified signed payload and refresh cookie
pub fn add_tokens_to_signed_response(
    response: Response,
    renewed_tokens: RenewedTokens,
    body_str: &str,
) -> Response {
    // Parse existing signed response
    let signed_response: SignedResponse = match serde_json::from_str(body_str) {
        Ok(resp) => resp,
        Err(_) => {
            println!("🔍 DEBUG: Failed to parse signed response, falling back to headers");
            return add_tokens_to_headers(response, renewed_tokens);
        }
    };

    // Decode Base64 payload
    let json_string = match SignedRequestValidator::decode_payload_base64(&signed_response.payload)
    {
        Ok(json) => json,
        Err(e) => {
            println!("❌ DEBUG: Failed to decode Base64 payload: {}", e);
            return add_tokens_to_headers(response, renewed_tokens);
        }
    };

    // Parse JSON payload
    let mut enhanced_payload: Value = match serde_json::from_str(&json_string) {
        Ok(payload) => payload,
        Err(e) => {
            println!("❌ DEBUG: Failed to parse JSON payload: {}", e);
            return add_tokens_to_headers(response, renewed_tokens);
        }
    };

    // Add access token to payload
    if let Value::Object(ref mut map) = enhanced_payload {
        map.insert(
            "access_token".to_string(),
            Value::String(renewed_tokens.access_token.clone()),
        );
        // Reactive auth: no expires_in needed - frontend responds to 401s
    } else {
        println!("🔍 DEBUG: Payload is not an object, cannot add access_token");
        return add_tokens_to_headers(response, renewed_tokens);
    }

    // Generate new signed response
    let new_signed_response = match SignedResponseGenerator::create_signed_response(
        enhanced_payload,
        &renewed_tokens.user_id,
        &renewed_tokens.pub_key_hex,
    ) {
        Ok(resp) => resp,
        Err(e) => {
            println!(
                "🔍 DEBUG: Failed to create signed response: {}, falling back to headers",
                e
            );
            return add_tokens_to_headers(response, renewed_tokens);
        }
    };

    // Serialize new response
    let new_body = match serde_json::to_string(&new_signed_response) {
        Ok(body) => body,
        Err(_) => {
            println!("🔍 DEBUG: Failed to serialize signed response, falling back to headers");
            return add_tokens_to_headers(response, renewed_tokens);
        }
    };

    // Build response with copied headers (skip content-length as body changed)
    let mut binding = Response::builder();
    let mut builder = binding.status(*response.status());

    // Copy existing headers (except content-length which will change)
    for (name, value) in response.headers() {
        if name.to_lowercase() != "content-length" {
            builder = builder.header(name, value.as_str().unwrap_or(""));
        }
    }

    // Add refresh token cookie if provided
    if !renewed_tokens.refresh_token.is_empty() {
        let refresh_cookie = create_refresh_cookie(&renewed_tokens.refresh_token);
        builder = builder.header("set-cookie", &refresh_cookie);
    }

    builder.body(new_body).build()
}
