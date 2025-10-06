//! Core Ed25519 signing logic for responses

use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::SignedResponseError;
use super::key_derivation::derive_session_private_key;
use super::types::SignedResponse;
use crate::utils::signed_request::SignedRequestValidator;

/// Generate signed response with per-session Ed25519 key
///
/// CRYPTOGRAPHIC FLOW (Blake3 Pseudonimizer):
/// 1. user_id_bytes + pub_key_bytes → Blake3 pseudonimizer → Ed25519 private key (32 bytes)
/// 2. Ed25519 keypair generation from private key
/// 3. Deterministic JSON serialization (matching frontend)
/// 4. Ed25519 signature of serialized payload (base58-encoded, ~88 chars)
///
/// # Arguments
/// * `payload` - Response data to be signed
/// * `user_id` - User ID bytes (16 bytes)
/// * `pub_key_hex` - Frontend Ed25519 public key as hex string
///
/// # Returns
/// * `Result<SignedResponse, SignedResponseError>` - Signed response with base58 signature
pub fn create_signed_response<T>(
    payload: T,
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<SignedResponse, SignedResponseError>
where
    T: Serialize,
{
    // Step 1: Derive per-session private key
    let private_key = derive_session_private_key(user_id, pub_key_hex)?;

    // Step 2: Generate Ed25519 keypair
    let signing_key = SigningKey::from_bytes(&private_key);

    // Step 3: Serialize payload to deterministic JSON for frontend consistency
    let json_string = SignedRequestValidator::serialize_payload_deterministic(&payload)
        .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

    // Step 4: Encode JSON as Base64 URL-safe
    let base64_payload = {
        let bytes = json_string.as_bytes();
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
            .replace('+', "-")
            .replace('/', "_")
            .replace('=', "")
    };

    // Step 5: Sign Base64 payload with Ed25519 (same as frontend!)
    let signature_bytes = signing_key.sign(base64_payload.as_bytes());
    // let signature_hex = hex::encode(signature_bytes.to_bytes());
    let signature_base58 = bs58::encode(signature_bytes.to_bytes()).into_string();

    Ok(SignedResponse {
        payload: base64_payload,
        signature: signature_base58,
    })
}

/// Create signed response with server public key included
///
/// Used for magic link creation and token refresh endpoints
/// where frontend needs server's public key for verification
///
/// # Arguments
/// * `payload` - Response data to be signed
/// * `user_id` - User ID bytes (16 bytes)
/// * `pub_key_hex` - Frontend Ed25519 public key as hex string
///
/// # Returns
/// * `Result<SignedResponse, SignedResponseError>` - Signed response with server_pub_key
pub fn create_signed_response_with_server_pubkey<T>(
    payload: T,
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<SignedResponse, SignedResponseError>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    // Derive session private key and generate keypair
    let private_key = derive_session_private_key(user_id, pub_key_hex)?;
    let signing_key = SigningKey::from_bytes(&private_key);
    let public_key = signing_key.verifying_key();

    // Add server_pub_key to payload
    let mut payload_value = serde_json::to_value(&payload)
        .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

    if let Value::Object(ref mut map) = payload_value {
        map.insert(
            "server_pub_key".to_string(),
            Value::String(hex::encode(public_key.as_bytes())),
        );
    } else {
        return Err(SignedResponseError::SerializationError(
            "Payload must be a JSON object to add server_pub_key".to_string(),
        ));
    }

    // Convert back to original type
    let enhanced_payload: T = serde_json::from_value(payload_value)
        .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

    // Create signed response with enhanced payload
    create_signed_response(enhanced_payload, user_id, pub_key_hex)
}

/// Create signed response for key rotation (TRAMO 2/3)
///
/// SECURITY: Uses OLD pub_key to sign response (prevents MITM)
/// but includes NEW server_pub_key in payload (for rotation)
///
/// # Arguments
/// * `payload` - Response data to be signed
/// * `user_id` - User ID bytes (16 bytes)
/// * `signing_pub_key_hex` - OLD frontend pub_key (used to derive signing key)
/// * `payload_pub_key_hex` - NEW frontend pub_key (used to derive server_pub_key for payload)
///
/// # Returns
/// * `Result<SignedResponse, SignedResponseError>` - Signed response with NEW server_pub_key
pub fn create_signed_response_with_rotation<T>(
    payload: T,
    user_id: &[u8],
    signing_pub_key_hex: &str,
    payload_pub_key_hex: &str,
) -> Result<SignedResponse, SignedResponseError>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    // Derive NEW server keypair from NEW frontend pub_key (for payload)
    let new_private_key = derive_session_private_key(user_id, payload_pub_key_hex)?;
    let new_signing_key = SigningKey::from_bytes(&new_private_key);
    let new_public_key = new_signing_key.verifying_key();

    // Add NEW server_pub_key to payload
    let mut payload_value = serde_json::to_value(&payload)
        .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

    if let Value::Object(ref mut map) = payload_value {
        map.insert(
            "server_pub_key".to_string(),
            Value::String(hex::encode(new_public_key.as_bytes())),
        );
    } else {
        return Err(SignedResponseError::SerializationError(
            "Payload must be a JSON object to add server_pub_key".to_string(),
        ));
    }

    // Convert back to original type
    let enhanced_payload: T = serde_json::from_value(payload_value)
        .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

    // Create signed response using OLD pub_key for signing (SECURITY)
    create_signed_response(enhanced_payload, user_id, signing_pub_key_hex)
}
