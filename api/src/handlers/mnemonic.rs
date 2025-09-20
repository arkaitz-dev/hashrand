use crate::types::CustomHashResponse;
use crate::utils::{
    base58_to_seed, generate_otp, generate_random_seed, query::parse_query_params, seed_to_base58,
    ProtectedEndpointMiddleware, ProtectedEndpointResult,
};
use crate::utils::auth::ErrorResponse;
use crate::utils::protected_endpoint_middleware::{payload_to_params, extract_seed_from_payload};
use bip39::{Language, Mnemonic};
use spin_sdk::http::{Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Handle GET request for mnemonic generation
fn handle_mnemonic_get(req: Request) -> anyhow::Result<Response> {
    // Parse query parameters
    let uri_string = req.uri().to_string();
    let query_string = uri_string.split('?').nth(1).unwrap_or("");
    let params = parse_query_params(query_string);

    // Use shared logic with no provided seed (will generate random seed)
    handle_mnemonic_with_params(params, None)
}

/// Handle POST requests with signed request validation (UNIVERSAL)
pub async fn handle_mnemonic_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request and extract payload (UNIVERSAL)
    let result: ProtectedEndpointResult<serde_json::Value> = match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
        Ok(result) => result,
        Err(error_response) => return Ok(error_response),
    };

    println!("✅ Mnemonic endpoint: validated signed request for user {}", result.user_id);

    // Convert payload to parameter map using UNIVERSAL function
    let params = payload_to_params(&result.payload);

    // Extract seed using UNIVERSAL function
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: e,
                })?)
                .build());
        }
    };

    // Use existing business logic
    handle_mnemonic_with_params(params, provided_seed)
}

/// Handle POST request for mnemonic generation with seed (LEGACY)
fn handle_mnemonic_post(req: Request) -> anyhow::Result<Response> {
    let body = req.body();
    let json_str = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body("Invalid UTF-8 in request body")
                .build());
        }
    };

    let json_value: serde_json::Value = match serde_json::from_str(&json_str) {
        Ok(json) => json,
        Err(_) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body("Invalid JSON in request body")
                .build());
        }
    };

    // Extract required seed parameter
    let seed_str = match json_value.get("seed").and_then(|v| v.as_str()) {
        Some(seed) => seed,
        None => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body("Missing 'seed' field in JSON body")
                .build());
        }
    };

    // Validate and convert seed from base58
    let seed_32 = match base58_to_seed(seed_str) {
        Ok(seed) => seed,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "text/plain")
                .body(format!("Invalid seed: {}", e))
                .build());
        }
    };

    // Extract optional parameters from JSON
    let mut params = HashMap::new();
    if let Some(language) = json_value.get("language").and_then(|v| v.as_str()) {
        params.insert("language".to_string(), language.to_string());
    }
    if let Some(words) = json_value.get("words").and_then(|v| v.as_u64()) {
        params.insert("words".to_string(), words.to_string());
    }

    handle_mnemonic_with_params(params, Some(seed_32))
}

/// Core logic for handling mnemonic generation
fn handle_mnemonic_with_params(
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
