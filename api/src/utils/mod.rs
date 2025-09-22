#[cfg(test)]
pub mod argon2_test;
pub mod auth;
pub mod ed25519;
pub mod email;
pub mod jwt;
pub mod jwt_middleware;
pub mod jwt_middleware_auth;
pub mod jwt_middleware_cookies;
pub mod jwt_middleware_core;
pub mod jwt_middleware_errors;
pub mod jwt_middleware_renewal;
pub mod jwt_middleware_types;
pub mod protected_endpoint_middleware;
pub mod query;
pub mod random_generator;
pub mod rate_limiter;
pub mod routing;
pub mod signed_request;
pub mod signed_response;
pub mod validation;

// Auth functions imported directly in routing.rs
pub use email::send_magic_link_email;
pub use jwt::JwtUtils;
pub use protected_endpoint_middleware::{ProtectedEndpointMiddleware, ProtectedEndpointResult};
pub use query::parse_query_params;
pub use random_generator::{
    base58_to_seed, generate_otp, generate_random_seed, generate_with_seed, seed_to_base58,
};
pub use rate_limiter::{check_rate_limit, extract_client_ip, init_rate_limiter};
pub use routing::route_request_with_req;
pub use signed_request::{SignedRequest, SignedRequestValidator};
pub use signed_response::{SignedResponse, SignedResponseGenerator};
pub use validation::{validate_email, validate_length, validate_prefix_suffix};
