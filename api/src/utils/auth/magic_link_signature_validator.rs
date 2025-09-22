//! Magic Link Signature Validator - Ed25519 Signature Verification
//!
//! Single Responsibility: Handle Ed25519 digital signature verification operations
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use hex;
use spin_sdk::http::Response;

use super::types::ErrorResponse;
use crate::utils::ed25519::{Ed25519Utils, SignatureVerificationResult};

/// Verify Ed25519 signature for magic link authentication
///
/// This is a critical security step that must pass before issuing JWT credentials.
/// The signature proves possession of the private key corresponding to the public key
/// embedded in the magic link token.
///
/// # Arguments
/// * `magic_token` - The magic link token that was signed
/// * `signature_hex` - The Ed25519 signature as hex string
/// * `pub_key_bytes` - The Ed25519 public key bytes (32 bytes)
///
/// # Returns
/// * `Result<(), Response>` - Success or error response
pub fn verify_magic_link_signature(
    magic_token: &str,
    signature_hex: &str,
    pub_key_bytes: &[u8; 32],
) -> Result<(), Response> {
    println!(
        "🔍 DEBUG Ed25519: Verifying signature for magic link token: {}",
        magic_token
    );

    // Convert pub_key_array to hex string for verification
    let pub_key_hex = hex::encode(pub_key_bytes);

    // The message that was signed is the magic link token itself
    let message_to_verify = magic_token.as_bytes();

    // Perform Ed25519 signature verification
    let signature_verification_result =
        Ed25519Utils::verify_signature(message_to_verify, signature_hex, &pub_key_hex);

    // Handle verification result
    match signature_verification_result {
        SignatureVerificationResult::Valid => {
            println!("✅ Ed25519 signature verification successful");
            Ok(())
        }
        SignatureVerificationResult::Invalid => {
            println!("❌ Ed25519 signature verification failed - invalid signature");
            Err(create_signature_error_response(
                401,
                "Ed25519 signature verification failed",
            ))
        }
        SignatureVerificationResult::MalformedPublicKey => {
            println!("❌ Ed25519 signature verification error: malformed public key");
            Err(create_signature_error_response(
                400,
                "Ed25519 malformed public key",
            ))
        }
        SignatureVerificationResult::MalformedSignature => {
            println!("❌ Ed25519 signature verification error: malformed signature");
            Err(create_signature_error_response(
                400,
                "Ed25519 malformed signature",
            ))
        }
        SignatureVerificationResult::MalformedMessage => {
            println!("❌ Ed25519 signature verification error: malformed message");
            Err(create_signature_error_response(
                400,
                "Ed25519 malformed message",
            ))
        }
    }
}

/// Create standardized error response for signature verification failures
fn create_signature_error_response(status: u16, error_message: &str) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ErrorResponse {
                error: error_message.to_string(),
            })
            .unwrap_or_default(),
        )
        .build()
}