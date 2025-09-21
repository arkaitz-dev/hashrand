//! Magic link token generation logic
//!
//! Provides functions for generating encrypted magic tokens and creating
//! secure magic link URLs with proper host determination.

use chrono::{DateTime, Duration, Utc};
use spin_sdk::http::{Request, Response};

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
    /// Prefers ui_host from request if provided, otherwise falls back to request host
    ///
    /// # Arguments
    /// * `req` - HTTP request to extract fallback host from
    /// * `ui_host` - Optional UI host from request payload
    ///
    /// # Returns
    /// * `String` - The determined host URL
    pub fn determine_host_url(req: &Request, ui_host: Option<&str>) -> String {
        println!("DEBUG: About to choose host URL");
        println!("DEBUG: ui_host = {:?}", ui_host);

        let fallback_host = JwtUtils::get_host_url_from_request(req);
        println!("DEBUG: fallback_host from request = {}", fallback_host);

        let host_url = ui_host.unwrap_or(&fallback_host);
        println!("DEBUG: Final chosen host_url = {}", host_url);

        host_url.to_string()
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
    /// * `req` - HTTP request for host determination
    /// * `email` - Email address for token generation
    /// * `ui_host` - Optional UI host from request payload
    /// * `duration_minutes` - Token expiration duration in minutes
    ///
    /// # Returns
    /// * `Result<TokenGenerationResult, Response>` - Complete result or error response
    pub fn generate_complete_result(
        req: &Request,
        email: &str,
        ui_host: Option<&str>,
        duration_minutes: i64,
    ) -> Result<TokenGenerationResult, Response> {
        // Generate encrypted magic token
        let (magic_token, encryption_blob, expires_at_nanos, magic_expires_at) =
            Self::generate_encrypted_token(email, duration_minutes)?;

        // Determine host URL
        let host_url = Self::determine_host_url(req, ui_host);

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