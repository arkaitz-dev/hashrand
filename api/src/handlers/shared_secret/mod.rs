//! Shared secret handlers module
//!
//! Provides HTTP handlers for shared secret operations:
//! - POST /api/shared-secret/create - Create new shared secret
//! - GET /api/shared-secret/{hash} - Retrieve secret (with OTP check)
//! - POST /api/shared-secret/{hash} - Retrieve secret with OTP validation
//! - DELETE /api/shared-secret/{hash} - Delete secret
//! - GET /api/shared-secret/confirm-read?hash={hash} - Confirm read by receiver

pub mod creation;
pub mod deletion;
pub mod retrieval;
pub mod tracking;

pub use creation::handle_create_secret;
pub use deletion::handle_delete_secret;
pub use retrieval::handle_retrieve_secret;
pub use tracking::handle_confirm_read;
