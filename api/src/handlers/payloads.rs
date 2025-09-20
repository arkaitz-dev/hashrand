//! Payload structures for signed requests to protected endpoints
//!
//! Defines the payload types that will be wrapped in SignedRequest<T>
//! for all authenticated API endpoints

use serde::{Deserialize, Serialize};

/// Payload for /api/custom endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>, // Base58-encoded seed (optional)
}

/// Payload for /api/password endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>, // Base58-encoded seed (optional)
}

/// Payload for /api/api-key endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeyPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>, // Base58-encoded seed (optional)
}

/// Payload for /api/mnemonic endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct MnemonicPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<u32>, // 12 or 24
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>, // Language code (e.g., "en", "es", "fr")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>, // Base58-encoded seed (optional)
}

/// Payload for /api/from-seed endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct FromSeedPayload {
    pub seed: String, // Base58-encoded seed (required)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
}

// Helper implementations for converting payloads to HashMap for legacy handlers

impl CustomPayload {
    /// Convert to HashMap for compatibility with existing handler logic
    pub fn to_params_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::new();

        if let Some(length) = self.length {
            params.insert("length".to_string(), length.to_string());
        }
        if let Some(ref alphabet) = self.alphabet {
            params.insert("alphabet".to_string(), alphabet.clone());
        }
        if let Some(ref prefix) = self.prefix {
            params.insert("prefix".to_string(), prefix.clone());
        }
        if let Some(ref suffix) = self.suffix {
            params.insert("suffix".to_string(), suffix.clone());
        }
        if let Some(raw) = self.raw {
            params.insert("raw".to_string(), raw.to_string());
        }
        if let Some(ref seed) = self.seed {
            params.insert("seed".to_string(), seed.clone());
        }

        params
    }
}

impl PasswordPayload {
    /// Convert to HashMap for compatibility with existing handler logic
    pub fn to_params_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::new();

        if let Some(length) = self.length {
            params.insert("length".to_string(), length.to_string());
        }
        if let Some(ref alphabet) = self.alphabet {
            params.insert("alphabet".to_string(), alphabet.clone());
        }
        if let Some(raw) = self.raw {
            params.insert("raw".to_string(), raw.to_string());
        }
        if let Some(ref seed) = self.seed {
            params.insert("seed".to_string(), seed.clone());
        }

        params
    }
}

impl ApiKeyPayload {
    /// Convert to HashMap for compatibility with existing handler logic
    pub fn to_params_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::new();

        if let Some(length) = self.length {
            params.insert("length".to_string(), length.to_string());
        }
        if let Some(ref alphabet) = self.alphabet {
            params.insert("alphabet".to_string(), alphabet.clone());
        }
        if let Some(raw) = self.raw {
            params.insert("raw".to_string(), raw.to_string());
        }
        if let Some(ref seed) = self.seed {
            params.insert("seed".to_string(), seed.clone());
        }

        params
    }
}

impl MnemonicPayload {
    /// Convert to HashMap for compatibility with existing handler logic
    pub fn to_params_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::new();

        if let Some(word_count) = self.word_count {
            params.insert("word_count".to_string(), word_count.to_string());
        }
        if let Some(ref language) = self.language {
            params.insert("language".to_string(), language.clone());
        }
        if let Some(ref seed) = self.seed {
            params.insert("seed".to_string(), seed.clone());
        }

        params
    }
}

impl FromSeedPayload {
    /// Convert to HashMap for compatibility with existing handler logic
    pub fn to_params_map(&self) -> std::collections::HashMap<String, String> {
        let mut params = std::collections::HashMap::new();

        params.insert("seed".to_string(), self.seed.clone());

        if let Some(length) = self.length {
            params.insert("length".to_string(), length.to_string());
        }
        if let Some(ref alphabet) = self.alphabet {
            params.insert("alphabet".to_string(), alphabet.clone());
        }
        if let Some(ref prefix) = self.prefix {
            params.insert("prefix".to_string(), prefix.clone());
        }
        if let Some(ref suffix) = self.suffix {
            params.insert("suffix".to_string(), suffix.clone());
        }
        if let Some(raw) = self.raw {
            params.insert("raw".to_string(), raw.to_string());
        }

        params
    }
}