//! Middleware for protected endpoints with universal signed request validation
//!
//! Provides JWT token validation + Ed25519 signature verification
//! for all authenticated API endpoints

use serde::{Deserialize, Serialize};
use spin_sdk::http::{Request, Response};
use std::collections::HashMap;

use super::{SignedRequest, SignedRequestValidator};
use crate::utils::auth::ErrorResponse;

/// Universal payload wrapper for protected endpoints
/// CORRECTED: No longer generic since SignedRequest uses Base64-encoded JSON payload
pub type ProtectedSignedRequest = SignedRequest;

/// Protected endpoint middleware result
pub struct ProtectedEndpointResult<T> {
    pub payload: T,
}

/// Protected endpoint middleware
pub struct ProtectedEndpointMiddleware;

/// Universal function to convert any JSON payload to HashMap for legacy handler compatibility
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

impl ProtectedEndpointMiddleware {
    /// Validate protected endpoint request with JWT + Ed25519 signature
    ///
    /// # Arguments
    /// * `req` - HTTP request with Authorization header
    /// * `body_bytes` - Request body as bytes
    ///
    /// # Returns
    /// * `Result<ProtectedEndpointResult<T>, Response>` - Validated payload + JWT claims or error response
    pub async fn validate_request<T>(
        req: &Request,
        body_bytes: &[u8],
    ) -> Result<ProtectedEndpointResult<T>, Response>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        // Step 1: Validate JWT token and extract pub_key
        let (user_id, pub_key_hex) = match Self::extract_jwt_info(req) {
            Ok(info) => info,
            Err(response) => return Err(response),
        };

        // Step 2: Parse signed request from body (now with Base64-encoded JSON payload)
        let signed_request: ProtectedSignedRequest = match serde_json::from_slice(body_bytes) {
            Ok(req) => req,
            Err(e) => {
                println!("🔍 DEBUG: Failed to parse SignedRequest: {}", e);
                return Err(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_string(&ErrorResponse {
                            error: "Invalid SignedRequest structure".to_string(),
                        })
                        .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
                    )
                    .build());
            }
        };

        // Step 3: Validate Ed25519 signature using JWT pub_key (Base64 payload)
        if let Err(e) = SignedRequestValidator::validate_base64_payload(
            &signed_request.payload,
            &signed_request.signature,
            &pub_key_hex,
        ) {
            println!("🔍 DEBUG: SignedRequest validation failed: {}", e);
            return Err(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Invalid signature: {}", e),
                    })
                    .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
                )
                .build());
        }

        println!(
            "✅ Protected endpoint validation successful for user: {}",
            user_id
        );

        // Step 4: Deserialize Base64-encoded JSON payload to typed structure
        let deserialized_payload: T =
            match SignedRequestValidator::deserialize_base64_payload(&signed_request.payload) {
                Ok(payload) => payload,
                Err(e) => {
                    println!("❌ DEBUG: Failed to deserialize Base64 payload: {}", e);
                    return Err(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(
                            serde_json::to_string(&ErrorResponse {
                                error: format!("Invalid payload format: {}", e),
                            })
                            .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
                        )
                        .build());
                }
            };

        println!("✅ Msgpack payload deserialized successfully");

        Ok(ProtectedEndpointResult {
            payload: deserialized_payload,
        })
    }

    /// Extract JWT info (user_id + pub_key_hex) from Authorization header
    fn extract_jwt_info(req: &Request) -> Result<(String, String), Response> {
        // SECURITY: Validate that request doesn't contain both Authorization header AND refresh cookie
        if let Err(e) = crate::utils::validate_no_simultaneous_tokens(req) {
            println!(
                "🚨 [SECURITY VIOLATION] Protected endpoint received request with both tokens"
            );
            return Err(Response::builder()
                .status(403)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse { error: e }).unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()))
                .build());
        }

        // Get Authorization header
        let auth_header = req
            .header("authorization")
            .and_then(|h| h.as_str())
            .ok_or_else(|| {
                Response::builder()
                    .status(401)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_string(&ErrorResponse {
                            error: "Missing Authorization header".to_string(),
                        })
                        .unwrap_or_default(),
                    )
                    .build()
            })?;

        // Extract Bearer token
        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: "Invalid Authorization header format".to_string(),
                    })
                    .unwrap_or_default(),
                )
                .build()
        })?;

        // Validate JWT token and extract claims
        let claims = crate::utils::JwtUtils::validate_access_token(token).map_err(|e| {
            println!("🔍 DEBUG: JWT validation failed: {}", e);
            Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Invalid JWT token: {}", e),
                    })
                    .unwrap_or_default(),
                )
                .build()
        })?;

        // Convert pub_key bytes to hex string
        let pub_key_hex = hex::encode(claims.pub_key);

        // Use username (sub field) as user identifier
        let user_id = claims.sub;

        Ok((user_id, pub_key_hex))
    }
}

/// Helper macro for protected endpoint handlers
///
/// Usage:
/// ```rust
/// protected_endpoint_handler!(handle_custom_protected, CustomPayload, |result, req| {
///     // Your endpoint logic here with result.payload and result.jwt_claims
///     handle_custom_with_params(result.payload.into(), None)
/// });
/// ```
#[macro_export]
macro_rules! protected_endpoint_handler {
    ($handler_name:ident, $payload_type:ty, $logic:expr) => {
        pub async fn $handler_name(
            req: spin_sdk::http::Request,
        ) -> anyhow::Result<spin_sdk::http::Response> {
            use $crate::utils::protected_endpoint_middleware::{
                ProtectedEndpointMiddleware, ProtectedEndpointResult,
            };

            let body_bytes = req.body();

            let result: ProtectedEndpointResult<$payload_type> =
                match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
                    Ok(result) => result,
                    Err(error_response) => return Ok(error_response),
                };

            let logic_fn: fn(
                ProtectedEndpointResult<$payload_type>,
                &spin_sdk::http::Request,
            ) -> anyhow::Result<spin_sdk::http::Response> = $logic;
            logic_fn(result, &req)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_jwt_public_key_extractor() {
    //     let extractor = JwtPublicKeyExtractor {
    //         public_key: "test_key_123".to_string(),
    //     };

    //     let result = extractor.extract_public_key().unwrap();
    //     assert_eq!(result, "test_key_123");
    // }
}
