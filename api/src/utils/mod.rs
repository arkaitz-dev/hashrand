pub mod auth;
pub mod email;
pub mod jwt;
pub mod query;
pub mod random_generator;
pub mod rate_limiter;
pub mod routing;
pub mod validation;
#[cfg(test)]
pub mod argon2_test;

// Auth functions imported directly in routing.rs
pub use email::send_magic_link_email;
pub use jwt::JwtUtils;
pub use query::parse_query_params;
pub use random_generator::{
    base58_to_seed, generate_otp, generate_random_seed, generate_with_seed, seed_to_base58,
};
pub use routing::route_request_with_req;
pub use rate_limiter::{check_rate_limit, extract_client_ip, init_rate_limiter};
pub use validation::{
    validate_email, validate_length, validate_prefix_suffix, validate_seed_string,
};
