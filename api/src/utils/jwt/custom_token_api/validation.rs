use super::super::custom_token_types::TokenType;
use super::super::custom_tokens::validate_custom_token;
use super::super::types::AccessTokenClaims;

/// Validate any token using custom token system (validation logic is same: expiration_timestamp < now)
pub fn validate_custom_access_token(token: &str) -> Result<AccessTokenClaims, String> {
    // Since both token types now use the same keys, try access first (most common)
    let access_result = validate_custom_token(token, TokenType::Access);
    if let Ok(claims) = access_result {
        return Ok(claims.to_access_token_claims());
    }

    // Fallback to refresh (though should work with either due to same keys)
    let refresh_result = validate_custom_token(token, TokenType::Refresh);
    if let Ok(claims) = refresh_result {
        return Ok(claims.to_access_token_claims());
    }

    // ENHANCED ERROR DETECTION: Check if token is expired even if other validations fail
    // This allows middleware to detect true expiration vs corruption/invalidity
    let access_error = access_result.unwrap_err();
    let refresh_error = refresh_result.unwrap_err();

    // If either validation reached expiration check, prefer that error
    if access_error.contains("expired") {
        Err(access_error)
    } else if refresh_error.contains("expired") {
        Err(refresh_error)
    } else {
        // No expiration detected in either validation - token is invalid for other reasons
        Err("Invalid token - corrupted, malformed, or wrong key".to_string())
    }
}

/// Validate custom refresh token specifically (uses refresh token configuration only)
pub fn validate_custom_refresh_token(token: &str) -> Result<AccessTokenClaims, String> {
    let claims = validate_custom_token(token, TokenType::Refresh)?;
    Ok(claims.to_access_token_claims())
}
