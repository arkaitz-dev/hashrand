//! Cookie-based Token Refresh Logic
//!
//! Handles automatic token refresh from HTTP-only cookies with 2/3 system

use chrono::{DateTime, Utc};
use spin_sdk::http::{Request, Response};
use tracing::debug;

use crate::utils::JwtUtils;
use crate::utils::jwt::config::get_refresh_token_duration_minutes;
use crate::utils::jwt_middleware_cookies::extract_refresh_token_from_cookies;
use crate::utils::jwt_middleware_errors::{
    create_auth_error_response, create_dual_expiry_response,
};
use crate::utils::jwt_middleware_types::{AuthContext, RenewedTokens};

use super::helpers::{create_token_validation_error, decode_username_to_user_id};

/// Handle token refresh when Bearer token validation fails
///
/// # Arguments
/// * `req` - HTTP request with cookie header
/// * `error_msg` - Original validation error message
///
/// # Returns
/// * `Result<AuthContext, Response>` - New auth context or error response
pub fn handle_token_refresh_from_cookies(
    req: &Request,
    error_msg: &str,
) -> Result<AuthContext, Response> {
    // println!("🔍 DEBUG: Token expired, attempting refresh from cookies...");
    debug!("🔍 DEBUG: Token expired, attempting refresh from cookies...");

    // Try to extract refresh token from cookies
    let cookie_header = req
        .header("cookie")
        .and_then(|h| h.as_str())
        .ok_or_else(|| {
            // println!("🔍 DEBUG: NO cookie header found in request");
            debug!("🔍 DEBUG: NO cookie header found in request");
            create_token_validation_error(error_msg)
        })?;

    let refresh_token = extract_refresh_token_from_cookies(cookie_header).ok_or_else(|| {
        // println!("🔍 DEBUG: No refresh token found in cookies");
        debug!("🔍 DEBUG: No refresh token found in cookies");
        create_token_validation_error(error_msg)
    })?;

    // println!(
    //     "🔍 DEBUG: Refresh token extracted: {}...",
    //     &refresh_token[..20.min(refresh_token.len())]
    // );
    debug!(
        "🔍 DEBUG: Refresh token extracted: {}...",
        &refresh_token[..20.min(refresh_token.len())]
    );

    // Validate refresh token and handle 2/3 system renewal
    match JwtUtils::validate_refresh_token(&refresh_token) {
        Ok(refresh_claims) => {
            // println!(
            //     "🔍 DEBUG: Refresh token validated successfully for user: {}",
            //     refresh_claims.sub
            // );
            debug!(
                "🔍 DEBUG: Refresh token validated successfully for user: {}",
                refresh_claims.sub
            );
            handle_23_system_renewal(refresh_claims)
        }
        Err(validation_error) => {
            // println!(
            //     "🔍 DEBUG: Refresh token validation failed: {}",
            //     validation_error
            // );
            // println!(
            //     "🔍 DEBUG: DUAL EXPIRY detected - both access and refresh tokens failed validation"
            // );
            debug!(
                "🔍 DEBUG: Refresh token validation failed: {}",
                validation_error
            );
            debug!(
                "🔍 DEBUG: DUAL EXPIRY detected - both access and refresh tokens failed validation"
            );
            Err(create_dual_expiry_response())
        }
    }
}

/// Handle 2/3 system renewal logic when refresh token is valid
///
/// # Arguments
/// * `refresh_claims` - Valid refresh token claims
///
/// # Returns
/// * `Result<AuthContext, Response>` - New auth context with renewed tokens or error
fn handle_23_system_renewal(
    refresh_claims: crate::utils::jwt::types::RefreshTokenClaims,
) -> Result<AuthContext, Response> {
    let now = Utc::now();
    let refresh_duration_minutes = match get_refresh_token_duration_minutes() {
        Ok(duration) => duration as i64,
        Err(_) => {
            // println!("🔍 DEBUG: Error getting refresh token duration from .env, using default");
            debug!("🔍 DEBUG: Error getting refresh token duration from .env, using default");
            9 // Default fallback only if .env fails
        }
    };

    let refresh_expires_at = match DateTime::from_timestamp(refresh_claims.exp, 0) {
        Some(dt) => dt,
        None => {
            // println!("🔍 DEBUG: Invalid refresh token expiration timestamp, failing auth");
            debug!("🔍 DEBUG: Invalid refresh token expiration timestamp, failing auth");
            return Err(create_auth_error_response("Invalid token timestamp", None));
        }
    };

    let refresh_created_at =
        refresh_expires_at - chrono::Duration::minutes(refresh_duration_minutes);
    let time_elapsed_duration = now - refresh_created_at;
    let one_third_threshold = chrono::Duration::seconds((refresh_duration_minutes * 60) / 3);

    // println!(
    //     "🔍 DEBUG 2/3 System: time_elapsed={:.0}min, 1/3_threshold={:.0}min ({}2/3 remaining)",
    //     time_elapsed_duration.num_minutes(),
    //     one_third_threshold.num_minutes(),
    //     if time_elapsed_duration > one_third_threshold {
    //         "✅ Activate: "
    //     } else {
    //         "⏳ Wait: "
    //     }
    // );
    debug!(
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
    // Use pub_key from refresh token claims
    let (new_access_token, access_expires) =
        JwtUtils::create_access_token_from_username_with_refresh_context(
            &refresh_claims.sub,
            refresh_expires_at,
            &refresh_claims.pub_key,
        )
        .map_err(|_| {
            // println!("🔍 DEBUG: Failed to create new access token");
            debug!("🔍 DEBUG: Failed to create new access token");
            create_auth_error_response("Failed to create new access token", None)
        })?;

    let now_timestamp = now.timestamp();
    let expires_in = access_expires.timestamp() - now_timestamp;

    // Check if we need to create new refresh token (2/3 system)
    let renewed_tokens = if time_elapsed_duration > one_third_threshold {
        // println!(
        //     "🔍 DEBUG 2/3 System: Beyond 1/3 elapsed (2/3 remaining), creating NEW refresh token (reset)"
        // );
        debug!(
            "🔍 DEBUG 2/3 System: Beyond 1/3 elapsed (2/3 remaining), creating NEW refresh token (reset)"
        );
        let (new_refresh_token, _) = JwtUtils::create_refresh_token_from_username(
            &refresh_claims.sub,
            &refresh_claims.pub_key,
        )
        .map_err(|_| {
            // println!("🔍 DEBUG: Failed to create new refresh token");
            debug!("🔍 DEBUG: Failed to create new refresh token");
            create_auth_error_response("Failed to create new refresh token", None)
        })?;

        // Extract cryptographic information for signed responses
        let user_id = decode_username_to_user_id(&refresh_claims.sub)?;
        let pub_key_hex = hex::encode(refresh_claims.pub_key);

        Some(RenewedTokens {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in,
            user_id,
            pub_key_hex,
        })
    } else {
        // println!(
        //     "🔍 DEBUG 2/3 System: Within first 1/3 (more than 2/3 remaining), keeping EXISTING refresh token"
        // );
        debug!(
            "🔍 DEBUG 2/3 System: Within first 1/3 (more than 2/3 remaining), keeping EXISTING refresh token"
        );
        // Extract cryptographic information for signed responses
        let user_id = decode_username_to_user_id(&refresh_claims.sub)?;
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
