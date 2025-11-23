//! Magic link generation handler
//!
//! Handles POST /api/login/ - Generates magic link using universal SignedRequest

use spin_sdk::http::{Request, Response};
use tracing::info;

use super::utilities::create_error_response;
use crate::utils::SignedRequestValidator;
use crate::utils::auth::types::MagicLinkPayload;
use crate::utils::auth::{MagicLinkSignedRequest, generate_magic_link_signed};

/// Handle POST /api/login/ - Generate magic link using universal SignedRequest
///
/// Parses the request body as a SignedRequest structure, validates the
/// Base64-encoded JSON payload, and delegates to the magic link generator
pub async fn handle_magic_link_generation(req: Request) -> anyhow::Result<Response> {
    use tracing::{debug, error};

    info!("🔗 Request to /api/login/ (magic link generation) endpoint");
    debug!("🔍 DEBUG: Handler started");

    // Parse request body
    debug!("🔍 DEBUG: About to read request body");
    let body_bytes = req.body();
    debug!("🔍 DEBUG: Body bytes length: {}", body_bytes.len());

    // Parse as SignedRequest structure
    debug!("🔍 DEBUG: About to parse SignedRequest structure");
    let signed_request = match parse_signed_request(body_bytes) {
        Ok(req) => {
            debug!("✅ DEBUG: SignedRequest parsed successfully");
            req
        },
        Err(response) => {
            debug!("⚠️  DEBUG: SignedRequest parsing failed, returning error response");
            return Ok(response);
        },
    };

    // Use universal SignedRequest handler
    debug!("🔍 DEBUG: About to call generate_magic_link_signed");
    let result = generate_magic_link_signed(&req, &signed_request).await;

    match &result {
        Ok(_) => debug!("✅ DEBUG: generate_magic_link_signed returned Ok"),
        Err(e) => error!("❌ DEBUG: generate_magic_link_signed returned Err: {}", e),
    }

    result
}

/// Parse and validate SignedRequest from body bytes
///
/// Returns the SignedRequest on success, or an error Response on failure
fn parse_signed_request(body_bytes: &[u8]) -> Result<MagicLinkSignedRequest, Response> {
    let signed_request = match serde_json::from_slice::<MagicLinkSignedRequest>(body_bytes) {
        Ok(req) => req,
        Err(_) => {
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
    match SignedRequestValidator::deserialize_base64_payload::<MagicLinkPayload>(
        &signed_request.payload,
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(
            create_error_response(400, "Invalid Base64 JSON payload format")
                .expect("Failed to create error response"),
        ),
    }
}
