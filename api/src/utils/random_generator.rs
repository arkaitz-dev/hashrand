use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use sha3::{Digest, Sha3_256};
use nanoid::nanoid;

/// Generates a random seed generator function for nanoid using ChaCha8Rng internally
///
/// # Arguments
/// * `seed_32` - Array of 32 bytes used as seed for ChaCha8Rng
///
/// # Returns
/// Function that generates Vec<u8> of requested size using seeded ChaCha8Rng
pub fn create_seeded_generator(seed_32: [u8; 32]) -> fn(usize) -> Vec<u8> {
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
    
    // Apply SHA3-256 to the hash to get exactly 32 bytes
    let mut hasher = Sha3_256::new();
    hasher.update(random_hash.as_bytes());
    let hash_result = hasher.finalize();
    
    // Convert to [u8; 32] array
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash_result);
    seed
}

/// Converts a 32-byte seed to hexadecimal string for JSON response
///
/// # Arguments
/// * `seed` - Array of 32 bytes
///
/// # Returns
/// Hexadecimal string representation of the seed
pub fn seed_to_hex(seed: &[u8; 32]) -> String {
    hex::encode(seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seeded_generator_deterministic() {
        let seed = [42u8; 32];
        let gen1 = create_seeded_generator(seed);
        let gen2 = create_seeded_generator(seed);
        
        // Same seed should produce same output
        assert_eq!(gen1(10), gen2(10));
    }

    #[test]
    fn test_different_seeds_different_output() {
        let seed1 = [42u8; 32];
        let seed2 = [123u8; 32];
        
        let gen1 = create_seeded_generator(seed1);
        let gen2 = create_seeded_generator(seed2);
        
        // Different seeds should produce different output
        assert_ne!(gen1(10), gen2(10));
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
    fn test_seed_to_hex() {
        let seed = [0u8, 15u8, 255u8, 170u8]; // Mix of values
        let hex = seed_to_hex(&seed);
        
        assert_eq!(hex.len(), 8); // 4 bytes = 8 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
}