//! Database operations for user management
//!
//! Provides CRUD operations for the users table and magic link operations with
//! proper error handling and type safety using Spin's SQLite interface.
//!
//! This module has been refactored into smaller, focused submodules for better
//! maintainability while preserving the original API for backwards compatibility.

pub mod magic_link_ops;
pub mod user_ops;

// Re-export for backwards compatibility
pub use magic_link_ops::MagicLinkOperations;
pub use user_ops::UserOperations;
