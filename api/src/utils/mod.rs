pub mod auth;
pub mod email;
pub mod email_templates;
pub mod jwt;
pub mod query;
pub mod random_generator;
pub mod routing;

// Auth functions imported directly in routing.rs
pub use email::send_magic_link_email;
pub use jwt::JwtUtils;
pub use query::parse_query_params;
pub use random_generator::{
    base58_to_seed, generate_otp, generate_random_seed, generate_with_seed, seed_to_base58,
};
pub use routing::route_request_with_req;
