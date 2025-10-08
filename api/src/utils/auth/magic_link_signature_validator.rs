//! Magic Link Signature Validator - Ed25519 Signature Verification
//!
//! Single Responsibility: Handle Ed25519 digital signature verification operations
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use hex;
use spin_sdk::http::Response;
use tracing::{debug, error};

use super::types::ErrorResponse;
use crate::utils::ed25519::{Ed25519Utils, SignatureVerificationResult};

/// Verify Ed25519 signature for magic link authentication
///
/// This is a critical security step that must pass before issuing JWT credentials.
/// The signature proves possession of the private key corresponding to the public key
/// embedded in the magic link token.
///
/// CORRECTED BASE64 APPROACH: Verifies Base64 payload signature directly.
/// Frontend signs the Base64 payload string directly - backend verifies against same string.
/// NO RECREATION NEEDED - we already have what was signed!
///
/// # Arguments
/// * `base64_payload` - The Base64 payload string that was signed by frontend
/// * `signature_hex` - The Ed25519 signature as hex string
/// * `pub_key_bytes` - The Ed25519 public key bytes (32 bytes)
///
/// # Returns
/// * `Result<(), Response>` - Success or error response
pub fn verify_magic_link_signature(
    base64_payload: &str,
    signature_hex: &str,
    pub_key_bytes: &[u8; 32],
) -> Result<(), Response> {
    // println!(
    //     "🔍 DEBUG Ed25519: Verifying signature for Base64 payload: {}...",
    //     &base64_payload[..base64_payload.len().min(50)]
    // );
    debug!(
        "🔍 DEBUG Ed25519: Verifying signature for Base64 payload: {}...",
        &base64_payload[..base64_payload.len().min(50)]
    );

    // Convert pub_key_array to hex string for verification
    let pub_key_hex = hex::encode(pub_key_bytes);
    // println!(
    //     "🔍 DEBUG: Using pub_key_hex for verification: {}",
    //     pub_key_hex
    // );
    debug!(
        "🔍 DEBUG: Using pub_key_hex for verification: {}",
        pub_key_hex
    );

    // CORRECTED: Verify signature directly against the Base64 payload received from frontend!
    // NO NEED TO RECREATE ANYTHING - we already have what was signed!
    // println!(
    //     "🔍 DEBUG BASE64: Verifying signature against received Base64 payload (length {}): {}...",
    //     base64_payload.len(),
    //     &base64_payload[..base64_payload.len().min(100)]
    // );
    debug!(
        "🔍 DEBUG BASE64: Verifying signature against received Base64 payload (length {}): {}...",
        base64_payload.len(),
        &base64_payload[..base64_payload.len().min(100)]
    );

    // The message that was signed is the Base64 payload we received - USE IT DIRECTLY!
    let message_to_verify = base64_payload.as_bytes();

    // Perform Ed25519 signature verification
    let signature_verification_result =
        Ed25519Utils::verify_signature(message_to_verify, signature_hex, &pub_key_hex);

    // Handle verification result
    match signature_verification_result {
        SignatureVerificationResult::Valid => {
            // println!("✅ Ed25519 signature verification successful");
            debug!("✅ Ed25519 signature verification successful");
            Ok(())
        }
        SignatureVerificationResult::Invalid => {
            // println!("❌ Ed25519 signature verification failed - invalid signature");
            error!("❌ Ed25519 signature verification failed - invalid signature");
            Err(create_signature_error_response(
                401,
                "Ed25519 signature verification failed",
            ))
        }
        SignatureVerificationResult::MalformedPublicKey => {
            // println!("❌ Ed25519 signature verification error: malformed public key");
            error!("❌ Ed25519 signature verification error: malformed public key");
            Err(create_signature_error_response(
                400,
                "Ed25519 malformed public key",
            ))
        }
        SignatureVerificationResult::MalformedSignature => {
            // println!("❌ Ed25519 signature verification error: malformed signature");
            error!("❌ Ed25519 signature verification error: malformed signature");
            Err(create_signature_error_response(
                400,
                "Ed25519 malformed signature",
            ))
        }
        SignatureVerificationResult::MalformedMessage => {
            // println!("❌ Ed25519 signature verification error: malformed message");
            error!("❌ Ed25519 signature verification error: malformed message");
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
