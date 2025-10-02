//! Non-signed response handler - Add tokens via headers (backward compatibility)

use spin_sdk::http::Response;

use super::super::jwt_middleware_types::RenewedTokens;
use super::response_utilities::create_refresh_cookie;

/// Add renewed tokens to non-signed response via headers
///
/// Uses legacy x-new-access-token and x-token-expires-in headers
///
/// # Arguments
/// * `response` - Original response
/// * `renewed_tokens` - New tokens to add
///
/// # Returns
/// * `Response` - Response with token headers and refresh cookie
pub fn add_tokens_to_headers(response: Response, renewed_tokens: RenewedTokens) -> Response {
    let mut binding = Response::builder();
    let mut builder = binding.status(*response.status());

    // Copy existing headers
    for (name, value) in response.headers() {
        builder = builder.header(name, value.as_str().unwrap_or(""));
    }

    // Add access token headers (legacy method)
    let expires_in_str = renewed_tokens.expires_in.to_string();
    builder = builder
        .header("x-new-access-token", &renewed_tokens.access_token)
        .header("x-token-expires-in", &expires_in_str);

    // Add refresh token cookie if provided
    if !renewed_tokens.refresh_token.is_empty() {
        let refresh_cookie = create_refresh_cookie(&renewed_tokens.refresh_token);
        builder = builder.header("set-cookie", &refresh_cookie);
    }

    // Create response with original body
    let body_vec = response.body().to_vec();
    builder.body(body_vec).build()
}
