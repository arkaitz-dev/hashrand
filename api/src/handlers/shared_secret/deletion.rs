//! Shared secret deletion endpoint
//!
//! DELETE /api/shared-secret/{hash} - Delete shared secret
//! Requires JWT authentication and Ed25519 signature validation
//! Only allows deletion if pending_reads > 0

use crate::database::operations::{
    shared_secret_storage::SharedSecretStorage, shared_secret_types::constants::*,
};
use crate::utils::{
    CryptoMaterial, SignedRequestValidator, create_auth_error_response,
    create_client_error_response, create_server_error_response, create_signed_endpoint_response,
    endpoint_helpers::extract_query_params, extract_crypto_material_from_request,
};
use serde_json::json;
use spin_sdk::http::{Request, Response};

/// Handle DELETE /api/shared-secret/{hash}
pub async fn handle_delete_secret(req: Request, hash: &str) -> anyhow::Result<Response> {
    // Extract crypto material from JWT
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Authentication failed: {}",
                e
            )));
        }
    };

    // Extract query parameters
    let mut params = extract_query_params(&req);

    // Validate Ed25519 signature (DELETE must have signature parameter)
    if let Err(e) =
        SignedRequestValidator::validate_query_params(&mut params, &crypto_material.pub_key_hex)
    {
        return Ok(create_auth_error_response(&format!(
            "Signature validation failed: {}",
            e
        )));
    }

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

    // Validate and delete
    match delete_secret_validated(&encrypted_id, &user_id, &crypto_material) {
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

/// Delete secret with validation
fn delete_secret_validated(
    encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
    _user_id: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // TODO: Validate that user_id from JWT matches user_id encrypted in hash

    // Retrieve secret first to check pending_reads
    let secret_data = SharedSecretStorage::retrieve_secret(encrypted_id)
        .map_err(|e| format!("Failed to retrieve secret: {}", e))?;

    let (_, _, pending_reads, role) = match secret_data {
        Some(data) => data,
        None => {
            return Err("Secret not found or already deleted".to_string());
        }
    };

    // Check if deletion is allowed
    if pending_reads <= 0 {
        return Err(
            "Cannot delete secret: all reads have been consumed or it's already deleted"
                .to_string(),
        );
    }

    // Delete the secret
    let deleted = SharedSecretStorage::delete_secret(encrypted_id)
        .map_err(|e| format!("Failed to delete secret: {}", e))?;

    if !deleted {
        return Err("Secret not found or already deleted".to_string());
    }

    // Create success response
    let response_json = json!({
        "success": true,
        "message": "Secret deleted successfully",
        "role": role.to_str()
    });

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}
