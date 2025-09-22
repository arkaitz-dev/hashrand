//! Magic link validation business logic - SOLID Architecture
//!
//! Single Responsibility: Orchestrate magic link validation workflow
//! Part of enterprise-grade SOLID architecture transformation

use spin_sdk::http::Response;

use super::{
    magic_link_auth_response_builder::build_authentication_response,
    magic_link_jwt_generator::generate_jwt_tokens,
    magic_link_request_parser::{extract_request_data, parse_validation_request},
    magic_link_signature_validator::verify_magic_link_signature,
    magic_link_token_processor::validate_and_extract_token_data,
};

/// Validate magic link with Ed25519 signature verification (secure POST endpoint)
///
/// This orchestrates the complete magic link validation workflow using specialized modules:
/// - Parses unified SignedRequest structure containing magic link token and Ed25519 signature
/// - Validates and consumes the encrypted magic token extracting embedded data
/// - Verifies Ed25519 signature using public key from magic link payload
/// - Generates JWT access/refresh tokens upon successful verification
/// - Returns complete authentication response with secure HttpOnly cookies
///
/// # Arguments
/// * `request_body` - Raw HTTP request body containing SignedRequest JSON
///
/// # Returns
/// * `anyhow::Result<Response>` - Complete HTTP response or error
pub fn validate_magic_link_secure(request_body: &[u8]) -> anyhow::Result<Response> {
    println!("🔐 Starting secure magic link validation with Ed25519 verification");

    // Step 1: Parse and validate request structure
    let signed_request = match parse_validation_request(request_body) {
        Ok(request) => request,
        Err(error_response) => return Ok(error_response),
    };

    // Step 2: Extract magic token and signature from request
    let (magic_token, signature_hex) = extract_request_data(&signed_request);

    // Step 3: Validate magic link token and extract embedded data
    let token_data = match validate_and_extract_token_data(&magic_token) {
        Ok(data) => data,
        Err(error_response) => return Ok(error_response),
    };

    // Step 4: Verify Ed25519 signature (critical security step)
    if let Err(error_response) = verify_magic_link_signature(
        &magic_token,
        &signature_hex,
        &token_data.pub_key_bytes,
    ) {
        return Ok(error_response);
    }

    // Step 5: Generate JWT access and refresh tokens
    let jwt_tokens = match generate_jwt_tokens(&token_data.user_id_bytes, &token_data.pub_key_bytes) {
        Ok(tokens) => tokens,
        Err(error_response) => return Ok(error_response),
    };

    // Step 6: Build complete authentication response with secure cookies (SignedResponse format)
    let auth_response = build_authentication_response(
        jwt_tokens,
        token_data.next_param,
        &token_data.user_id_bytes,
        &token_data.pub_key_bytes,
    )?;

    println!("✅ Magic link validation completed successfully");

    Ok(auth_response)
}
