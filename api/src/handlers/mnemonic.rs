use crate::types::{CustomHashResponse, MnemonicLanguage};
use crate::utils::protected_endpoint::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, create_error_response,
    create_signed_endpoint_response, extract_crypto_material_from_request, generate_otp,
    generate_random_seed, handle_signed_get_request, seed_to_base58,
};
use bip39::{Language, Mnemonic};
use spin_sdk::http::{Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

// External crates
extern crate hex;

/// Handle mnemonic requests (GET and POST)
pub async fn handle_mnemonic_request(req: Request) -> anyhow::Result<Response> {
    info!("📝 Request to /api/mnemonic endpoint");
    match req.method() {
        spin_sdk::http::Method::Get => handle_mnemonic_get(req),
        spin_sdk::http::Method::Post => handle_mnemonic_post_signed(req).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET requests with SignedResponse (DRY implementation)
fn handle_mnemonic_get(req: Request) -> anyhow::Result<Response> {
    handle_signed_get_request(&req, generate_mnemonic_signed)
}

/// Handle POST requests with signed request validation
pub async fn handle_mnemonic_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request using protected middleware
    let result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    // Extract crypto material for signing response
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_error_response(
                401,
                &format!("Crypto extraction failed: {}", e),
            ));
        }
    };

    // Convert payload to parameters and extract seed
    let mut params = payload_to_params(&result.payload);
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => return Ok(create_error_response(400, &e)),
    };

    // Use provided seed or generate fresh one
    if provided_seed.is_none() && !params.contains_key("seed") {
        let seed = generate_random_seed();
        params.insert("seed".to_string(), seed_to_base58(&seed));
    }

    // Generate mnemonic and create signed response
    generate_mnemonic_signed(&params, &crypto_material)
}

// DELETED: Legacy function handle_mnemonic_with_params removed - was completely unused legacy code

/// Generate secure mnemonic and return SignedResponse (DRY implementation)
fn generate_mnemonic_signed(
    params: &HashMap<String, String>,
    crypto_material: &crate::utils::CryptoMaterial,
) -> anyhow::Result<Response> {
    // Parse language parameter (integer: 0=english, 1=spanish, ..., 9=czech)
    let language = if let Some(lang_str) = params.get("language") {
        match lang_str.parse::<u8>() {
            Ok(index) => match MnemonicLanguage::try_from(index) {
                Ok(mnemonic_lang) => Language::from(mnemonic_lang),
                Err(_) => {
                    return Ok(create_error_response(
                        400,
                        "Invalid language index. Valid range: 0-9 (0=english, 1=spanish, 2=french, 3=portuguese, 4=japanese, 5=chinese-simplified, 6=chinese-traditional, 7=italian, 8=korean, 9=czech)",
                    ));
                }
            },
            Err(_) => {
                return Ok(create_error_response(
                    400,
                    "Invalid language parameter. Must be integer 0-9 (0=english, 1=spanish, 2=french, 3=portuguese, 4=japanese, 5=chinese-simplified, 6=chinese-traditional, 7=italian, 8=korean, 9=czech)",
                ));
            }
        }
    } else {
        Language::English // Default
    };

    // Parse words parameter (12 or 24)
    let words = match params.get("words") {
        Some(words_str) => match words_str.parse::<u32>() {
            Ok(12) => 12,
            Ok(24) => 24,
            Ok(other) => {
                return Ok(create_error_response(
                    400,
                    &format!(
                        "Invalid words parameter: {}. Only 12 and 24 are supported.",
                        other
                    ),
                ));
            }
            Err(_) => {
                return Ok(create_error_response(
                    400,
                    "Invalid words parameter. Must be 12 or 24.",
                ));
            }
        },
        None => 12, // Default to 12 words
    };

    // Get or generate seed
    let seed_32 = if let Some(seed_str) = params.get("seed") {
        crate::utils::base58_to_seed(seed_str)
            .map_err(|e| anyhow::anyhow!("Invalid seed: {}", e))?
    } else {
        generate_random_seed()
    };

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate mnemonic based on requested word count
    let mnemonic = match words {
        12 => {
            // Use first 16 bytes of the 32-byte seed for 12-word mnemonic (128 bits entropy)
            let entropy_16: [u8; 16] = seed_32[0..16]
                .try_into()
                .map_err(|_| anyhow::anyhow!("Failed to extract 16 bytes from seed"))?;
            Mnemonic::from_entropy_in(language, &entropy_16)
                .map_err(|e| anyhow::anyhow!("Failed to generate 12-word mnemonic: {}", e))?
        }
        24 => {
            // Use full 32 bytes for 24-word mnemonic (256 bits entropy)
            Mnemonic::from_entropy_in(language, &seed_32)
                .map_err(|e| anyhow::anyhow!("Failed to generate 24-word mnemonic: {}", e))?
        }
        _ => unreachable!("Words parameter already validated to be 12 or 24"),
    };

    // Convert mnemonic to string (12 or 24 words separated by spaces)
    let mnemonic_phrase = mnemonic.to_string();

    // Generate OTP and timestamp
    let otp = generate_otp(seed_32);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow::anyhow!("Time error: {}", e))?
        .as_secs();

    // Create payload directly
    let payload = CustomHashResponse::new(mnemonic_phrase, seed_base58, otp, timestamp);

    // Create signed response using DRY helper
    match create_signed_endpoint_response(payload, crypto_material) {
        Ok(signed_response) => Ok(signed_response),
        Err(e) => Ok(create_error_response(
            500,
            &format!("Failed to create signed response: {}", e),
        )),
    }
}

// DELETED: Legacy parse_language() function removed - replaced with MnemonicLanguage::try_from() for DRY
