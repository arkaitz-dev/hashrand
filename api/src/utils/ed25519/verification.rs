//! Ed25519 Signature Verification
//!
//! Core cryptographic verification using ed25519-dalek

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use super::conversion::{decode_public_key, decode_signature};
use super::types::SignatureVerificationResult;

/// Verify Ed25519 signature against message
///
/// # Arguments
/// * `message` - The original message that was signed (as bytes)
/// * `signature_base58` - The Ed25519 signature as base58 string (~88 chars)
/// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
///
/// # Returns
/// * `SignatureVerificationResult` - Verification result
pub fn verify_signature(
    message: &[u8],
    signature_base58: &str,
    public_key_hex: &str,
) -> SignatureVerificationResult {
    // Validate message
    if message.is_empty() {
        return SignatureVerificationResult::MalformedMessage;
    }

    // Decode and validate public key
    let public_key_bytes = match decode_public_key(public_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return SignatureVerificationResult::MalformedPublicKey,
    };

    // Decode and validate signature (now base58)
    let signature_bytes = match decode_signature(signature_base58) {
        Ok(bytes) => bytes,
        Err(_) => return SignatureVerificationResult::MalformedSignature,
    };

    // Create Ed25519 verifying key
    let verifying_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => return SignatureVerificationResult::MalformedPublicKey,
    };

    // Create Ed25519 signature
    let signature = Signature::from_bytes(&signature_bytes);

    // Verify signature
    match verifying_key.verify(message, &signature) {
        Ok(()) => SignatureVerificationResult::Valid,
        Err(_) => SignatureVerificationResult::Invalid,
    }
}

/// Verify Ed25519 signature for string message
///
/// # Arguments
/// * `message` - The original message that was signed (as string)
/// * `signature_base58` - The Ed25519 signature as base58 string (~88 chars)
/// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
///
/// # Returns
/// * `SignatureVerificationResult` - Verification result
pub fn verify_signature_string(
    message: &str,
    signature_base58: &str,
    public_key_hex: &str,
) -> SignatureVerificationResult {
    let message_bytes = message.as_bytes();
    verify_signature(message_bytes, signature_base58, public_key_hex)
}
