//! 2/3 threshold renewal window calculation

use crate::utils::jwt::types::RefreshTokenClaims;

/// Calculate if token is in 2/3 renewal window for key rotation
///
/// PERIOD 2/3 logic: If token has consumed 2/3 of its lifetime, rotate keys
/// PERIOD 1/3 logic: If token is still fresh (< 2/3 consumed), just refresh access token
///
/// # Arguments
/// * `claims` - Validated refresh token claims
///
/// # Returns
/// * `bool` - true if in 2/3 renewal window (key rotation needed), false otherwise
pub fn is_in_renewal_window(claims: &RefreshTokenClaims) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock error")
        .as_secs() as i64;

    let time_remaining = claims.exp - now;
    let refresh_duration_seconds = get_refresh_duration_seconds();
    let two_thirds_threshold = (refresh_duration_seconds * 2) / 3;

    time_remaining < two_thirds_threshold
}

/// Get refresh token duration from config in seconds
///
/// # Returns
/// * `i64` - Refresh token duration in seconds
pub fn get_refresh_duration_seconds() -> i64 {
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
    (refresh_duration_minutes * 60) as i64
}
