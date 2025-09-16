//! Authentication module
//!
//! Contains business logic for authentication operations:
//! - Magic link generation and validation
//! - JWT token refresh
//! - Authentication types and data structures

pub mod magic_link_gen;
pub mod magic_link_val;
pub mod refresh_token;
pub mod types;

// Re-export commonly used types
pub use types::{ErrorResponse, MagicLinkRequest};

// Re-export main functions
pub use magic_link_gen::generate_magic_link;
pub use magic_link_val::validate_magic_link_secure;
pub use refresh_token::handle_refresh_token;
