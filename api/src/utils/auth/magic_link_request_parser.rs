//! Magic Link Request Parser - Request Parsing Operations
//!
//! Single Responsibility: Handle parsing and initial validation of SignedRequest structures
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;

use super::types::{ErrorResponse, MagicLinkValidationRequest, MagicLinkValidationPayload};
use crate::utils::SignedRequestValidator;

/// Parse and validate magic link validation request from JSON body
///
/// # Arguments
/// * `request_body` - Raw HTTP request body bytes
///
/// # Returns
/// * `Result<MagicLinkValidationRequest, Response>` - Parsed request or error response
pub fn parse_validation_request(request_body: &[u8]) -> Result<MagicLinkValidationRequest, Response> {
    match serde_json::from_slice(request_body) {
        Ok(signed_request) => {
            println!("✅ Successfully parsed SignedRequest magic link validation");
            Ok(signed_request)
        }
        Err(e) => {
            println!(
                "❌ Failed to parse signed magic link validation request: {}",
                e
            );

            let error_response = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: "Invalid SignedRequest JSON structure".to_string(),
                    })
                    .unwrap_or_default(),
                )
                .build();

            Err(error_response)
        }
    }
}

/// Extract magic token and signature from validated request
/// CORRECTED: Deserializes Base64-encoded JSON payload to access fields
///
/// # Arguments
/// * `signed_request` - Validated magic link validation request
///
/// # Returns
/// * `Result<(String, String), String>` - Tuple of (magic_token, signature_hex) or error
pub fn extract_request_data(signed_request: &MagicLinkValidationRequest) -> Result<(String, String), String> {
    // CORRECTED: Deserialize Base64-encoded JSON payload to access magiclink field
    let payload: MagicLinkValidationPayload = SignedRequestValidator::deserialize_base64_payload(&signed_request.payload)
        .map_err(|e| format!("Failed to deserialize Base64 payload: {}", e))?;

    let magic_token = payload.magiclink.clone();
    let signature_hex = signed_request.signature.clone();

    println!(
        "🔍 DEBUG: Magic token received for secure validation: '{}'",
        magic_token
    );
    println!(
        "🔍 DEBUG Ed25519: Received signature for validation: {}",
        signature_hex
    );

    Ok((magic_token, signature_hex))
}