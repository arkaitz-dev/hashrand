//! Pseudonimizer: Universal Blake3 cryptographic pipeline
//!
//! Provides deterministic pseudonymization with domain separation via Base58-encoded context
//! combining Blake3 KDF and keyed XOF for variable-length cryptographic outputs

use blake3;

/// Universal Blake3 pipeline: hmac_env_key + data → variable-length output
///
/// CRYPTOGRAPHIC FLOW:
/// 1. hmac_env_key[64] → Base58 → context (domain separation, fixed per use case)
/// 2. data[n] → key_material preparation:
///    - If data.len() >= 32: use data directly as key_material
///    - If data.len() < 32: Blake3 hash(data) → key_material[32 bytes]
/// 3. (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
/// 4. (data, deterministic_key, length) → Blake3 keyed+XOF → output[length bytes]
///
/// # Security Properties
/// - **Domain separation**: Different hmac_env_key → cryptographically independent outputs
/// - **Deterministic**: Same inputs always produce same output
/// - **Variable output**: Supports any output length (1 byte to 2^64 bytes)
/// - **Key derivation**: Unique 32-byte key derived per data input via KDF
/// - **Blake3 KDF compliance**: key_material always ≥32 bytes (recommended minimum)
///
/// # Arguments
/// * `hmac_env_key` - 64-byte fixed key for domain separation (one per use case)
/// * `data` - Variable-length input data (any size)
/// * `output_length` - Desired output length in bytes
///
/// # Returns
/// * `Vec<u8>` - Cryptographically derived output of specified length
///
/// # Example
/// ```rust
/// use crate::utils::pseudonimizer::blake3_keyed_variable;
///
/// let hmac_key = [0u8; 64];  // From environment variable
/// let user_data = b"user_id + pub_key";
/// let private_key = blake3_keyed_variable(&hmac_key, user_data, 32);
/// ```
pub fn blake3_keyed_variable(
    hmac_env_key: &[u8; 64],
    data: &[u8],
    output_length: usize,
) -> Vec<u8> {
    // PASO 1: hmac_env_key → Base58 → context (fixed per use case)
    let context = bs58::encode(hmac_env_key).into_string();

    // PASO 2: Prepare key_material (Blake3 KDF requires minimum 32 bytes)
    let key_material_bytes: Vec<u8> = if data.len() >= 32 {
        // Use data directly if it meets minimum length requirement
        data.to_vec()
    } else {
        // Hash short data to meet 32-byte minimum for Blake3 KDF
        blake3::hash(data).as_bytes().to_vec()
    };

    // PASO 3: (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
    let deterministic_key = blake3::derive_key(&context, &key_material_bytes);

    // PASO 4: (data, deterministic_key, length) → Blake3 keyed+XOF → output
    let mut hasher = blake3::Hasher::new_keyed(&deterministic_key);
    hasher.update(data);
    let mut output_reader = hasher.finalize_xof();

    let mut output = vec![0u8; output_length];
    output_reader.fill(&mut output);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_keyed_variable_deterministic() {
        let hmac_key = [1u8; 64];
        let data = b"test data";

        let output1 = blake3_keyed_variable(&hmac_key, data, 32);
        let output2 = blake3_keyed_variable(&hmac_key, data, 32);

        assert_eq!(output1, output2, "Same inputs should produce same output");
    }

    #[test]
    fn test_blake3_keyed_variable_different_lengths() {
        let hmac_key = [2u8; 64];
        let data = b"test";

        let output_32 = blake3_keyed_variable(&hmac_key, data, 32);
        let output_64 = blake3_keyed_variable(&hmac_key, data, 64);
        let output_128 = blake3_keyed_variable(&hmac_key, data, 128);

        assert_eq!(output_32.len(), 32);
        assert_eq!(output_64.len(), 64);
        assert_eq!(output_128.len(), 128);

        // First 32 bytes of longer outputs should match 32-byte output (XOF property)
        assert_eq!(&output_64[..32], &output_32[..]);
        assert_eq!(&output_128[..32], &output_32[..]);
    }

    #[test]
    fn test_blake3_keyed_variable_domain_separation() {
        let hmac_key1 = [3u8; 64];
        let hmac_key2 = [4u8; 64];
        let data = b"same data";

        let output1 = blake3_keyed_variable(&hmac_key1, data, 32);
        let output2 = blake3_keyed_variable(&hmac_key2, data, 32);

        assert_ne!(
            output1, output2,
            "Different hmac_env_key should produce different outputs"
        );
    }

    #[test]
    fn test_blake3_keyed_variable_data_sensitivity() {
        let hmac_key = [5u8; 64];

        let output1 = blake3_keyed_variable(&hmac_key, b"data1", 32);
        let output2 = blake3_keyed_variable(&hmac_key, b"data2", 32);

        assert_ne!(
            output1, output2,
            "Different data should produce different outputs"
        );
    }

    #[test]
    fn test_blake3_keyed_variable_short_data_handling() {
        let hmac_key = [6u8; 64];

        // Test with data < 32 bytes (should be hashed first)
        let short_data = b"short"; // 5 bytes
        let output_short = blake3_keyed_variable(&hmac_key, short_data, 32);

        // Test with data >= 32 bytes (used directly)
        let long_data = b"this is a long data string with more than 32 bytes of content"; // 62 bytes
        let output_long = blake3_keyed_variable(&hmac_key, long_data, 32);

        // Both should produce valid 32-byte outputs
        assert_eq!(
            output_short.len(),
            32,
            "Short data should produce 32-byte output"
        );
        assert_eq!(
            output_long.len(),
            32,
            "Long data should produce 32-byte output"
        );

        // Different inputs should produce different outputs
        assert_ne!(
            output_short, output_long,
            "Short and long data should produce different outputs"
        );
    }

    #[test]
    fn test_blake3_keyed_variable_exactly_32_bytes() {
        let hmac_key = [7u8; 64];

        // Test with exactly 32 bytes (boundary condition)
        let exact_32_data = b"exactly_32_bytes_of_test_data!!!"; // 32 bytes
        let output1 = blake3_keyed_variable(&hmac_key, exact_32_data, 32);

        // Should use data directly without hashing
        assert_eq!(
            output1.len(),
            32,
            "Exact 32-byte data should produce 32-byte output"
        );

        // Test determinism with exact 32 bytes
        let output2 = blake3_keyed_variable(&hmac_key, exact_32_data, 32);
        assert_eq!(
            output1, output2,
            "Exact 32-byte data should be deterministic"
        );
    }
}
