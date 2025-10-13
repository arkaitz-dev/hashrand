//! Shared secret creation endpoint
//!
//! POST /api/shared-secret/create - Create shared secret with dual-URL system
//! Requires JWT authentication and Ed25519 signature validation

use tracing::{info, warn};

use crate::database::operations::{
    shared_secret_crypto::SharedSecretCrypto,
    shared_secret_ops::SharedSecretOps,
    shared_secret_types::{SecretRole, constants::*},
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
///
/// NOTE: receiver_language and sender_language are EXCEPTIONS to the integer
/// encoding policy (see api/src/utils/auth/types.rs module doc).
/// They use ISO 639-1 string codes because rust_i18n requires strings.
#[derive(Debug, Deserialize, Serialize)]
struct CreateSecretRequest {
    sender_email: String,
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
    /// EXCEPTION: Uses ISO string instead of integer (rust_i18n requirement)
    #[serde(default)]
    receiver_language: Option<String>,
    /// EXCEPTION: Uses ISO string instead of integer (rust_i18n requirement)
    #[serde(default)]
    sender_language: Option<String>,
    ui_host: String, // Required: UI hostname for URL generation
}

fn default_expires_hours() -> i64 {
    DEFAULT_EXPIRES_HOURS
}

fn default_max_reads() -> i64 {
    DEFAULT_READS
}

/// Build complete URL with protocol based on hostname
///
/// Logic:
/// - localhost or 127.0.0.1 → http://
/// - Other domains → https://
///
/// # Arguments
/// * `ui_host` - Hostname from frontend (e.g., "localhost", "app.domain.com")
/// * `path` - Path to append (e.g., "?shared=abc123")
///
/// # Returns
/// Complete URL with protocol (e.g., "http://localhost?shared=abc123")
fn build_complete_url(ui_host: &str, path: &str) -> String {
    let base_url = ui_host.trim_end_matches('/');
    let clean_path = path.trim_start_matches('/');

    // Check if protocol is already present
    let url_with_protocol = if base_url.starts_with("http://") || base_url.starts_with("https://") {
        // Protocol already present - use as is
        base_url.to_string()
    } else {
        // No protocol - add appropriate one based on host
        if base_url.contains("localhost") || base_url.contains("127.0.0.1") {
            // Development: use http://
            format!("http://{}", base_url)
        } else {
            // Production/remote: use https://
            format!("https://{}", base_url)
        }
    };

    format!("{}/{}", url_with_protocol, clean_path)
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
    info!("📤 Request to /api/shared-secret/create endpoint");
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
    // Validate sender email
    if validate_email(&request.sender_email).is_err() {
        return Err("Invalid sender email format".to_string());
    }

    // Validate that sender_email matches sender_user_id from JWT (Zero Knowledge verification)
    let calculated_sender_id = SharedSecretCrypto::calculate_user_id(&request.sender_email)
        .map_err(|e| format!("Failed to calculate sender user_id: {}", e))?;

    if calculated_sender_id != *sender_user_id {
        return Err("Sender email does not match authenticated user".to_string());
    }

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

    // ============================================================================
    // NEW ZERO KNOWLEDGE HASH GENERATION (v2.0)
    // ============================================================================

    // Generate random reference hash (shared between sender and receiver)
    let reference_hash = SharedSecretCrypto::generate_reference_hash();

    // Generate 40-byte hashes with Zero Knowledge user_id derivation
    let sender_hash_40 = SharedSecretCrypto::generate_shared_secret_hash(
        &reference_hash,
        &request.sender_email,
        SecretRole::Sender,
    )
    .map_err(|e| format!("Failed to generate sender hash: {}", e))?;

    let receiver_hash_40 = SharedSecretCrypto::generate_shared_secret_hash(
        &reference_hash,
        &request.receiver_email,
        SecretRole::Receiver,
    )
    .map_err(|e| format!("Failed to generate receiver hash: {}", e))?;

    // Encrypt hashes with ChaCha20
    let sender_encrypted = SharedSecretCrypto::encrypt_url_hash(&sender_hash_40)
        .map_err(|e| format!("Failed to encrypt sender hash: {}", e))?;

    let receiver_encrypted = SharedSecretCrypto::encrypt_url_hash(&receiver_hash_40)
        .map_err(|e| format!("Failed to encrypt receiver hash: {}", e))?;

    // Generate db_index for database storage (PRIMARY KEY)
    let sender_db_index = SharedSecretCrypto::generate_db_index(&reference_hash, sender_user_id)
        .map_err(|e| format!("Failed to generate sender db_index: {}", e))?;

    let receiver_db_index =
        SharedSecretCrypto::generate_db_index(&reference_hash, &receiver_user_id)
            .map_err(|e| format!("Failed to generate receiver db_index: {}", e))?;

    // Create secret pair using SharedSecretOps (pass pre-generated reference_hash)
    let _created_reference = SharedSecretOps::create_secret_pair(
        &request.sender_email,
        &request.receiver_email,
        &request.secret_text,
        otp.clone(),
        request.expires_hours,
        request.max_reads,
        &sender_db_index,
        &receiver_db_index,
        &reference_hash, // Pass the already-generated reference_hash
    )
    .map_err(|e| format!("Failed to create secret: {}", e))?;

    // Convert reference_hash to Base58 for response
    let reference_base58 = bs58::encode(&reference_hash).into_string();

    // Generate complete URLs with encrypted hashes (Base58 encoded)
    // Using query parameter format (?shared=hash) for cleaner UX, similar to magic links
    let sender_path = format!("?shared={}", bs58::encode(&sender_encrypted).into_string());
    let receiver_path = format!(
        "?shared={}",
        bs58::encode(&receiver_encrypted).into_string()
    );

    let url_sender = build_complete_url(&request.ui_host, &sender_path);
    let url_receiver = build_complete_url(&request.ui_host, &receiver_path);

    // Log shared secret creation with complete URLs and participants
    info!(
        "🔐 Shared secret created: {} → {} | Sender URL: {} | Receiver URL: {}",
        request.sender_email, request.receiver_email, url_sender, url_receiver
    );

    // Send email to receiver (always)
    let receiver_email_result = crate::utils::email::send_shared_secret_receiver_email(
        &request.receiver_email,
        &url_receiver,
        &reference_base58,
        otp.as_deref(),
        &request.sender_email,
        request.expires_hours,
        request.max_reads,
        request.receiver_language.as_deref(),
    )
    .await;

    if let Err(e) = receiver_email_result {
        // println!("⚠️  Warning: Failed to send receiver email: {}", e);
        warn!("⚠️  Warning: Failed to send receiver email: {}", e);
        // Don't fail the entire operation, just log the error
    }

    // Send email to sender (optional)
    if request.send_copy_to_sender {
        let sender_email_result = crate::utils::email::send_shared_secret_sender_email(
            &request.sender_email,
            &url_sender,
            &reference_base58,
            &request.receiver_email,
            request.expires_hours,
            request.sender_language.as_deref(),
        )
        .await;

        if let Err(e) = sender_email_result {
            // println!("⚠️  Warning: Failed to send sender (copy) email: {}", e);
            warn!("⚠️  Warning: Failed to send sender (copy) email: {}", e);
            // Don't fail the entire operation, just log the error
        }
    }

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
