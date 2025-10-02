//! Protected Endpoint Helper Functions
//!
//! Utility functions for payload processing

use std::collections::HashMap;

/// Universal function to convert any JSON payload to HashMap for legacy handler compatibility
///
/// # Arguments
/// * `payload` - JSON value to convert
///
/// # Returns
/// * `HashMap<String, String>` - Converted parameters
pub fn payload_to_params(payload: &serde_json::Value) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(obj) = payload.as_object() {
        for (key, value) in obj {
            match value {
                serde_json::Value::String(s) => {
                    params.insert(key.clone(), s.clone());
                }
                serde_json::Value::Number(n) => {
                    params.insert(key.clone(), n.to_string());
                }
                serde_json::Value::Bool(b) => {
                    params.insert(key.clone(), b.to_string());
                }
                _ => {
                    // Skip null, arrays, and objects for now
                }
            }
        }
    }

    params
}

/// Universal function to extract and validate seed from payload (DRY)
///
/// # Arguments
/// * `payload` - JSON value containing optional seed field
///
/// # Returns
/// * `Result<Option<[u8; 32]>, String>` - Validated seed bytes or error
pub fn extract_seed_from_payload(payload: &serde_json::Value) -> Result<Option<[u8; 32]>, String> {
    if let Some(seed_value) = payload.get("seed") {
        if let Some(seed_str) = seed_value.as_str() {
            match crate::utils::base58_to_seed(seed_str) {
                Ok(seed_bytes) => Ok(Some(seed_bytes)),
                Err(e) => Err(format!("Invalid seed format: {}", e)),
            }
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
