//! Database operations for magic link and shared secret management
//!
//! Provides operations with proper error handling and type safety
//! using Spin's SQLite interface.
//!
//! This module has been refactored into smaller, focused submodules for better
//! maintainability while preserving the original API for backwards compatibility.

// Magic link operations
pub mod magic_link_crypto;
pub mod magic_link_ops;
pub mod magic_link_storage;
pub mod magic_link_types;
pub mod magic_link_validation;

// Shared secret operations
pub mod shared_secret_crypto;
pub mod shared_secret_ops;
pub mod shared_secret_storage;
pub mod shared_secret_types;

// User private key context operations
pub mod user_privkey_ops;

// Re-export for backwards compatibility
pub use magic_link_ops::MagicLinkOperations;
