//! Magic link generation business logic - Main coordination
//!
//! Orchestrates magic link generation using specialized modules following
//! SOLID and DRY principles to eliminate code duplication.

use spin_sdk::http::{Request, Response};

use super::magic_link_email_delivery::MagicLinkEmailDelivery;
use super::magic_link_request_validation::MagicLinkRequestValidation;
use super::magic_link_response_builder::MagicLinkResponseBuilder;
use super::magic_link_token_gen::MagicLinkTokenGeneration;
use super::types::{MagicLinkRequest, MagicLinkSignedRequest};
use crate::database::operations::MagicLinkOperations;
use crate::utils::jwt::crypto::derive_user_id;
use crate::utils::signed_response::SignedResponseGenerator;
use serde_json::json;

/// Generate and send magic link for authentication
///
/// Orchestrates the complete magic link generation flow using specialized modules:
/// - Request validation (rate limiting, email, Ed25519 signature)
/// - Token generation and URL creation
/// - Database storage and email delivery
pub async fn generate_magic_link(
    req: &Request,
    magic_request: &MagicLinkRequest,
) -> anyhow::Result<Response> {
    // Step 1: Validate request (rate limiting, email, Ed25519 signature)
    if let Err(response) = MagicLinkRequestValidation::check_rate_limiting(req) {
        return Ok(response);
    }

    if let Err(response) = MagicLinkRequestValidation::validate_email_format(&magic_request.email) {
        return Ok(response);
    }

    if let Err(response) = MagicLinkRequestValidation::validate_ed25519_signature(magic_request) {
        return Ok(response);
    }

    // Step 2: Generate token and create magic link URL
    let token_result = match MagicLinkTokenGeneration::generate_complete_result(
        req,
        &magic_request.email,
        magic_request.ui_host.as_deref(),
        15, // 15 minutes expiration
    ) {
        Ok(result) => result,
        Err(response) => return Ok(response),
    };

    // Step 3: Store in database and send email
    match MagicLinkOperations::store_magic_link_encrypted(
        &token_result.magic_token,
        &token_result.encryption_blob,
        token_result.expires_at_nanos,
        magic_request.next.as_deref().unwrap_or("/"),
        &magic_request.pub_key,
    ) {
        Ok(_) => {
            // Send email with fallback to console logging
            let _ = MagicLinkEmailDelivery::send_with_fallback(
                &magic_request.email,
                &token_result.magic_link,
                Some(&magic_request.email_lang),
                magic_request.ui_host.as_deref(),
                &MagicLinkTokenGeneration::determine_host_url(
                    req,
                    magic_request.ui_host.as_deref(),
                ),
                token_result.magic_expires_at,
            )
            .await;

            // Clean up expired sessions
            let _ = MagicLinkOperations::cleanup_expired_links();

            Ok(MagicLinkResponseBuilder::build_success_response())
        }
        Err(e) => Ok(MagicLinkResponseBuilder::build_storage_error_response(
            &e.to_string(),
        )?),
    }
}

/// Generate and send magic link using universal signed request structure
///
/// Orchestrates the complete signed magic link generation flow using specialized modules:
/// - Request validation (rate limiting, signed request validation, email)
/// - Token generation and URL creation
/// - Database storage and email delivery
pub async fn generate_magic_link_signed(
    req: &Request,
    signed_request: &MagicLinkSignedRequest,
) -> anyhow::Result<Response> {
    // Step 1: Validate request (rate limiting and signed request)
    if let Err(response) = MagicLinkRequestValidation::check_rate_limiting(req) {
        return Ok(response);
    }

    let pub_key_hex = match MagicLinkRequestValidation::validate_signed_request(req, signed_request) {
        Ok(key) => key,
        Err(response) => return Ok(response),
    };

    if let Err(response) =
        MagicLinkRequestValidation::validate_email_format(&signed_request.payload.email)
    {
        return Ok(response);
    }

    // Step 2: Generate token and create magic link URL
    let token_result = match MagicLinkTokenGeneration::generate_complete_result(
        req,
        &signed_request.payload.email,
        signed_request.payload.ui_host.as_deref(),
        15, // 15 minutes expiration
    ) {
        Ok(result) => result,
        Err(response) => return Ok(response),
    };

    // Step 3: Store in database and send email
    match MagicLinkOperations::store_magic_link_encrypted(
        &token_result.magic_token,
        &token_result.encryption_blob,
        token_result.expires_at_nanos,
        &signed_request.payload.next,
        &signed_request.payload.pub_key,
    ) {
        Ok(_) => {
            // Send email with fallback to console logging
            let _ = MagicLinkEmailDelivery::send_with_fallback(
                &signed_request.payload.email,
                &token_result.magic_link,
                Some(&signed_request.payload.email_lang),
                signed_request.payload.ui_host.as_deref(),
                &MagicLinkTokenGeneration::determine_host_url(
                    req,
                    signed_request.payload.ui_host.as_deref(),
                ),
                token_result.magic_expires_at,
            )
            .await;

            // Clean up expired sessions
            let _ = MagicLinkOperations::cleanup_expired_links();

            // Create signed response with server public key (magic link creation scenario)
            match create_signed_magic_link_response(&signed_request.payload.email, &pub_key_hex) {
                Ok(response) => Ok(response),
                Err(e) => {
                    println!("❌ Error creating signed response: {}", e);
                    Ok(MagicLinkResponseBuilder::build_success_response()) // Fallback to unsigned
                }
            }
        }
        Err(e) => Ok(MagicLinkResponseBuilder::build_storage_error_response(
            &e.to_string(),
        )?),
    }
}

/// Create signed response for magic link generation (with server_pub_key)
///
/// Generates a signed response using Ed25519 per-session keypair derived from
/// user_id + frontend_pub_key, includes server public key for verification.
///
/// # Arguments
/// * `email` - User email to derive user_id
/// * `pub_key_hex` - Frontend Ed25519 public key as hex string
///
/// # Returns
/// * `Result<Response, String>` - Signed HTTP response or error
fn create_signed_magic_link_response(email: &str, pub_key_hex: &str) -> Result<Response, String> {
    // Step 1: Derive user_id from email
    let user_id = derive_user_id(email)
        .map_err(|e| format!("Failed to derive user_id: {}", e))?;

    // Step 2: Create payload with status OK
    let payload = json!({
        "status": "OK"
    });

    // Step 3: Generate signed response with server public key
    let signed_response = SignedResponseGenerator::create_signed_response_with_server_pubkey(
        payload,
        &user_id,
        pub_key_hex,
    )
    .map_err(|e| format!("Failed to create signed response: {}", e))?;

    // Step 4: Serialize signed response to JSON
    let response_json = serde_json::to_string(&signed_response)
        .map_err(|e| format!("Failed to serialize signed response: {}", e))?;

    // Step 5: Build HTTP response with CORS headers
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "POST, GET, OPTIONS")
        .header("access-control-allow-headers", "Content-Type")
        .body(response_json)
        .build())
}
