//! JWT middleware authentication logic - Bearer token validation and auth flow

use crate::utils::JwtUtils;
use crate::utils::jwt::config::get_refresh_token_duration_minutes;
use chrono::{DateTime, Utc};
use spin_sdk::http::{Request, Response};

use super::jwt_middleware_cookies::extract_refresh_token_from_cookies;
use super::jwt_middleware_errors::{create_auth_error_response, create_dual_expiry_response};
use super::jwt_middleware_renewal::check_proactive_renewal;
use super::jwt_middleware_types::{AuthContext, RenewedTokens};

/// Extract and validate Bearer token from Authorization header
///
/// # Arguments
/// * `req` - HTTP request to check for Authorization header
///
/// # Returns
/// * `Result<AuthContext, Response>` - Either valid auth context or error response
pub fn validate_bearer_token(req: &Request) -> Result<AuthContext, Response> {
    // Extract Authorization header
    let auth_header = match req.header("authorization") {
        Some(header) => header.as_str().unwrap_or(""),
        None => {
            return Err(create_auth_error_response(
                "Missing Authorization header. Include 'Authorization: Bearer <token>'",
                None,
            ));
        }
    };

    // Check Bearer token format
    if !auth_header.starts_with("Bearer ") {
        return Err(create_auth_error_response(
            "Invalid Authorization header format. Use 'Bearer <token>'",
            None,
        ));
    }

    // Extract token (skip "Bearer " prefix)
    let token = &auth_header[7..];
    if token.is_empty() {
        return Err(create_auth_error_response(
            "Empty Bearer token provided",
            None,
        ));
    }

    // Validate JWT token
    match JwtUtils::validate_access_token(token) {
        Ok(claims) => {
            let now = Utc::now().timestamp();
            let refresh_expires_at = claims.refresh_expires_at;

            // Check if we need proactive renewal (2/3 threshold)
            // Extract cryptographic information for signed responses
            let user_id = bs58::decode(&claims.sub).into_vec().map_err(|_| {
                println!("🔍 DEBUG: Failed to decode Base58 username from access token");
                create_auth_error_response("Invalid username format", None)
            })?;
            let pub_key_hex = hex::encode(claims.pub_key);

            let renewed_tokens = check_proactive_renewal(
                &claims.sub,
                refresh_expires_at,
                now,
                user_id,
                pub_key_hex,
            )?;

            Ok(AuthContext {
                username: claims.sub,
                expires_at: claims.exp,
                refresh_expires_at,
                renewed_tokens,
            })
        }
        Err(error_msg) => {
            println!("🔍 DEBUG: Token validation failed: {}", error_msg);

            // If token validation fails (any reason), try to refresh using cookies (2/3 system)
            handle_token_refresh_from_cookies(req, &error_msg)
        }
    }
}

/// Handle token refresh when Bearer token validation fails
fn handle_token_refresh_from_cookies(
    req: &Request,
    error_msg: &str,
) -> Result<AuthContext, Response> {
    println!("🔍 DEBUG: Token expired, attempting refresh from cookies...");

    // Try to extract refresh token from cookies
    let cookie_header = req
        .header("cookie")
        .and_then(|h| h.as_str())
        .ok_or_else(|| {
            println!("🔍 DEBUG: NO cookie header found in request");
            create_token_validation_error(error_msg)
        })?;

    let refresh_token = extract_refresh_token_from_cookies(cookie_header).ok_or_else(|| {
        println!("🔍 DEBUG: No refresh token found in cookies");
        create_token_validation_error(error_msg)
    })?;

    println!(
        "🔍 DEBUG: Refresh token extracted: {}...",
        &refresh_token[..20.min(refresh_token.len())]
    );

    // Validate refresh token and handle 2/3 system renewal
    match JwtUtils::validate_refresh_token(&refresh_token) {
        Ok(refresh_claims) => {
            println!(
                "🔍 DEBUG: Refresh token validated successfully for user: {}",
                refresh_claims.sub
            );
            handle_23_system_renewal(refresh_claims)
        }
        Err(validation_error) => {
            println!(
                "🔍 DEBUG: Refresh token validation failed: {}",
                validation_error
            );
            println!(
                "🔍 DEBUG: DUAL EXPIRY detected - both access and refresh tokens failed validation"
            );
            Err(create_dual_expiry_response())
        }
    }
}

/// Handle 2/3 system renewal logic when refresh token is valid
fn handle_23_system_renewal(
    refresh_claims: crate::utils::jwt::types::RefreshTokenClaims,
) -> Result<AuthContext, Response> {
    let now = Utc::now();
    let refresh_duration_minutes = match get_refresh_token_duration_minutes() {
        Ok(duration) => duration as i64,
        Err(_) => {
            println!("🔍 DEBUG: Error getting refresh token duration from .env, using default");
            9 // Default fallback only if .env fails
        }
    };

    let refresh_expires_at = match DateTime::from_timestamp(refresh_claims.exp, 0) {
        Some(dt) => dt,
        None => {
            println!("🔍 DEBUG: Invalid refresh token expiration timestamp, failing auth");
            return Err(create_auth_error_response("Invalid token timestamp", None));
        }
    };

    let refresh_created_at =
        refresh_expires_at - chrono::Duration::minutes(refresh_duration_minutes);
    let time_elapsed_duration = now - refresh_created_at;
    let one_third_threshold = chrono::Duration::seconds((refresh_duration_minutes * 60) / 3);

    println!(
        "🔍 DEBUG 2/3 System: time_elapsed={:.0}min, 1/3_threshold={:.0}min ({}2/3 remaining)",
        time_elapsed_duration.num_minutes(),
        one_third_threshold.num_minutes(),
        if time_elapsed_duration > one_third_threshold {
            "✅ Activate: "
        } else {
            "⏳ Wait: "
        }
    );

    // Create new access token - PRESERVE refresh context for 2/3 system
    let placeholder_pub_key = [0u8; 32];
    let (new_access_token, access_expires) =
        JwtUtils::create_access_token_from_username_with_refresh_context(
            &refresh_claims.sub,
            refresh_expires_at,
            &placeholder_pub_key,
        )
        .map_err(|_| {
            println!("🔍 DEBUG: Failed to create new access token");
            create_auth_error_response("Failed to create new access token", None)
        })?;

    let now_timestamp = now.timestamp();
    let expires_in = access_expires.timestamp() - now_timestamp;

    // Check if we need to create new refresh token (2/3 system)
    let renewed_tokens = if time_elapsed_duration > one_third_threshold {
        println!(
            "🔍 DEBUG 2/3 System: Beyond 1/3 elapsed (2/3 remaining), creating NEW refresh token (reset)"
        );
        let (new_refresh_token, _) =
            JwtUtils::create_refresh_token_from_username(&refresh_claims.sub, None).map_err(
                |_| {
                    println!("🔍 DEBUG: Failed to create new refresh token");
                    create_auth_error_response("Failed to create new refresh token", None)
                },
            )?;

        // Extract cryptographic information for signed responses
        let user_id = bs58::decode(&refresh_claims.sub).into_vec().map_err(|_| {
            println!("🔍 DEBUG: Failed to decode Base58 username");
            create_auth_error_response("Invalid username format", None)
        })?;
        let pub_key_hex = hex::encode(refresh_claims.pub_key);

        Some(RenewedTokens {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in,
            user_id,
            pub_key_hex,
        })
    } else {
        println!(
            "🔍 DEBUG 2/3 System: Within first 1/3 (more than 2/3 remaining), keeping EXISTING refresh token"
        );
        // Extract cryptographic information for signed responses
        let user_id = bs58::decode(&refresh_claims.sub).into_vec().map_err(|_| {
            println!("🔍 DEBUG: Failed to decode Base58 username");
            create_auth_error_response("Invalid username format", None)
        })?;
        let pub_key_hex = hex::encode(refresh_claims.pub_key);

        Some(RenewedTokens {
            access_token: new_access_token,
            refresh_token: String::new(), // Empty = keep existing cookie
            expires_in,
            user_id,
            pub_key_hex,
        })
    };

    Ok(AuthContext {
        username: refresh_claims.sub,
        expires_at: access_expires.timestamp(),
        refresh_expires_at: refresh_claims.exp,
        renewed_tokens,
    })
}

/// Create token validation error response
fn create_token_validation_error(error_msg: &str) -> Response {
    let (error, expires_hint) = if error_msg.contains("expired") || error_msg.contains("exp") {
        (
            "Access token has expired. Use refresh token to obtain a new access token",
            Some("20 seconds from issuance".to_string()),
        )
    } else {
        ("Invalid access token. Please authenticate again", None)
    };

    create_auth_error_response(error, expires_hint)
}
