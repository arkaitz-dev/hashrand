//! Shared secret retrieval endpoint
//!
//! GET/POST /api/shared-secret/{hash} - Retrieve shared secret with OTP validation
//! Requires JWT authentication and Ed25519 signature validation

use tracing::info;

use crate::database::operations::{
    shared_secret_crypto::SharedSecretCrypto,
    shared_secret_ops::SharedSecretOps,
    shared_secret_storage::SharedSecretStorage,
    shared_secret_types::{SecretRole, constants::*},
};
use crate::utils::{
    CryptoMaterial, ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator,
    create_auth_error_response, create_client_error_response, create_forbidden_response,
    create_server_error_response, create_signed_endpoint_response,
    crypto::{ed25519_public_to_x25519, encrypt_with_ecdh, get_backend_x25519_private_key},
    endpoint_helpers::extract_query_params, extract_crypto_material_from_request,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::http::{Method, Request, Response};

/// Request payload for POST with OTP
///
/// Requester's Ed25519 public key comes from JWT (crypto_material.pub_key_hex)
/// and is used for ECDH encryption of key_material in the response
#[derive(Debug, Deserialize, Serialize)]
struct RetrieveSecretRequest {
    otp: String,
}

/// Response payload for retrieved shared secret with E2E encryption
#[derive(Debug, Serialize)]
struct RetrieveSecretResponse {
    /// ChaCha20-Poly1305 encrypted secret (base64 encoded)
    encrypted_secret: String,
    /// ECDH encrypted key_material for receiver (base64 encoded, 60 bytes)
    encrypted_key_material: String,
    sender_email: String,
    receiver_email: String,
    pending_reads: i64,
    max_reads: i64,
    expires_at: i64,
    reference: String,
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    otp: Option<String>, // Only included for sender role
    #[serde(skip_serializing_if = "Option::is_none")]
    read_at: Option<i64>, // Timestamp in seconds, None if not yet read
}

/// Main handler for GET/POST /api/shared-secret/{hash}
pub async fn handle_retrieve_secret(req: Request, hash: &str) -> anyhow::Result<Response> {
    info!("📥 Request to /api/shared-secret/{{hash}} GET/POST endpoint");
    match req.method() {
        Method::Get => handle_retrieve_secret_get(req, hash).await,
        Method::Post => handle_retrieve_secret_post(req, hash).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET request (no OTP provided)
async fn handle_retrieve_secret_get(req: Request, hash: &str) -> anyhow::Result<Response> {
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
    let encrypted_hash = match decode_hash(hash) {
        Ok(hash) => hash,
        Err(e) => return Ok(create_client_error_response(&e)),
    };

    // Extract user_id from crypto material (JWT)
    let mut user_id_from_jwt = [0u8; USER_ID_LENGTH];
    if crypto_material.user_id.len() != USER_ID_LENGTH {
        return Ok(create_auth_error_response("Invalid user_id length in JWT"));
    }
    user_id_from_jwt.copy_from_slice(&crypto_material.user_id);

    // Retrieve secret with 3-layer validation (checksum → ownership → database)
    // Requester's public key comes from JWT for ECDH encryption of key_material
    match retrieve_and_respond(
        &encrypted_hash,
        &user_id_from_jwt,
        None,
        &crypto_material.pub_key_hex,
        &crypto_material,
    ) {
        Ok(response) => Ok(response),
        Err(e) => {
            // Detect authorization errors (403 Forbidden) vs server errors (500)
            if e.starts_with("FORBIDDEN:") {
                Ok(create_forbidden_response(
                    e.replacen("FORBIDDEN:", "", 1).trim(),
                ))
            } else {
                Ok(create_server_error_response(&e))
            }
        }
    }
}

/// Handle POST request (with OTP)
async fn handle_retrieve_secret_post(req: Request, hash: &str) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request
    let result: ProtectedEndpointResult<RetrieveSecretRequest> =
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

    // Decode hash from Base58 (40 bytes - encrypted with ChaCha20)
    let encrypted_hash = match decode_hash(hash) {
        Ok(hash) => hash,
        Err(e) => return Ok(create_client_error_response(&e)),
    };

    // Extract user_id from crypto material (JWT)
    let mut user_id_from_jwt = [0u8; USER_ID_LENGTH];
    if crypto_material.user_id.len() != USER_ID_LENGTH {
        return Ok(create_auth_error_response("Invalid user_id length in JWT"));
    }
    user_id_from_jwt.copy_from_slice(&crypto_material.user_id);

    // Retrieve secret with OTP validation and 3-layer validation
    // Requester's public key comes from JWT for ECDH encryption of key_material
    match retrieve_and_respond(
        &encrypted_hash,
        &user_id_from_jwt,
        Some(&result.payload.otp),
        &crypto_material.pub_key_hex,
        &crypto_material,
    ) {
        Ok(response) => Ok(response),
        Err(e) => {
            // Detect authorization errors (403 Forbidden) vs server errors (500)
            if e.starts_with("FORBIDDEN:") {
                Ok(create_forbidden_response(
                    e.replacen("FORBIDDEN:", "", 1).trim(),
                ))
            } else {
                Ok(create_server_error_response(&e))
            }
        }
    }
}

/// Decode Base58 hash to encrypted 40-byte hash
fn decode_hash(hash: &str) -> Result<[u8; 40], String> {
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

/// Retrieve secret and create response with 3-layer validation
fn retrieve_and_respond(
    encrypted_hash: &[u8; 40],
    user_id_from_jwt: &[u8; USER_ID_LENGTH],
    provided_otp: Option<&str>,
    requester_public_key_hex: &str,
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
            "FORBIDDEN: Access denied: You cannot access a shared secret that doesn't belong to you"
                .to_string(),
        );
    }

    // Generate db_index for database lookup
    let db_index = SharedSecretCrypto::generate_db_index(&reference_hash, &user_id_from_hash)
        .map_err(|e| format!("Failed to generate db_index: {}", e))?;

    // ============================================================================
    // VALIDATION: Check if tracking exists (if sender deleted, cleanup receiver)
    // ============================================================================
    if !SharedSecretStorage::tracking_exists(&reference_hash)
        .map_err(|e| format!("Failed to check tracking existence: {}", e))?
    {
        // Tracking doesn't exist → Sender deleted everything
        // Cleanup: delete receiver's shared_secrets entry if exists
        let _ = SharedSecretStorage::delete_secret(&db_index); // Ignore errors (may not exist)

        return Err(
            "SECRET_DELETED: Secret no longer available: sender has deleted it".to_string(),
        );
    }

    // Read secret from database (no decrement - that happens in confirm-read endpoint)
    // v3: Pass reference_hash for centralized payload retrieval
    let (payload, pending_reads, expires_at, _role_from_db) =
        SharedSecretOps::read_secret(&db_index, &reference_hash)
            .map_err(|e| format!("Failed to read secret: {}", e))?;

    // Note: We use 'role' from hash (validated via checksum), not from database

    // Validate OTP if present (only for receiver - sender bypasses OTP)
    if payload.otp.is_some() && provided_otp.is_none() && role == SecretRole::Receiver {
        // OTP required but not provided (receiver only)
        let error_json = json!({
            "error": "OTP_REQUIRED",
            "message": "This secret requires a 9-digit OTP"
        });
        return create_signed_endpoint_response(&error_json, crypto_material)
            .map_err(|e| format!("Failed to create error response: {}", e));
    }

    if let Some(stored_otp) = &payload.otp
        && let Some(provided) = provided_otp
        && stored_otp != provided
    {
        let error_json = json!({
            "error": "INVALID_OTP",
            "message": "Invalid OTP provided"
        });
        return create_signed_endpoint_response(&error_json, crypto_material)
            .map_err(|e| format!("Failed to create error response: {}", e));
    }

    // Convert reference_hash to Base58
    let reference_base58 = bs58::encode(&payload.reference_hash).into_string();

    // Get read_at from tracking table
    let reference_hash_array: [u8; 16] = payload
        .reference_hash
        .as_slice()
        .try_into()
        .map_err(|_| "Invalid reference_hash length".to_string())?;
    let read_at = SharedSecretStorage::get_read_at_from_tracking(&reference_hash_array)
        .map_err(|e| format!("Failed to get read_at: {}", e))?;

    // Include OTP and read_at only for sender (role from hash, not DB)
    let otp_for_response = if role == SecretRole::Sender {
        payload.otp.clone()
    } else {
        None
    };
    let read_at_for_response = if role == SecretRole::Sender {
        read_at
    } else {
        None
    };

    // ============================================================================
    // E2E ENCRYPTION: Encrypt key_material with ECDH for requester
    // ============================================================================

    // 1. Validate requester's public key format
    if requester_public_key_hex.len() != 64 {
        return Err(format!(
            "Invalid requester public key hex length: {} (expected 64)",
            requester_public_key_hex.len()
        ));
    }

    let requester_ed25519_public = hex::decode(requester_public_key_hex)
        .map_err(|e| format!("Failed to decode requester public key hex: {}", e))?;

    if requester_ed25519_public.len() != 32 {
        return Err(format!(
            "Invalid requester public key byte length: {} (expected 32)",
            requester_ed25519_public.len()
        ));
    }

    let requester_ed25519_public_array: [u8; 32] = requester_ed25519_public
        .try_into()
        .map_err(|_| "Failed to convert requester public key to array".to_string())?;

    // 2. Convert requester's Ed25519 public key → X25519 public key
    let requester_x25519_public = ed25519_public_to_x25519(&requester_ed25519_public_array)
        .map_err(|e| format!("Failed to convert requester Ed25519→X25519: {}", e))?;

    // 3. Get backend's per-user X25519 private key
    // Use requester's user_id (from JWT) and pub_key for per-user derivation
    let backend_x25519_private = get_backend_x25519_private_key(user_id_from_jwt, requester_public_key_hex)
        .map_err(|e| format!("Failed to derive backend X25519 private key (per-user): {}", e))?;

    // 4. Encrypt key_material with ECDH
    let encrypted_key_material = encrypt_with_ecdh(
        &payload.key_material,
        &backend_x25519_private,
        &requester_x25519_public,
    )
    .map_err(|e| format!("Failed to encrypt key_material with ECDH: {}", e))?;

    // 5. Encode encrypted data to base64 for JSON response
    let encrypted_secret_base64 = BASE64.encode(&payload.encrypted_secret);
    let encrypted_key_material_base64 = BASE64.encode(&encrypted_key_material);

    // Create response
    let response_data = RetrieveSecretResponse {
        encrypted_secret: encrypted_secret_base64,
        encrypted_key_material: encrypted_key_material_base64,
        sender_email: payload.sender_email,
        receiver_email: payload.receiver_email,
        pending_reads,
        max_reads: payload.max_reads,
        expires_at,
        reference: reference_base58,
        role: role.to_str().to_string(),
        otp: otp_for_response,
        read_at: read_at_for_response,
    };

    let response_json = json!(response_data);

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}
