//! Protected Endpoint Macro Definitions
//!
//! Helper macros for protected endpoint handlers

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
            use $crate::utils::protected_endpoint::{
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
