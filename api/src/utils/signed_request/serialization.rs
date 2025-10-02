//! Deterministic serialization utilities for consistent signing
//!
//! Provides Base64 and JSON utilities for creating identical
//! serializations between frontend and backend

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Decode Base64 URL-safe payload back to original JSON string
///
/// # Arguments
/// * `base64_payload` - Base64 URL-safe encoded JSON string
///
/// # Returns
/// * `Result<String, String>` - Original JSON string or error
pub fn decode_payload_base64(base64_payload: &str) -> Result<String, String> {
    // Convert Base64 URL-safe to standard Base64
    let base64_standard = base64_payload.replace('-', "+").replace('_', "/");

    // Add padding if needed
    let padding_len = (4 - (base64_standard.len() % 4)) % 4;
    let base64_padded = format!("{}{}", base64_standard, "=".repeat(padding_len));

    // Decode Base64 to bytes
    let bytes = general_purpose::STANDARD
        .decode(&base64_padded)
        .map_err(|e| format!("Base64 decoding failed: {}", e))?;

    // Convert bytes to UTF-8 string
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 conversion failed: {}", e))
}

/// Deserialize JSON payload back to typed structure
///
/// First decodes Base64, then parses JSON
///
/// # Arguments
/// * `base64_payload` - Base64 URL-safe encoded JSON string
///
/// # Returns
/// * `Result<T, String>` - Deserialized payload or error
pub fn deserialize_base64_payload<T>(base64_payload: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    // Step 1: Decode Base64 to original JSON string
    let json_string = decode_payload_base64(base64_payload)?;

    // Step 2: Parse JSON to typed structure
    serde_json::from_str(&json_string).map_err(|e| format!("JSON deserialization failed: {}", e))
}

/// Deterministic JSON serialization for consistent signing
///
/// Creates identical JSON strings between frontend and backend
pub fn serialize_payload_deterministic<T>(payload: &T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    // First serialize to Value to manipulate structure
    let value = serde_json::to_value(payload)?;

    // Sort keys recursively
    let sorted_value = sort_json_keys(value);

    // Serialize to string with no whitespace (compact)
    serde_json::to_string(&sorted_value)
}

/// Deterministic query parameters serialization
///
/// Converts HashMap to sorted JSON string for consistent signing
pub fn serialize_query_params_deterministic(
    params: &std::collections::HashMap<String, String>,
) -> Result<String, serde_json::Error> {
    // Convert HashMap to JSON Value
    let mut json_map = serde_json::Map::new();
    for (key, value) in params {
        json_map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }
    let value = serde_json::Value::Object(json_map);

    // Sort keys recursively and serialize
    let sorted_value = sort_json_keys(value);
    serde_json::to_string(&sorted_value)
}

/// Recursively sort JSON object keys for deterministic serialization
pub fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();

            // Sort keys and recursively process values
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();

            for key in keys {
                if let Some(val) = map.get(key) {
                    sorted_map.insert(key.clone(), sort_json_keys(val.clone()));
                }
            }

            Value::Object(sorted_map)
        }
        Value::Array(array) => {
            // Recursively sort array elements (but preserve order)
            Value::Array(array.into_iter().map(sort_json_keys).collect())
        }
        other => other, // Primitives remain unchanged
    }
}
