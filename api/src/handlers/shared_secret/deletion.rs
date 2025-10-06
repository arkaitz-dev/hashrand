//! Shared secret deletion endpoint
//!
//! DELETE /api/shared-secret/{hash} - Delete shared secret
//! Requires JWT authentication and Ed25519 signature validation
//! Only allows deletion if pending_reads > 0

use crate::database::operations::{
    shared_secret_crypto::SharedSecretCrypto, shared_secret_ops::SharedSecretOps,
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

    // Decode hash from Base58 (40 bytes - encrypted with ChaCha20)
    let encrypted_hash = match decode_hash_v2(hash) {
        Ok(hash) => hash,
        Err(e) => return Ok(create_client_error_response(&e)),
    };

    // Extract user_id from crypto material (JWT)
    let mut user_id_from_jwt = [0u8; USER_ID_LENGTH];
    if crypto_material.user_id.len() != USER_ID_LENGTH {
        return Ok(create_auth_error_response("Invalid user_id length in JWT"));
    }
    user_id_from_jwt.copy_from_slice(&crypto_material.user_id);

    // Validate and delete with 3-layer validation
    match delete_secret_validated_v2(&encrypted_hash, &user_id_from_jwt, &crypto_material) {
        Ok(response) => Ok(response),
        Err(e) => Ok(create_server_error_response(&e)),
    }
}

/// Decode Base58 hash to encrypted 40-byte hash (v2 - NEW)
fn decode_hash_v2(hash: &str) -> Result<[u8; 40], String> {
    let decoded = bs58::decode(hash)
        .into_vec()
        .map_err(|_| "Invalid Base58 hash".to_string())?;

    if decoded.len() != 40 {
        return Err(format!(
            "Invalid hash length: expected 40, got {}",
            decoded.len()
        ));
    }

    let mut encrypted_hash = [0u8; 40];
    encrypted_hash.copy_from_slice(&decoded);
    Ok(encrypted_hash)
}

/// Delete secret with 3-layer validation (v2 - NEW)
fn delete_secret_validated_v2(
    encrypted_hash: &[u8; 40],
    user_id_from_jwt: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // ============================================================================
    // 3-LAYER VALIDATION: Checksum → Ownership → Database
    // ============================================================================

    // Layer 1: Decrypt ChaCha20 hash
    let decrypted_hash = SharedSecretCrypto::decrypt_url_hash(encrypted_hash)
        .map_err(|e| format!("Failed to decrypt hash: {}", e))?;

    // Layer 2: Validate checksum + Extract components (reference_hash, user_id, role)
    let (reference_hash, user_id_from_hash, role) =
        SharedSecretCrypto::validate_and_extract_hash(&decrypted_hash)
            .map_err(|e| format!("Invalid hash checksum: {}", e))?;

    // Layer 3: CRITICAL - Validate ownership (user_id from JWT must match user_id from hash)
    if user_id_from_jwt != &user_id_from_hash {
        return Err(
            "Access denied: You cannot delete a shared secret that doesn't belong to you"
                .to_string(),
        );
    }

    // Generate db_index for database lookup
    let db_index = SharedSecretCrypto::generate_db_index(&reference_hash, &user_id_from_hash)
        .map_err(|e| format!("Failed to generate db_index: {}", e))?;

    // Read secret to get pending_reads from tracking
    let (_, pending_reads, _, _role_from_db) = SharedSecretOps::read_secret(&db_index)
        .map_err(|e| format!("Failed to read secret: {}", e))?;

    // Check if deletion is allowed (pending_reads > 0 or sender with unlimited reads)
    if pending_reads <= 0 {
        return Err(
            "Cannot delete secret: all reads have been consumed or it's already deleted"
                .to_string(),
        );
    }

    // Delete the secret
    let deleted = SharedSecretStorage::delete_secret(&db_index)
        .map_err(|e| format!("Failed to delete secret: {}", e))?;

    if !deleted {
        return Err("Secret not found or already deleted".to_string());
    }

    // Create success response (use role from hash, not database)
    let response_json = json!({
        "success": true,
        "message": "Secret deleted successfully",
        "role": role.to_str()
    });

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}

/// Decode Base58 hash to encrypted ID (OLD - deprecated)
#[allow(dead_code)]
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

/// Delete secret with validation (OLD - deprecated)
#[allow(dead_code)]
fn delete_secret_validated(
    encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
    _user_id: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // TODO: Validate that user_id from JWT matches user_id encrypted in hash

    // Read secret to get pending_reads from tracking
    let (_, pending_reads, _, role) = SharedSecretOps::read_secret(encrypted_id)
        .map_err(|e| format!("Failed to read secret: {}", e))?;

    // Check if deletion is allowed (pending_reads > 0 or sender with unlimited reads)
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
