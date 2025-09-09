//! Database models for HashRand application
//!
//! Defines data structures that represent database entities.

use serde::{Deserialize, Serialize};

/// User model representing a user in the database
///
/// This struct represents the users table structure and provides
/// serialization capabilities for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier (auto-increment primary key)
    pub id: Option<i64>,

    /// Username (unique constraint)
    pub username: String,

    /// Email address (unique constraint)
    pub email: String,

    /// Account creation timestamp (ISO 8601 format)
    pub created_at: Option<String>,

    /// Last update timestamp (ISO 8601 format)
    pub updated_at: Option<String>,
}

impl User {
    /// Create a new user instance for insertion
    ///
    /// # Arguments
    /// * `username` - Unique username
    /// * `email` - Unique email address
    ///
    /// # Returns
    /// * `User` - New user instance with None for auto-generated fields
    #[allow(dead_code)]
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: None,
            username,
            email,
            created_at: None,
            updated_at: None,
        }
    }
}

