//! User public keys endpoints (Sistema B - E2EE)
//!
//! Handles publication and retrieval of permanent Ed25519/X25519 public keys
//! for user-to-user end-to-end encryption.
//!
//! Endpoints:
//! - POST /api/keys/rotate - Publish/update user's permanent public keys
//! - GET /api/user/keys/ - Retrieve public keys for a target user

use crate::database::operations::UserKeysOperations;
use crate::utils::protected_endpoint::ProtectedEndpointMiddleware;
use crate::utils::signed_request::SignedRequestValidator;
use crate::utils::signed_response::SignedResponseGenerator;
use crate::utils::{
    create_auth_error_response, create_client_error_response, extract_crypto_material_from_request,
    ProtectedEndpointResult,
};
use serde::{Deserialize, Serialize};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use tracing::{debug, error, info};

/// Main entry point for /api/keys/* endpoints
pub async fn handle_keys_request(
    req: Request,
    _query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    match *req.method() {
        Method::Post => handle_keys_rotate(req).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Main entry point for /api/user/keys/ endpoint (GET only)
pub async fn handle_user_keys_request(
    req: Request,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    match *req.method() {
        Method::Get => handle_user_keys_get(req, query_params).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle POST /api/keys/rotate - Publish/update permanent public keys
///
/// JWT + Sistema A (temporary keys) authentication required
/// Payload contains Sistema B (permanent keys) to publish
async fn handle_keys_rotate(req: Request) -> anyhow::Result<Response> {
    info!("🔑 Request to POST /api/keys/rotate");

    let body_bytes = req.body();

    // Validate signed request using protected middleware (JWT + Sistema A)
    let result: ProtectedEndpointResult<KeysRotatePayload> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    // Extract crypto material (user_id, pub_key_hex) from JWT
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Crypto extraction failed: {}",
                e
            )));
        }
    };

    // Convert user_id to [u8; 16]
    let user_id_array: [u8; 16] = crypto_material.user_id.as_slice().try_into().map_err(|_| {
        anyhow::anyhow!(
            "Invalid user_id length: expected 16 bytes, got {}",
            crypto_material.user_id.len()
        )
    })?;

    debug!("🔐 Payload validated, ed25519_pub={}..., x25519_pub={}...",
           &result.payload.ed25519_pub[..16], &result.payload.x25519_pub[..16]);

    // Validate hex lengths
    if result.payload.ed25519_pub.len() != 64 {
        error!("❌ Invalid ed25519_pub length: {}", result.payload.ed25519_pub.len());
        return Ok(create_client_error_response(&format!(
            "Invalid ed25519_pub length: expected 64 hex chars, got {}",
            result.payload.ed25519_pub.len()
        )));
    }
    if result.payload.x25519_pub.len() != 64 {
        error!("❌ Invalid x25519_pub length: {}", result.payload.x25519_pub.len());
        return Ok(create_client_error_response(&format!(
            "Invalid x25519_pub length: expected 64 hex chars, got {}",
            result.payload.x25519_pub.len()
        )));
    }

    debug!("✅ Hex lengths validated (both 64 chars)");

    // Insert/update user entry
    if let Err(e) = UserKeysOperations::insert_or_update_user(&user_id_array) {
        error!("Failed to insert/update user: {}", e);
        return Ok(create_auth_error_response(&format!(
            "Database error: {}",
            e
        )));
    }

    // Insert Ed25519 public key (idempotent)
    if let Err(e) =
        UserKeysOperations::insert_ed25519_key(&user_id_array, &result.payload.ed25519_pub)
    {
        error!("Failed to insert Ed25519 key: {}", e);
        return Ok(create_auth_error_response(&format!(
            "Database error: {}",
            e
        )));
    }

    // Insert X25519 public key (idempotent)
    if let Err(e) =
        UserKeysOperations::insert_x25519_key(&user_id_array, &result.payload.x25519_pub)
    {
        error!("Failed to insert X25519 key: {}", e);
        return Ok(create_auth_error_response(&format!(
            "Database error: {}",
            e
        )));
    }

    debug!("✅ Keys published successfully for user");

    // Create success response
    let response_payload = KeysRotateResponse {
        success: true,
        message: "Keys published successfully".to_string(),
    };

    // Sign response with Sistema A (temporary keys)
    match SignedResponseGenerator::create_signed_response(
        response_payload,
        &user_id_array,
        &crypto_material.pub_key_hex,
    ) {
        Ok(signed_response) => {
            let json = serde_json::to_string(&signed_response)?;
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json)
                .build())
        }
        Err(e) => {
            error!("Failed to create signed response: {}", e);
            Ok(create_auth_error_response(&format!(
                "Failed to sign response: {}",
                e
            )))
        }
    }
}

/// Payload for POST /api/keys/rotate
#[derive(Debug, Deserialize, Serialize)]
struct KeysRotatePayload {
    /// Ed25519 public key (Sistema B - permanent) as hex string (64 chars)
    ed25519_pub: String,
    /// X25519 public key (Sistema B - permanent) as hex string (64 chars)
    x25519_pub: String,
}

/// Response for POST /api/keys/rotate
#[derive(Debug, Serialize)]
struct KeysRotateResponse {
    success: bool,
    message: String,
}

/// Handle GET /api/user/keys/ - Retrieve public keys for a target user
///
/// JWT + Sistema A (temporary keys) authentication required
/// Query params: target_user (hex), signature (base58)
async fn handle_user_keys_get(
    req: Request,
    mut query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    info!("🔍 Request to GET /api/user/keys/");

    // Extract crypto material (user_id, pub_key_hex) from JWT (requester)
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Crypto extraction failed: {}",
                e
            )));
        }
    };

    // Validate signed query params (Sistema A)
    if let Err(e) =
        SignedRequestValidator::validate_query_params(&mut query_params, &crypto_material.pub_key_hex)
    {
        error!("Query param signature validation failed: {}", e);
        return Ok(create_auth_error_response(&format!(
            "Signature validation failed: {}",
            e
        )));
    }

    // Extract target_user from query params
    let target_user_hex = match query_params.get("target_user") {
        Some(user) => {
            debug!("📋 target_user query param extracted: {}", user);
            user
        }
        None => {
            error!("❌ Missing target_user query parameter");
            return Ok(create_client_error_response(
                "Missing required query parameter: target_user",
            ));
        }
    };

    debug!("🔍 Decoding target_user hex to bytes");
    // Decode target_user_hex to [u8; 16]
    let target_user_bytes = match hex::decode(target_user_hex) {
        Ok(bytes) if bytes.len() == 16 => {
            let mut arr = [0u8; 16];
            arr.copy_from_slice(&bytes);
            arr
        }
        Ok(bytes) => {
            return Ok(create_client_error_response(&format!(
                "Invalid target_user length: expected 32 hex chars (16 bytes), got {} hex chars ({} bytes)",
                target_user_hex.len(),
                bytes.len()
            )));
        }
        Err(e) => {
            return Ok(create_client_error_response(&format!(
                "Invalid target_user hex format: {}",
                e
            )));
        }
    };

    // Get public keys from database (latest 5 of each type)
    let (ed25519_keys, x25519_keys) = match UserKeysOperations::get_user_keys(&target_user_bytes, 5)
    {
        Ok(keys) => keys,
        Err(e) => {
            error!("Failed to get user keys: {}", e);
            return Ok(create_auth_error_response(&format!(
                "Database error: {}",
                e
            )));
        }
    };

    debug!(
        "✅ Retrieved {} Ed25519 keys and {} X25519 keys for target user",
        ed25519_keys.len(),
        x25519_keys.len()
    );

    // Create response payload
    let response_payload = UserKeysResponse {
        user_id: target_user_hex.clone(),
        ed25519_keys: ed25519_keys
            .into_iter()
            .map(|k| PublicKeyInfo {
                pub_key: k.pub_key,
                created_at: k.created_at,
            })
            .collect(),
        x25519_keys: x25519_keys
            .into_iter()
            .map(|k| PublicKeyInfo {
                pub_key: k.pub_key,
                created_at: k.created_at,
            })
            .collect(),
    };

    // Convert requester user_id to [u8; 16] for signing
    let requester_user_id_array: [u8; 16] =
        crypto_material.user_id.as_slice().try_into().map_err(|_| {
            anyhow::anyhow!(
                "Invalid user_id length: expected 16 bytes, got {}",
                crypto_material.user_id.len()
            )
        })?;

    // Sign response with Sistema A (requester's temporary keys)
    match SignedResponseGenerator::create_signed_response(
        response_payload,
        &requester_user_id_array,
        &crypto_material.pub_key_hex,
    ) {
        Ok(signed_response) => {
            let json = serde_json::to_string(&signed_response)?;
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json)
                .build())
        }
        Err(e) => {
            error!("Failed to create signed response: {}", e);
            Ok(create_auth_error_response(&format!(
                "Failed to sign response: {}",
                e
            )))
        }
    }
}

/// Response for GET /api/user/keys/
#[derive(Debug, Serialize)]
struct UserKeysResponse {
    user_id: String,
    ed25519_keys: Vec<PublicKeyInfo>,
    x25519_keys: Vec<PublicKeyInfo>,
}

/// Public key with timestamp
#[derive(Debug, Serialize)]
struct PublicKeyInfo {
    pub_key: String,
    created_at: i64,
}
