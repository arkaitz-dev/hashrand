//! Magic link generation business logic

use chrono::{Duration, Utc};
use spin_sdk::http::{Request, Response};

use super::types::{ErrorResponse, MagicLinkRequest};
use crate::database::operations::MagicLinkOperations;
use crate::utils::{
    JwtUtils, check_rate_limit, extract_client_ip, send_magic_link_email, validate_email,
};

/// Generate and send magic link for authentication
///
/// This function handles the business logic for magic link generation:
/// - Validates email format and rate limiting
/// - Generates encrypted magic token with ChaCha20
/// - Stores encrypted token in database
/// - Sends email or falls back to console logging
pub async fn generate_magic_link(
    req: &Request,
    magic_request: &MagicLinkRequest,
) -> anyhow::Result<Response> {
    // Check rate limiting for authentication requests
    let client_ip = extract_client_ip(req.headers());
    if let Err(e) = check_rate_limit(&client_ip) {
        return Ok(Response::builder()
            .status(429) // Too Many Requests
            .header("content-type", "application/json")
            .header("retry-after", "900") // 15 minutes in seconds
            .body(serde_json::to_string(&ErrorResponse {
                error: format!("Rate limited: {}", e),
            })?)
            .build());
    }

    // Validate email format (strict validation)
    if let Err(e) = validate_email(&magic_request.email) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: format!("Invalid email: {}", e),
            })?)
            .build());
    }

    // Validate random_hash is provided (required for dual-factor authentication)
    if magic_request.random_hash.is_none() {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Missing random_hash: Dual-factor validation hash is required".to_string(),
            })?)
            .build());
    }

    // Generate encrypted magic token with ChaCha20 protection (15 minutes)
    let magic_expires_at = Utc::now() + Duration::minutes(15);
    let (magic_token, encryption_blob, expires_at_nanos) =
        match JwtUtils::generate_magic_token_encrypted(&magic_request.email, magic_expires_at) {
            Ok((token, blob, expires_at)) => (token, blob, expires_at),
            Err(e) => {
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_string(&ErrorResponse {
                            error: format!("Failed to generate magic token: {}", e),
                        })
                        .unwrap_or_default(),
                    )
                    .build());
            }
        };

    // Get host URL for magic link (prefer ui_host from request, fallback to request host)
    println!("DEBUG: About to choose host URL");
    println!("DEBUG: magic_request.ui_host = {:?}", magic_request.ui_host);

    let fallback_host = JwtUtils::get_host_url_from_request(req);
    println!("DEBUG: fallback_host from request = {}", fallback_host);

    let host_url = magic_request
        .ui_host
        .as_deref() // Más limpio que .as_ref().map(|s| s.as_str())
        .unwrap_or(&fallback_host);

    println!("DEBUG: Final chosen host_url = {}", host_url);
    let magic_link = JwtUtils::create_magic_link_url(host_url, &magic_token);
    println!("DEBUG: Generated magic_link = {}", magic_link);

    // Store encrypted magic token in database with ChaCha20 encryption data and random hash
    match MagicLinkOperations::store_magic_link_encrypted(
        &magic_token,
        &encryption_blob,
        expires_at_nanos,
        magic_request.next.as_deref(),
        magic_request.random_hash.as_deref(),
    ) {
        Ok(_) => {
            // Try to send email via Mailtrap, fallback to console logging
            match send_magic_link_email(
                &magic_request.email,
                &magic_link,
                magic_request.email_lang.as_deref(),
            )
            .await
            {
                Ok(()) => {
                    println!("✅ Email sent successfully to: {}", magic_request.email);
                }
                Err(e) => {
                    println!(
                        "⚠️ Email sending failed, falling back to console logging: {}",
                        e
                    );

                    // Fallback: simulate email content in console (development mode)
                    println!("\n🔗 === EMAIL FALLBACK (DEVELOPMENT MODE) ===");
                    println!("📧 TO: {}", magic_request.email);
                    println!("📬 FROM: HashRand Spin <noreply@hashrand.dev>");
                    println!("📝 SUBJECT: Your Magic Link for HashRand Spin");
                    println!("📄 EMAIL BODY:");
                    println!("──────────────────────────────────────────────────");
                    println!("Hi there!");
                    println!();
                    println!(
                        "You requested access to HashRand Spin. Click the link below to sign in:"
                    );
                    println!();
                    println!("🔗 {}", magic_link);
                    println!();
                    println!(
                        "This link will expire at: {}",
                        magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!();
                    println!("If you didn't request this, you can safely ignore this email.");
                    println!();
                    println!("Best regards,");
                    println!("The HashRand Spin Team");
                    println!("──────────────────────────────────────────────────");
                    println!("🔧 DEVELOPMENT INFO:");
                    println!("   • UI Host: {:?}", magic_request.ui_host);
                    println!("   • Final Host URL: {}", host_url);
                    println!(
                        "   • Token expires: {}",
                        magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!("   • Email send error: {}", e);
                    println!("═══════════════════════════════════════════════════\n");
                }
            }

            // Clean up expired sessions
            let _ = MagicLinkOperations::cleanup_expired_links();

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("access-control-allow-origin", "*")
                .header("access-control-allow-methods", "POST, GET, OPTIONS")
                .header("access-control-allow-headers", "Content-Type")
                .body("{\"status\":\"OK\"}")
                .build())
        }
        Err(e) => {
            println!("Failed to create auth session: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Failed to generate magic link".to_string(),
                })?)
                .build())
        }
    }
}
