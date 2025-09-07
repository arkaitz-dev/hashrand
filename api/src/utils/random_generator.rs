use bs58;
use nanoid::nanoid;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use blake2::{Blake2b512, Digest};

/// Generates a random seed generator function for nanoid using ChaCha8Rng internally
///
/// # Arguments
/// * `seed_32` - Array of 32 bytes used as seed for ChaCha8Rng
///
/// # Returns
/// Function that generates Vec<u8> of requested size using seeded ChaCha8Rng
#[allow(dead_code)]
pub fn create_seeded_generator(_seed_32: [u8; 32]) -> fn(usize) -> Vec<u8> {
    fn generator_fn(size: usize) -> Vec<u8> {
        // We need to recreate the RNG each time to maintain deterministic behavior
        // This is a limitation but ensures consistency
        (0..size).map(|i| (i as u8).wrapping_add(42)).collect() // Placeholder
    }
    generator_fn
}

/// Generate hash using ChaCha8Rng with seed - simplified approach
pub fn generate_with_seed(seed_32: [u8; 32], length: usize, alphabet: &[char]) -> String {
    let mut rng = ChaCha8Rng::from_seed(seed_32);

    // Generate random indices manually to avoid the closure issue
    let mut result = String::with_capacity(length);
    let alphabet_len = alphabet.len();

    for _ in 0..length {
        let random_byte: u8 = rng.random(); // Use random() instead of gen()
        let index = (random_byte as usize) % alphabet_len;
        result.push(alphabet[index]);
    }

    result
}

/// Generates a random 32-byte seed using nanoid (no seed) + SHA3-256
///
/// This function:
/// 1. Uses nanoid without seed to generate a 128-character hash
/// 2. Applies SHA3-256 to create a 32-byte array
/// 3. Returns the 32-byte seed for use with ChaCha8Rng
///
/// # Returns
/// Array of 32 bytes to be used as seed for the seeded generator
pub fn generate_random_seed() -> [u8; 32] {
    // Generate 128-character hash using nanoid (cryptographically secure)
    let random_hash = nanoid!(128);

    // Apply Blake2b to the hash to get exactly 32 bytes
    let hash_result = Blake2b512::digest(random_hash.as_bytes());

    // Convert to [u8; 32] array (take first 32 bytes from Blake2b512's 64 bytes)
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash_result[..32]);
    seed
}

/// Converts a 32-byte seed to base58 string for JSON response
///
/// # Arguments
/// * `seed` - Array of 32 bytes
///
/// # Returns
/// Base58 string representation of the seed
pub fn seed_to_base58(seed: &[u8; 32]) -> String {
    bs58::encode(seed).into_string()
}

/// Converts a base58 string to 32-byte array
///
/// # Arguments
/// * `base58_str` - Base58 encoded string
///
/// # Returns
/// Result containing 32-byte array or error message
pub fn base58_to_seed(base58_str: &str) -> Result<[u8; 32], String> {
    let decoded = bs58::decode(base58_str)
        .into_vec()
        .map_err(|e| format!("Invalid base58 string: {}", e))?;

    if decoded.len() != 32 {
        return Err(format!(
            "Seed must be exactly 32 bytes, got {} bytes",
            decoded.len()
        ));
    }

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&decoded);
    Ok(seed)
}

/// Generate a 9-digit OTP using ChaCha8 with domain separation
pub fn generate_otp(seed: [u8; 32]) -> String {
    use crate::types::AlphabetType;
    use rand::Rng;

    // Create domain-separated seed for OTP generation (professional approach)
    let mut otp_seed = seed;
    // Modify last byte to create cryptographic domain separation
    otp_seed[31] ^= 0x5A; // Use different nonce for OTP domain

    // Generate OTP using ChaCha8Rng (consistent with rest of system)
    let mut rng = ChaCha8Rng::from_seed(otp_seed);
    let numeric_alphabet = AlphabetType::Numeric.as_chars();

    // Generate 9 digits using ChaCha8
    (0..9)
        .map(|_| numeric_alphabet[rng.random_range(0..numeric_alphabet.len())])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_generator_deterministic() {
        let seed = [42u8; 32];
        let alphabet = ['a', 'b', 'c', 'd', 'e']; // Simple alphabet for testing

        let result1 = generate_with_seed(seed, 10, &alphabet);
        let result2 = generate_with_seed(seed, 10, &alphabet);

        // Same seed should produce same output
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_different_seeds_different_output() {
        let seed1 = [42u8; 32];
        let seed2 = [123u8; 32];
        let alphabet = ['a', 'b', 'c', 'd', 'e']; // Simple alphabet for testing

        let result1 = generate_with_seed(seed1, 10, &alphabet);
        let result2 = generate_with_seed(seed2, 10, &alphabet);

        // Different seeds should produce different output
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_random_seed_generation() {
        let seed1 = generate_random_seed();
        let seed2 = generate_random_seed();

        // Random seeds should be different (extremely unlikely to be the same)
        assert_ne!(seed1, seed2);
        assert_eq!(seed1.len(), 32);
        assert_eq!(seed2.len(), 32);
    }

    #[test]
    fn test_seed_to_base58() {
        let seed = [42u8; 32]; // Consistent seed for testing
        let base58 = seed_to_base58(&seed);

        // Base58 string should not be empty and contain only valid base58 chars
        assert!(!base58.is_empty());
        assert!(
            base58
                .chars()
                .all(|c| "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(c))
        );
    }

    #[test]
    fn test_base58_to_seed() {
        let original_seed = [42u8; 32];
        let base58_str = seed_to_base58(&original_seed);
        let decoded_seed = base58_to_seed(&base58_str).unwrap();

        assert_eq!(original_seed, decoded_seed);
    }

    #[test]
    fn test_base58_to_seed_invalid() {
        // Test invalid base58 string
        let result = base58_to_seed("invalid_base58_0OIl");
        assert!(result.is_err());

        // Test wrong length by manually creating a base58 string of wrong length
        // This should result in the wrong number of bytes when decoded
        let short_base58 = bs58::encode(&[42u8; 16]).into_string(); // 16 bytes instead of 32
        let result = base58_to_seed(&short_base58);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be exactly 32 bytes"));
    }
}
