//! Database operations for magic link management
//!
//! Provides magic link operations with proper error handling and type safety
//! using Spin's SQLite interface.
//!
//! This module has been refactored into smaller, focused submodules for better
//! maintainability while preserving the original API for backwards compatibility.

pub mod magic_link_crypto;
pub mod magic_link_ops;
pub mod magic_link_storage;
pub mod magic_link_types;
pub mod magic_link_validation;

// Re-export for backwards compatibility
pub use magic_link_ops::MagicLinkOperations;
