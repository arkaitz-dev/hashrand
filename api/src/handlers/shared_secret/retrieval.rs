//! Shared secret retrieval endpoint
//!
//! GET/POST /api/shared-secret/{hash} - Retrieve shared secret with OTP validation
//! Requires JWT authentication and Ed25519 signature validation

use crate::database::operations::{
    shared_secret_ops::SharedSecretOps, shared_secret_types::constants::*,
};
use crate::utils::{
    CryptoMaterial, ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator,
    create_auth_error_response, create_client_error_response, create_server_error_response,
    create_signed_endpoint_response, endpoint_helpers::extract_query_params,
    extract_crypto_material_from_request,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::http::{Method, Request, Response};

/// Request payload for POST with OTP
#[derive(Debug, Deserialize, Serialize)]
struct RetrieveSecretRequest {
    otp: String,
}

/// Response payload for retrieved shared secret
#[derive(Debug, Serialize)]
struct RetrieveSecretResponse {
    secret_text: String,
    sender_email: String,
    receiver_email: String,
    pending_reads: i64,
    max_reads: i64,
    expires_at: i64,
    reference: String,
    role: String,
}

/// Main handler for GET/POST /api/shared-secret/{hash}
pub async fn handle_retrieve_secret(req: Request, hash: &str) -> anyhow::Result<Response> {
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

    // Retrieve secret (without decrementing first, to check for OTP)
    match retrieve_and_respond(&encrypted_id, &user_id, None, &crypto_material) {
        Ok(response) => Ok(response),
        Err(e) => Ok(create_server_error_response(&e)),
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

    // Decode hash
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

    // Retrieve secret with OTP validation
    match retrieve_and_respond(
        &encrypted_id,
        &user_id,
        Some(&result.payload.otp),
        &crypto_material,
    ) {
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

/// Retrieve secret and create response
fn retrieve_and_respond(
    encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
    _user_id: &[u8; USER_ID_LENGTH],
    provided_otp: Option<&str>,
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // TODO: Validate that user_id from JWT matches user_id encrypted in hash
    // For now, just proceed with retrieval

    // Read secret (no decrement - that happens in confirm-read endpoint)
    let (payload, pending_reads, expires_at, role) = SharedSecretOps::read_secret(encrypted_id)
        .map_err(|e| format!("Failed to read secret: {}", e))?;

    // Validate OTP if present
    if payload.otp.is_some() && provided_otp.is_none() {
        // OTP required but not provided
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

    // Create response
    let response_data = RetrieveSecretResponse {
        secret_text: payload.secret_text,
        sender_email: payload.sender_email,
        receiver_email: payload.receiver_email,
        pending_reads,
        max_reads: payload.max_reads,
        expires_at,
        reference: reference_base58,
        role: role.to_str().to_string(),
    };

    let response_json = json!(response_data);

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
        .map_err(|e| format!("Failed to create signed response: {}", e))
}
