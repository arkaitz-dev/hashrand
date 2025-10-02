//! Magic link generation handler
//!
//! Handles POST /api/login/ - Generates magic link using universal SignedRequest

use spin_sdk::http::{Request, Response};

use super::utilities::create_error_response;
use crate::utils::SignedRequestValidator;
use crate::utils::auth::types::MagicLinkPayload;
use crate::utils::auth::{MagicLinkSignedRequest, generate_magic_link_signed};

/// Handle POST /api/login/ - Generate magic link using universal SignedRequest
///
/// Parses the request body as a SignedRequest structure, validates the
/// Base64-encoded JSON payload, and delegates to the magic link generator
pub async fn handle_magic_link_generation(req: Request) -> anyhow::Result<Response> {
    // Parse request body
    let body_bytes = req.body();
    println!(
        "DEBUG: Request body bytes: {:?}",
        std::str::from_utf8(body_bytes)
    );

    // Parse as SignedRequest structure
    let signed_request = match parse_signed_request(body_bytes) {
        Ok(req) => req,
        Err(response) => return Ok(response),
    };

    // Use universal SignedRequest handler
    generate_magic_link_signed(&req, &signed_request).await
}

/// Parse and validate SignedRequest from body bytes
///
/// Returns the SignedRequest on success, or an error Response on failure
fn parse_signed_request(body_bytes: &[u8]) -> Result<MagicLinkSignedRequest, Response> {
    let signed_request = match serde_json::from_slice::<MagicLinkSignedRequest>(body_bytes) {
        Ok(req) => {
            println!("🔐 DEBUG: Received SignedRequest structure with Base64-encoded JSON payload");
            req
        }
        Err(e) => {
            println!("DEBUG: JSON parse error for SignedRequest: {}", e);
            return Err(create_error_response(
                400,
                "Invalid JSON body - must be SignedRequest structure",
            )
            .expect("Failed to create error response"));
        }
    };

    // Deserialize and validate Base64-encoded JSON payload
    validate_payload_structure(&signed_request)?;

    Ok(signed_request)
}

/// Validate the deserialized payload structure
///
/// Ensures the Base64-encoded JSON payload can be properly deserialized
fn validate_payload_structure(signed_request: &MagicLinkSignedRequest) -> Result<(), Response> {
    let deserialized_payload: MagicLinkPayload =
        match SignedRequestValidator::deserialize_base64_payload(&signed_request.payload) {
            Ok(payload) => payload,
            Err(e) => {
                println!("❌ DEBUG: Failed to deserialize Base64 payload: {}", e);
                return Err(
                    create_error_response(400, "Invalid Base64 JSON payload format")
                        .expect("Failed to create error response"),
                );
            }
        };

    println!(
        "DEBUG: Deserialized Payload - Email: {}, UI Host: {:?}, Email Lang: {:?}",
        deserialized_payload.email, deserialized_payload.ui_host, deserialized_payload.email_lang
    );

    Ok(())
}
