use serde::Serialize;

// /// Standard response structure for hash generation endpoints
// /// NOTE: Deprecated in favor of CustomHashResponse for all endpoints
// ///
// /// Contains the generated hash and the hexadecimal representation
// /// of the 32-byte seed used to generate it
// #[derive(Serialize, Debug)]
// #[allow(dead_code)]
// pub struct HashResponse {
//     /// The generated hash/ID/password/key
//     pub hash: String,
//     /// Hexadecimal representation of the 32-byte seed used for generation
//     pub seed: String,
// }

/// Enhanced response structure for all hash generation endpoints
///
/// Contains the generated hash, seed, plus additional OTP and timestamp
#[derive(Serialize, Debug)]
pub struct CustomHashResponse {
    /// The generated hash/ID
    pub hash: String,
    /// Base58 representation of the 32-byte seed used for generation
    pub seed: String,
    /// 9-digit OTP generated using the same seed
    pub otp: String,
    /// Generation timestamp in seconds since Unix epoch
    pub timestamp: u64,
}

// impl HashResponse {
//     /// Creates a new HashResponse
//     ///
//     /// # Arguments
//     /// * `hash` - The generated hash string
//     /// * `seed` - The 32-byte seed as hexadecimal string
//     #[allow(dead_code)]
//     pub fn new(hash: String, seed: String) -> Self {
//         Self { hash, seed }
//     }
// }

/// JWT authentication response structure for magic link validation endpoint
///
/// Contains JWT tokens and user information for successful authentication
#[derive(Serialize, serde::Deserialize, Debug)]
pub struct JwtAuthResponse {
    /// JWT access token for API authentication
    pub access_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Base58-encoded user ID for privacy-safe identification
    pub user_id: String,
    /// Optional next parameter for post-auth redirect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// Refresh cookie expiration timestamp (only included when new refresh cookie is set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

impl JwtAuthResponse {
    /// Creates a new JwtAuthResponse
    ///
    /// # Arguments
    /// * `access_token` - JWT access token string
    /// * `user_id` - Base58-encoded user ID
    /// * `next` - Optional next parameter for redirect
    /// * `expires_at` - Optional refresh cookie expiration timestamp (included when new refresh cookie is set)
    pub fn new(
        access_token: String,
        user_id: String,
        next: Option<String>,
        expires_at: Option<i64>,
    ) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            user_id,
            next,
            expires_at,
        }
    }
}

impl CustomHashResponse {
    /// Creates a new CustomHashResponse
    ///
    /// # Arguments
    /// * `hash` - The generated hash string
    /// * `seed` - The 32-byte seed as base58 string
    /// * `otp` - The 9-digit OTP generated from the same seed
    /// * `timestamp` - Generation timestamp in seconds since Unix epoch
    pub fn new(hash: String, seed: String, otp: String, timestamp: u64) -> Self {
        Self {
            hash,
            seed,
            otp,
            timestamp,
        }
    }
}

/// Response structure for the /api/version endpoint
#[derive(Serialize, Debug)]
pub struct VersionResponse {
    pub api_version: String,
    pub ui_version: String,
}

impl VersionResponse {
    #[allow(dead_code)]
    pub fn new(api_version: String, ui_version: String) -> Self {
        Self {
            api_version,
            ui_version,
        }
    }
}
