//! Shared secret creation endpoint
//!
//! POST /api/shared-secret/create - Create shared secret with dual-URL system
//! Requires JWT authentication and Ed25519 signature validation

use crate::database::operations::{
    shared_secret_crypto::SharedSecretCrypto, shared_secret_ops::SharedSecretOps,
    shared_secret_types::constants::*,
};
use crate::utils::{
    CryptoMaterial, ProtectedEndpointMiddleware, ProtectedEndpointResult,
    create_auth_error_response, create_server_error_response, create_signed_endpoint_response,
    extract_crypto_material_from_request, validate_email,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::http::{Request, Response};

/// Request payload for creating a shared secret
#[derive(Debug, Deserialize, Serialize)]
struct CreateSecretRequest {
    receiver_email: String,
    secret_text: String,
    #[serde(default = "default_expires_hours")]
    expires_hours: i64,
    #[serde(default = "default_max_reads")]
    max_reads: i64,
    #[serde(default)]
    require_otp: bool,
    #[serde(default)]
    send_copy_to_sender: bool,
}

fn default_expires_hours() -> i64 {
    DEFAULT_EXPIRES_HOURS
}

fn default_max_reads() -> i64 {
    DEFAULT_READS
}

/// Response payload for created shared secret
#[derive(Debug, Serialize)]
struct CreateSecretResponse {
    url_sender: String,
    url_receiver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    otp: Option<String>,
    reference: String,
}

/// Handle POST /api/shared-secret/create
pub async fn handle_create_secret(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request using protected middleware
    let result: ProtectedEndpointResult<CreateSecretRequest> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    // Extract crypto material for signing response
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Crypto extraction failed: {}",
                e
            )));
        }
    };

    // Extract sender user_id from crypto material
    // Convert Vec<u8> to [u8; USER_ID_LENGTH]
    let mut sender_user_id = [0u8; USER_ID_LENGTH];
    if crypto_material.user_id.len() != USER_ID_LENGTH {
        return Ok(create_auth_error_response("Invalid user_id length in JWT"));
    }
    sender_user_id.copy_from_slice(&crypto_material.user_id);

    // Validate and process request
    match create_shared_secret(&result.payload, &sender_user_id, &crypto_material).await {
        Ok(response) => Ok(response),
        Err(e) => Ok(create_server_error_response(&e)),
    }
}

/// Create shared secret with all validations and email sending
async fn create_shared_secret(
    request: &CreateSecretRequest,
    sender_user_id: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // Validate receiver email
    if validate_email(&request.receiver_email).is_err() {
        return Err("Invalid receiver email format".to_string());
    }

    // Validate secret text length
    let char_count = request.secret_text.chars().count();
    if char_count == 0 {
        return Err("Secret text cannot be empty".to_string());
    }
    if char_count > MAX_TEXT_LENGTH {
        return Err(format!(
            "Secret text exceeds {} characters",
            MAX_TEXT_LENGTH
        ));
    }

    // Validate expiration hours
    if request.expires_hours < MIN_EXPIRES_HOURS || request.expires_hours > MAX_EXPIRES_HOURS {
        return Err(format!(
            "Expiration must be between {} and {} hours",
            MIN_EXPIRES_HOURS, MAX_EXPIRES_HOURS
        ));
    }

    // Validate max reads
    if request.max_reads < MIN_READS || request.max_reads > MAX_READS {
        return Err(format!(
            "Max reads must be between {} and {}",
            MIN_READS, MAX_READS
        ));
    }

    // Calculate receiver user_id
    let receiver_user_id = SharedSecretCrypto::calculate_user_id(&request.receiver_email)
        .map_err(|e| format!("Failed to calculate receiver user_id: {}", e))?;

    // Generate OTP if requested
    let otp = if request.require_otp {
        Some(SharedSecretCrypto::generate_otp())
    } else {
        None
    };

    // Generate unique encrypted IDs for sender and receiver (must be unique for each secret)
    // Using timestamp + random data to ensure uniqueness
    use chrono::Utc;

    let timestamp = Utc::now().timestamp_nanos_opt().unwrap_or(0);
    let random_sender = SharedSecretCrypto::generate_reference_hash();
    let sender_id_material = format!(
        "sender_{:?}_{}_{:?}",
        sender_user_id, timestamp, random_sender
    );
    let sender_id_hash = blake3::hash(sender_id_material.as_bytes());
    let mut sender_id = [0u8; ENCRYPTED_ID_LENGTH];
    sender_id.copy_from_slice(&sender_id_hash.as_bytes()[0..ENCRYPTED_ID_LENGTH]);

    let random_receiver = SharedSecretCrypto::generate_reference_hash();
    let receiver_id_material = format!(
        "receiver_{:?}_{}_{:?}",
        receiver_user_id, timestamp, random_receiver
    );
    let receiver_id_hash = blake3::hash(receiver_id_material.as_bytes());
    let mut receiver_id = [0u8; ENCRYPTED_ID_LENGTH];
    receiver_id.copy_from_slice(&receiver_id_hash.as_bytes()[0..ENCRYPTED_ID_LENGTH]);

    // Get sender email from crypto material (TODO: improve this)
    let sender_email = "sender@example.com"; // Placeholder

    // Create secret pair using SharedSecretOps
    let reference_hash = SharedSecretOps::create_secret_pair(
        sender_email,
        &request.receiver_email,
        &request.secret_text,
        otp.clone(),
        request.expires_hours,
        request.max_reads,
        &sender_id,
        &receiver_id,
    )
    .map_err(|e| format!("Failed to create secret: {}", e))?;

    // Convert reference_hash to Base58
    let reference_base58 = bs58::encode(&reference_hash).into_string();

    // Generate URLs
    let url_sender = format!("/shared-secret/{}", bs58::encode(&sender_id).into_string());
    let url_receiver = format!(
        "/shared-secret/{}",
        bs58::encode(&receiver_id).into_string()
    );

    // TODO: Send emails to receiver (always) and sender (if requested)
    // Will be implemented in email templates phase

    // Create response
    let response_data = CreateSecretResponse {
        url_sender,
        url_receiver,
        otp: otp.clone(),
        reference: reference_base58,
    };

    let response_json = json!(response_data);

    // Create signed response
    create_signed_endpoint_response(&response_json, crypto_material)
}
