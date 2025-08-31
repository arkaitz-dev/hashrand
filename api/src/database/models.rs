//! Database models for HashRand application
//!
//! Defines data structures that represent database entities.

use chrono::{DateTime, Utc};
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

/// Authentication session model for managing user authentication
///
/// This struct represents the auth_sessions table structure for handling
/// JWT token-based authentication with magic link flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    /// Unique session identifier (auto-increment primary key)
    pub id: Option<i64>,

    /// User ID derived from email (Base58 encoded, 32-byte hash)
    pub user_id: String,

    /// User email address for magic link sending (not stored in DB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Magic token for email-based authentication (includes user_id + timestamp + HMAC)
    pub magic_token: String,

    /// JWT access token (15 minutes validity)
    pub access_token: Option<String>,

    /// JWT refresh token (1 week validity)
    pub refresh_token: Option<String>,

    /// Session creation timestamp in Unix seconds
    pub created_at: Option<u64>,

    /// Magic token expiration timestamp in Unix seconds
    pub magic_expires_at: u64,

    /// Access token expiration timestamp in Unix seconds
    pub access_expires_at: Option<u64>,

    /// Refresh token expiration timestamp in Unix seconds
    pub refresh_expires_at: Option<u64>,

    /// Whether the magic token has been used for authentication
    pub is_used: bool,
}

impl AuthSession {
    /// Create a new auth session for magic link generation
    ///
    /// # Arguments
    /// * `email` - User email address (used to derive user_id, not stored)
    /// * `magic_token` - Secure magic token with integrity protection
    /// * `magic_expires_at` - Magic token expiration time
    ///
    /// # Returns
    /// * `Result<AuthSession, String>` - New session instance ready for insertion or error
    pub fn new_magic_link(
        email: String,
        magic_token: String,
        magic_expires_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        // Import JwtUtils here to avoid circular dependencies
        use crate::utils::JwtUtils;
        let user_id = JwtUtils::email_to_username(&email)?;

        Ok(Self {
            id: None,
            user_id,
            email: Some(email), // Keep for magic link generation, won't be stored in DB
            magic_token,
            access_token: None,
            refresh_token: None,
            created_at: None,
            magic_expires_at: magic_expires_at.timestamp() as u64,
            access_expires_at: None,
            refresh_expires_at: None,
            is_used: false,
        })
    }

    /// Update session with JWT tokens after successful magic link validation
    ///
    /// # Arguments
    /// * `access_token` - JWT access token
    /// * `refresh_token` - JWT refresh token  
    /// * `access_expires_at` - Access token expiration time
    /// * `refresh_expires_at` - Refresh token expiration time
    #[allow(dead_code)]
    pub fn activate_tokens(
        &mut self,
        access_token: String,
        refresh_token: String,
        access_expires_at: DateTime<Utc>,
        refresh_expires_at: DateTime<Utc>,
    ) {
        self.access_token = Some(access_token);
        self.refresh_token = Some(refresh_token);
        self.access_expires_at = Some(access_expires_at.timestamp() as u64);
        self.refresh_expires_at = Some(refresh_expires_at.timestamp() as u64);
        self.is_used = true;
    }
}
