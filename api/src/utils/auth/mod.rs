//! Authentication module
//!
//! Contains business logic for authentication operations:
//! - Magic link generation and validation
//! - JWT token refresh
//! - Authentication types and data structures

pub mod magic_link_auth_response_builder;
pub mod magic_link_email_delivery;
pub mod magic_link_gen;
pub mod magic_link_jwt_generator;
pub mod magic_link_request_parser;
pub mod magic_link_request_validation;
pub mod magic_link_response_builder;
pub mod magic_link_signature_validator;
pub mod magic_link_token_gen;
pub mod magic_link_token_processor;
pub mod magic_link_val;
pub mod refresh_token;
pub mod types;

// Re-export commonly used types
pub use types::{ErrorResponse, MagicLinkSignedRequest};

// Re-export main functions
pub use magic_link_gen::generate_magic_link_signed;
pub use magic_link_val::validate_magic_link_secure;
pub use refresh_token::handle_refresh_token;
