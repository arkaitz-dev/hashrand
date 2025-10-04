//! Shared secret tracking endpoint
//!
//! GET /api/shared-secret/confirm-read?hash={hash}&signature={sig}
//! Confirms read by updating tracking record
//! Requires JWT authentication and Ed25519 signature validation

use crate::database::operations::{
    shared_secret_crypto::SharedSecretCrypto, shared_secret_ops::SharedSecretOps,
    shared_secret_storage::SharedSecretStorage, shared_secret_types::constants::*,
};
use crate::utils::{
    CryptoMaterial, ProtectedEndpointMiddleware, ProtectedEndpointResult,
    create_auth_error_response, create_client_error_response, create_server_error_response,
    create_signed_endpoint_response, extract_crypto_material_from_request,
};
use serde_json::json;
use spin_sdk::http::{Request, Response};

/// Handle GET /api/shared-secret/confirm-read?hash={hash}
pub async fn handle_confirm_read(req: Request, hash: &str) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate JWT and signature (GET request with query params)
    let _result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    // Extract crypto material
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Crypto extraction failed: {}",
                e
            )));
        }
    };

    // Decode hash from Base58
    let encrypted_id = match decode_hash(hash) {
        Ok(id) => id,
        Err(e) => return Ok(create_client_error_response(&e)),
    };

    // Extract user_id from crypto material
    let mut user_id = [0u8; USER_ID_LENGTH];
    if crypto_material.user_id.len() != USER_ID_LENGTH {
        return Ok(create_auth_error_response("Invalid user_id length in JWT"));
    }
    user_id.copy_from_slice(&crypto_material.user_id);

    // Confirm read
    match confirm_read_validated(&encrypted_id, &user_id, &crypto_material) {
        Ok(response) => Ok(response),
        Err(e) => Ok(create_server_error_response(&e)),
    }
}

/// Decode Base58 hash to encrypted ID
fn decode_hash(hash: &str) -> Result<[u8; ENCRYPTED_ID_LENGTH], String> {
    let decoded = bs58::decode(hash)
        .into_vec()
        .map_err(|_| "Invalid Base58 hash".to_string())?;

    if decoded.len() != ENCRYPTED_ID_LENGTH {
        return Err(format!(
            "Invalid hash length: expected {}, got {}",
            ENCRYPTED_ID_LENGTH,
            decoded.len()
        ));
    }

    let mut id = [0u8; ENCRYPTED_ID_LENGTH];
    id.copy_from_slice(&decoded);
    Ok(id)
}

/// Confirm read with validation
fn confirm_read_validated(
    encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
    _user_id: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // TODO: Validate that user_id from JWT matches receiver_user_id in hash

    // Retrieve secret to get reference_hash
    let secret_data = SharedSecretStorage::retrieve_secret(encrypted_id)
        .map_err(|e| format!("Failed to retrieve secret: {}", e))?;

    let (encrypted_payload, _, _, role) = match secret_data {
        Some(data) => data,
        None => {
            return Err("Secret not found".to_string());
        }
    };

    // Decrypt payload to extract reference_hash
    let decrypted = SharedSecretCrypto::decrypt_payload(encrypted_id, &encrypted_payload)
        .map_err(|e| format!("Failed to decrypt payload: {}", e))?;

    let payload = SharedSecretOps::deserialize_payload(&decrypted)
        .map_err(|e| format!("Failed to deserialize payload: {}", e))?;

    // Convert reference_hash to [u8; REFERENCE_HASH_LENGTH]
    if payload.reference_hash.len() != REFERENCE_HASH_LENGTH {
        return Err(format!(
            "Invalid reference_hash length: expected {}, got {}",
            REFERENCE_HASH_LENGTH,
            payload.reference_hash.len()
        ));
    }

    let mut reference_hash = [0u8; REFERENCE_HASH_LENGTH];
    reference_hash.copy_from_slice(&payload.reference_hash);

    // Update tracking record (only if receiver)
    let updated = SharedSecretOps::confirm_read(&reference_hash)
        .map_err(|e| format!("Failed to confirm read: {}", e))?;

    // Create response
    let response_json = json!({
        "success": true,
        "updated": updated,
        "role": role.to_str(),
        "message": if updated {
            "Read confirmation updated"
        } else {
            "Already confirmed or not found"
        }
    });

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}
