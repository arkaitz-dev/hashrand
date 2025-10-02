//! Core validation logic for signed requests

use serde_json::Value;
use spin_sdk::http::Request;

use super::errors::SignedRequestError;
use super::extraction::{
    extract_pub_key_from_bearer, extract_pub_key_from_magiclink, extract_pub_key_from_payload,
};
use super::serialization::{decode_payload_base64, serialize_query_params_deterministic};
use super::types::SignedRequest;
use crate::utils::ed25519::{Ed25519Utils, SignatureVerificationResult};

/// Universal validation with strict auth method separation
///
/// Validates Base64 URL-safe encoded JSON payload for perfect consistency
///
/// SECURITY RULES:
/// 1. Bearer token present: ONLY Bearer allowed, NO pub_key/magiclink in payload
/// 2. No Bearer token: EXACTLY one of pub_key OR magiclink in payload (never both, never none)
///
/// # Arguments
/// * `signed_request` - The signed request with Base64-encoded JSON payload
/// * `request` - HTTP request (for Bearer token extraction)
///
/// # Returns
/// * `Result<String, SignedRequestError>` - pub_key_hex or error
pub fn validate_universal(
    signed_request: &SignedRequest,
    request: &Request,
) -> Result<String, SignedRequestError> {
    println!("🔍 Universal SignedRequest validation with strict auth separation...");

    // Decode Base64 payload to check auth method contents
    println!("🔍 DEBUG BASE64: Decoding Base64 payload for auth method detection...");
    let json_string = decode_payload_base64(&signed_request.payload).map_err(|e| {
        SignedRequestError::SerializationError(format!("Base64 decoding failed: {}", e))
    })?;

    let payload_value: Value = serde_json::from_str(&json_string).map_err(|e| {
        SignedRequestError::SerializationError(format!("JSON parsing failed: {}", e))
    })?;

    println!("🔍 DEBUG BASE64: Decoded payload: {}", payload_value);

    // Check what auth methods are present in payload
    let has_pub_key = payload_value
        .get("pub_key")
        .and_then(|v| v.as_str())
        .is_some();
    let has_magiclink = payload_value
        .get("magiclink")
        .and_then(|v| v.as_str())
        .is_some();

    // Check if Bearer token is present
    let has_bearer = extract_pub_key_from_bearer(request).is_ok();

    println!(
        "🔍 Auth method detection - Bearer: {}, pub_key: {}, magiclink: {}",
        has_bearer, has_pub_key, has_magiclink
    );

    // STRICT VALIDATION RULES
    if has_bearer {
        // Rule 1: Bearer token present - NO other auth methods allowed in payload
        if has_pub_key || has_magiclink {
            return Err(SignedRequestError::ConflictingAuthMethods(
                "Bearer token present but payload contains pub_key/magiclink - only Bearer allowed"
                    .to_string(),
            ));
        }

        // Use Bearer token for validation
        let pub_key_hex = extract_pub_key_from_bearer(request)?;
        println!("✅ Using ONLY Bearer token (strict mode)");
        validate_base64_payload(
            &signed_request.payload,
            &signed_request.signature,
            &pub_key_hex,
        )?;
        Ok(pub_key_hex)
    } else {
        // Rule 2: No Bearer token - EXACTLY one payload auth method required
        match (has_pub_key, has_magiclink) {
            (true, true) => Err(SignedRequestError::AmbiguousPayloadAuth(
                "Both pub_key and magiclink found in payload - only one allowed".to_string(),
            )),
            (true, false) => {
                // Use pub_key from payload
                let pub_key_hex = extract_pub_key_from_payload(&payload_value)?;
                println!("✅ Using ONLY pub_key from payload (strict mode)");
                validate_base64_payload(
                    &signed_request.payload,
                    &signed_request.signature,
                    &pub_key_hex,
                )?;
                Ok(pub_key_hex)
            }
            (false, true) => {
                // Use magiclink from payload
                let pub_key_hex = extract_pub_key_from_magiclink(&payload_value)?;
                println!("✅ Using ONLY magiclink from payload (strict mode)");
                validate_base64_payload(
                    &signed_request.payload,
                    &signed_request.signature,
                    &pub_key_hex,
                )?;
                Ok(pub_key_hex)
            }
            (false, false) => Err(SignedRequestError::MissingPublicKey(
                "No Bearer token and no pub_key/magiclink in payload - exactly one auth method required"
                    .to_string(),
            )),
        }
    }
}

/// Validate Base64 payload with Ed25519 signature
///
/// The Base64 string itself is signed/verified directly (most deterministic!)
///
/// # Arguments
/// * `base64_payload` - Base64 URL-safe encoded payload string (this is what was signed)
/// * `signature` - Ed25519 signature hex string
/// * `public_key_hex` - Ed25519 public key as hex string
///
/// # Returns
/// * `Result<(), SignedRequestError>` - Success or error
pub fn validate_base64_payload(
    base64_payload: &str,
    signature: &str,
    public_key_hex: &str,
) -> Result<(), SignedRequestError> {
    println!(
        "🔍 DEBUG BASE64: Validating signature directly against Base64 string - Length: {}, Signature: {}...",
        base64_payload.len(),
        &signature[..20.min(signature.len())]
    );

    // Validate signature directly against the Base64 string (most deterministic approach!)
    validate_signature_string(base64_payload, signature, public_key_hex)
}

/// Validate Ed25519 signature for any serialized string
///
/// DRY function used by both GET (query params) and POST (JSON payload) endpoints
///
/// # Arguments
/// * `serialized_data` - Serialized data string (Base64, JSON, or query params)
/// * `signature` - Ed25519 signature hex string
/// * `public_key_hex` - Ed25519 public key as hex string
///
/// # Returns
/// * `Result<(), SignedRequestError>` - Success or error
pub fn validate_signature_string(
    serialized_data: &str,
    signature: &str,
    public_key_hex: &str,
) -> Result<(), SignedRequestError> {
    println!(
        "🔍 Validating Ed25519 signature - Data size: {}, Signature: {}...",
        serialized_data.len(),
        &signature[..20.min(signature.len())]
    );

    // Verify Ed25519 signature
    let verification_result =
        Ed25519Utils::verify_signature_string(serialized_data, signature, public_key_hex);

    verify_ed25519_signature_result(verification_result)
}

/// Process Ed25519 signature verification result
///
/// DRY function that converts SignatureVerificationResult to Result
fn verify_ed25519_signature_result(
    verification_result: SignatureVerificationResult,
) -> Result<(), SignedRequestError> {
    match verification_result {
        SignatureVerificationResult::Valid => {
            println!("✅ Ed25519 signature validation successful");
            Ok(())
        }
        SignatureVerificationResult::Invalid => Err(SignedRequestError::InvalidSignature(
            "Ed25519 signature verification failed".to_string(),
        )),
        SignatureVerificationResult::MalformedPublicKey => Err(
            SignedRequestError::InvalidSignature("Invalid Ed25519 public key format".to_string()),
        ),
        SignatureVerificationResult::MalformedSignature => Err(
            SignedRequestError::InvalidSignature("Invalid signature format".to_string()),
        ),
        SignatureVerificationResult::MalformedMessage => Err(SignedRequestError::InvalidSignature(
            "Invalid message format".to_string(),
        )),
    }
}

/// Validate GET request with query parameters + signature
///
/// Query parameters are serialized deterministically and validated with Ed25519
///
/// # Arguments
/// * `query_params` - Mutable HashMap of query parameters (signature will be removed)
/// * `public_key_hex` - Ed25519 public key as hex string
///
/// # Returns
/// * `Result<(), SignedRequestError>` - Success or error
pub fn validate_query_params(
    query_params: &mut std::collections::HashMap<String, String>,
    public_key_hex: &str,
) -> Result<(), SignedRequestError> {
    // Extract signature from query parameters
    let signature = query_params.remove("signature").ok_or_else(|| {
        SignedRequestError::MissingPublicKey("Missing 'signature' query parameter".to_string())
    })?;

    // Serialize remaining query parameters deterministically
    let serialized_params = serialize_query_params_deterministic(query_params)
        .map_err(|e| SignedRequestError::SerializationError(e.to_string()))?;

    // Validate signature using DRY function
    validate_signature_string(&serialized_params, &signature, public_key_hex)
}
