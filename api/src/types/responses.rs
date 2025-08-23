use serde::Serialize;

/// Standard response structure for all hash generation endpoints
/// 
/// Contains the generated hash and the hexadecimal representation 
/// of the 32-byte seed used to generate it
#[derive(Serialize, Debug)]
pub struct HashResponse {
    /// The generated hash/ID/password/key
    pub hash: String,
    /// Hexadecimal representation of the 32-byte seed used for generation
    pub seed: String,
}

impl HashResponse {
    /// Creates a new HashResponse
    ///
    /// # Arguments
    /// * `hash` - The generated hash string
    /// * `seed` - The 32-byte seed as hexadecimal string
    pub fn new(hash: String, seed: String) -> Self {
        Self { hash, seed }
    }
}

/// Response structure for the /api/version endpoint
#[derive(Serialize, Debug)]
pub struct VersionResponse {
    pub api_version: String,
    pub ui_version: String,
}

impl VersionResponse {
    pub fn new(api_version: String, ui_version: String) -> Self {
        Self { api_version, ui_version }
    }
}
