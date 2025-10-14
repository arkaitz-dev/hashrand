//! Shared secret tracking endpoint
//!
//! GET /api/shared-secret/confirm-read?hash={hash}&signature={sig}
//! Confirms read by updating tracking record
//! Requires JWT authentication and Ed25519 signature validation

use tracing::{info, warn};

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

/// Handle GET /api/shared-secret/confirm-read?hash={hash}
pub async fn handle_confirm_read(req: Request, hash: &str) -> anyhow::Result<Response> {
    info!("✅ Request to /api/shared-secret/confirm-read endpoint");
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

    // Validate Ed25519 signature (GET must have signature parameter)
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

    // Confirm read with 3-layer validation
    match confirm_read_validated_v2(&encrypted_hash, &user_id_from_jwt, &crypto_material) {
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

/// Confirm read with 3-layer validation (v2 - NEW)
fn confirm_read_validated_v2(
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
            "Access denied: You cannot confirm read for a shared secret that doesn't belong to you"
                .to_string(),
        );
    }

    // Generate db_index for database lookup
    let db_index = SharedSecretCrypto::generate_db_index(&reference_hash, &user_id_from_hash)
        .map_err(|e| format!("Failed to generate db_index: {}", e))?;

    // ============================================================================
    // v3: Use read_secret() for simplified payload retrieval (centralized decryption)
    // ============================================================================
    let (payload, _, _, _role_from_db) = SharedSecretOps::read_secret(&db_index, &reference_hash)
        .map_err(|e| format!("Failed to read secret: {}", e))?;

    // No need for manual decryption - read_secret() handles all layers

    // VALIDATION: Check for manual DB tampering (pending_reads should never exceed max_reads)
    let current_pending_reads =
        SharedSecretStorage::get_pending_reads_from_tracking(&reference_hash)
            .map_err(|e| format!("Failed to get pending_reads: {}", e))?
            .unwrap_or(0);

    if current_pending_reads > payload.max_reads {
        // println!(
        //     "⚠️  WARNING: Potential DB tampering detected! pending_reads ({}) > max_reads ({})",
        //     current_pending_reads, payload.max_reads
        // );
        warn!(
            "⚠️  WARNING: Potential DB tampering detected! pending_reads ({}) > max_reads ({})",
            current_pending_reads, payload.max_reads
        );
        // Continue anyway - don't block legitimate users
    }

    // Decrement pending_reads (simple decrement, no idempotency)
    let new_pending_reads = SharedSecretStorage::decrement_tracking_reads(&reference_hash)
        .map_err(|e| format!("Failed to decrement pending_reads: {}", e))?;

    // Update tracking record with read timestamp (always mark timestamp)
    let read_confirmed = SharedSecretOps::confirm_read(&reference_hash)
        .map_err(|e| format!("Failed to confirm read: {}", e))?;

    // Auto-delete shared_secret if pending_reads reached 0 (consumed)
    if new_pending_reads == 0 {
        SharedSecretStorage::delete_secret(&db_index)
            .map_err(|e| format!("Failed to auto-delete secret: {}", e))?;
        info!("🗑️  Auto-deleted shared_secret (pending_reads=0, hash consumed)");
    }

    // Create response (use role from hash, not database)
    let response_json = json!({
        "success": true,
        "pending_reads": new_pending_reads,
        "read_confirmed": read_confirmed,
        "role": role.to_str(),
        "message": "Read confirmed and counter decremented"
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

/// Confirm read with validation (OLD - deprecated)
#[allow(dead_code)]
fn confirm_read_validated(
    encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
    _user_id: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // TODO: Validate that user_id from JWT matches receiver_user_id in hash

    // Retrieve secret to get reference_hash
    let secret_data = SharedSecretStorage::retrieve_secret(encrypted_id)
        .map_err(|e| format!("Failed to retrieve secret: {}", e))?;

    let (encrypted_payload, _, role) = match secret_data {
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

    // VALIDATION: Check for manual DB tampering (pending_reads should never exceed max_reads)
    let current_pending_reads =
        SharedSecretStorage::get_pending_reads_from_tracking(&reference_hash)
            .map_err(|e| format!("Failed to get pending_reads: {}", e))?
            .unwrap_or(0);

    if current_pending_reads > payload.max_reads {
        // println!(
        //     "⚠️  WARNING: Potential DB tampering detected! pending_reads ({}) > max_reads ({})",
        //     current_pending_reads, payload.max_reads
        // );
        warn!(
            "⚠️  WARNING: Potential DB tampering detected! pending_reads ({}) > max_reads ({})",
            current_pending_reads, payload.max_reads
        );
        // Continue anyway - don't block legitimate users
    }

    // Decrement pending_reads in tracking table
    let new_pending_reads = SharedSecretStorage::decrement_tracking_reads(&reference_hash)
        .map_err(|e| format!("Failed to decrement pending_reads: {}", e))?;

    // Update tracking record with read timestamp (only if receiver and not already set)
    let read_confirmed = SharedSecretOps::confirm_read(&reference_hash)
        .map_err(|e| format!("Failed to confirm read: {}", e))?;

    // Create response
    let response_json = json!({
        "success": true,
        "pending_reads": new_pending_reads,
        "read_confirmed": read_confirmed,
        "role": role.to_str(),
        "message": if read_confirmed {
            "Read confirmed and counter decremented"
        } else {
            "Counter decremented (read already confirmed)"
        }
    });

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}
