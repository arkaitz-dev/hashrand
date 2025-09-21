use crate::types::CustomHashResponse;
use crate::utils::auth::ErrorResponse;
use crate::utils::protected_endpoint_middleware::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator, generate_otp,
    generate_random_seed, seed_to_base58,
};
use bip39::{Language, Mnemonic};
use spin_sdk::http::{Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// External crates
extern crate hex;

/// Handle mnemonic requests (GET and POST)
pub async fn handle_mnemonic_request(req: Request) -> anyhow::Result<Response> {
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

/// Handle GET request for mnemonic generation with Ed25519 signature validation
fn handle_mnemonic_get(req: Request) -> anyhow::Result<Response> {
    // Extract and validate Bearer token to get public key
    let auth_header = req
        .header("authorization")
        .and_then(|h| h.as_str())
        .unwrap_or("");

    if !auth_header.starts_with("Bearer ") {
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Missing or invalid Authorization header".to_string(),
            })?)
            .build());
    }

    let access_token = &auth_header[7..]; // Remove "Bearer " prefix
    let claims = match crate::utils::jwt::tokens::validate_access_token(access_token) {
        Ok(claims) => claims,
        Err(e) => {
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: format!("Invalid access token: {}", e),
                })?)
                .build());
        }
    };

    // Convert public key to hex string for signature validation
    let public_key_hex = hex::encode(claims.pub_key);

    // Extract query parameters from request URI
    let uri_string = req.uri().to_string();
    let query_string = uri_string.split('?').nth(1).unwrap_or("");

    let mut params: HashMap<String, String> = query_string
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();

    // Validate Ed25519 signature
    if let Err(e) = SignedRequestValidator::validate_query_params(&mut params, &public_key_hex) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: format!("Signature validation failed: {}", e),
            })?)
            .build());
    }

    println!(
        "✅ Mnemonic GET: Ed25519 signature validated for user {}",
        claims.sub
    );

    // Use shared logic with no provided seed (will generate random seed)
    handle_mnemonic_with_params(params, None)
}

/// Handle POST requests with signed request validation (UNIVERSAL)
pub async fn handle_mnemonic_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request and extract payload (UNIVERSAL)
    let result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    println!(
        "✅ Mnemonic endpoint: validated signed request for user {}",
        result.user_id
    );

    // Convert payload to parameter map using UNIVERSAL function
    let params = payload_to_params(&result.payload);

    // Extract seed using UNIVERSAL function
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse { error: e })?)
                .build());
        }
    };

    // Use existing business logic
    handle_mnemonic_with_params(params, provided_seed)
}

/// Core logic for handling mnemonic generation
pub fn handle_mnemonic_with_params(
    params: HashMap<String, String>,
    provided_seed: Option<[u8; 32]>,
) -> anyhow::Result<Response> {
    // Parse language parameter
    let language = match params.get("language") {
        Some(lang_str) => match parse_language(lang_str) {
            Ok(lang) => lang,
            Err(e) => {
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "text/plain")
                    .body(e.to_string())
                    .build());
            }
        },
        None => Language::English,
    };

    // Parse words parameter (12 or 24)
    let words = match params.get("words") {
        Some(words_str) => match words_str.parse::<u32>() {
            Ok(12) => 12,
            Ok(24) => 24,
            Ok(other) => {
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "text/plain")
                    .body(format!(
                        "Invalid words parameter: {}. Only 12 and 24 are supported.",
                        other
                    ))
                    .build());
            }
            Err(_) => {
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "text/plain")
                    .body("Invalid words parameter. Must be 12 or 24.")
                    .build());
            }
        },
        None => 12, // Default to 12 words
    };

    // Use provided seed or generate random one
    let seed_32 = provided_seed.unwrap_or_else(generate_random_seed);

    // Convert seed to base58 for response
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

    // Generate OTP from seed
    let otp = generate_otp(seed_32);

    // Get current timestamp
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // Create JSON response
    let response = CustomHashResponse::new(mnemonic_phrase, seed_base58, otp, timestamp);
    let json_body = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}

/// Parse language string to BIP39 Language enum
fn parse_language(lang: &str) -> anyhow::Result<Language> {
    match lang.to_lowercase().as_str() {
        "english" | "en" => Ok(Language::English),
        "spanish" | "es" => Ok(Language::Spanish),
        "french" | "fr" => Ok(Language::French),
        "portuguese" | "pt" => Ok(Language::Portuguese),
        "japanese" | "ja" => Ok(Language::Japanese),
        "chinese" | "zh" | "chinese-simplified" => Ok(Language::SimplifiedChinese),
        "chinese-traditional" | "zh-tw" => Ok(Language::TraditionalChinese),
        "italian" | "it" => Ok(Language::Italian),
        "korean" | "ko" => Ok(Language::Korean),
        "czech" | "cs" => Ok(Language::Czech),
        _ => Err(anyhow::anyhow!(
            "Unsupported language: {}. Supported: english, spanish, french, portuguese, japanese, chinese, chinese-traditional, italian, korean, czech",
            lang
        )),
    }
}
