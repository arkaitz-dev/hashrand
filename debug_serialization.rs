use serde_json::{json, Value};
use std::collections::HashMap;

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

/// Deterministic JSON serialization (matching frontend sortObjectKeys)
pub fn serialize_payload_deterministic<T>(payload: &T) -> Result<String, serde_json::Error>
where
    T: serde::Serialize,
{
    // First serialize to Value to manipulate structure
    let value = serde_json::to_value(payload)?;

    // Sort keys recursively
    let sorted_value = sort_json_keys(value);

    // Serialize to string with no whitespace (compact)
    serde_json::to_string(&sorted_value)
}

fn main() {
    println!("🔍 Backend JSON Serialization Test");
    println!("=====================================");

    // Test 1: Magic link payload
    println!("\n[1] Magic link payload");
    let magic_payload = json!({
        "magiclink": "8ukaMHhcnJJSEePzD5UYaoHgWib1tr8rS6ms73pC985s"
    });
    println!("Input: {}", magic_payload);
    let serialized = serialize_payload_deterministic(&magic_payload).unwrap();
    println!("Serialized: {}", serialized);
    println!("Length: {}", serialized.len());

    // Test 2: Empty object
    println!("\n[2] Empty object");
    let empty_payload = json!({});
    println!("Input: {}", empty_payload);
    let serialized = serialize_payload_deterministic(&empty_payload).unwrap();
    println!("Serialized: {}", serialized);
    println!("Length: {}", serialized.len());

    // Test 3: Login payload
    println!("\n[3] Login payload");
    let login_payload = json!({
        "email": "me@arkaitz.dev",
        "ui_host": "http://localhost:5173",
        "next": "/",
        "email_lang": "en",
        "pub_key": "abc123"
    });
    println!("Input: {}", login_payload);
    let serialized = serialize_payload_deterministic(&login_payload).unwrap();
    println!("Serialized: {}", serialized);
    println!("Length: {}", serialized.len());

    // Test 4: Nested object
    println!("\n[4] Nested object");
    let nested_payload = json!({
        "z_field": "last",
        "a_field": "first",
        "nested": {
            "z_nested": "nested_last",
            "a_nested": "nested_first"
        }
    });
    println!("Input: {}", nested_payload);
    let serialized = serialize_payload_deterministic(&nested_payload).unwrap();
    println!("Serialized: {}", serialized);
    println!("Length: {}", serialized.len());
}