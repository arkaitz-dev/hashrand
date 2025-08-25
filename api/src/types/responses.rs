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
