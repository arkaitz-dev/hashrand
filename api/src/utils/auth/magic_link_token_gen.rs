//! Magic link token generation logic
//!
//! Provides functions for generating encrypted magic tokens and creating
//! secure magic link URLs with proper host determination.

use chrono::{DateTime, Duration, Utc};
use spin_sdk::http::Response;

use super::types::ErrorResponse;
use crate::utils::JwtUtils;

/// Magic link token generation result
pub struct TokenGenerationResult {
    pub magic_token: String,
    pub encryption_blob: [u8; 44],
    pub expires_at_nanos: i64,
    pub magic_expires_at: DateTime<Utc>,
    pub magic_link: String,
}

/// Magic link token generation operations
pub struct MagicLinkTokenGeneration;

impl MagicLinkTokenGeneration {
    /// Generate encrypted magic token with ChaCha20 protection
    ///
    /// # Arguments
    /// * `email` - Email address for token generation
    /// * `duration_minutes` - Token expiration duration in minutes
    ///
    /// # Returns
    /// * `Result<(String, [u8; 44], i64, DateTime<Utc>), Response>` - Token data or error response
    pub fn generate_encrypted_token(
        email: &str,
        duration_minutes: i64,
    ) -> Result<(String, [u8; 44], i64, DateTime<Utc>), Response> {
        let magic_expires_at = Utc::now() + Duration::minutes(duration_minutes);

        match JwtUtils::generate_magic_token_encrypted(email, magic_expires_at) {
            Ok((token, blob, expires_at)) => Ok((token, blob, expires_at, magic_expires_at)),
            Err(e) => Err(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Failed to generate magic token: {}", e),
                    })
                    .unwrap_or_default(),
                )
                .build()),
        }
    }

    /// Determine the appropriate host URL for magic link
    ///
    /// ui_host MUST be provided by frontend - no fallback to request headers.
    /// Request header 'host' points to backend API, not frontend UI.
    ///
    /// # Arguments
    /// * `ui_host` - Required UI host from request payload
    ///
    /// # Returns
    /// * `Result<String, Response>` - Host URL or error response if ui_host is missing
    pub fn determine_host_url(ui_host: Option<&str>) -> Result<String, Response> {
        println!("DEBUG: Validating ui_host");
        println!("DEBUG: ui_host = {:?}", ui_host);

        match ui_host {
            Some(host) => {
                println!("DEBUG: ui_host provided: {}", host);
                Ok(host.to_string())
            }
            None => {
                println!("❌ ERROR: ui_host is required but was not provided by frontend");
                Err(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_string(&crate::utils::auth::types::ErrorResponse {
                            error: "ui_host is required - frontend must provide its URL".to_string(),
                        })
                        .unwrap_or_else(|_| r#"{"error":"ui_host required"}"#.to_string()),
                    )
                    .build())
            }
        }
    }

    /// Create complete magic link URL
    ///
    /// # Arguments
    /// * `host_url` - Base host URL for the magic link
    /// * `magic_token` - Encrypted magic token
    ///
    /// # Returns
    /// * `String` - Complete magic link URL
    pub fn create_magic_link_url(host_url: &str, magic_token: &str) -> String {
        let magic_link = JwtUtils::create_magic_link_url(host_url, magic_token);
        println!("DEBUG: Generated magic_link = {}", magic_link);
        magic_link
    }

    /// Generate complete token generation result
    ///
    /// Combines token generation, host determination, and URL creation
    ///
    /// # Arguments
    /// * `email` - Email address for token generation
    /// * `ui_host` - REQUIRED UI host from request payload (no fallback)
    /// * `duration_minutes` - Token expiration duration in minutes
    ///
    /// # Returns
    /// * `Result<TokenGenerationResult, Response>` - Complete result or error response
    pub fn generate_complete_result(
        email: &str,
        ui_host: Option<&str>,
        duration_minutes: i64,
    ) -> Result<TokenGenerationResult, Response> {
        // Generate encrypted magic token
        let (magic_token, encryption_blob, expires_at_nanos, magic_expires_at) =
            Self::generate_encrypted_token(email, duration_minutes)?;

        // Determine host URL - REQUIRED, returns error if ui_host is None
        let host_url = Self::determine_host_url(ui_host)?;

        // Create magic link URL
        let magic_link = Self::create_magic_link_url(&host_url, &magic_token);

        Ok(TokenGenerationResult {
            magic_token,
            encryption_blob,
            expires_at_nanos,
            magic_expires_at,
            magic_link,
        })
    }
}
