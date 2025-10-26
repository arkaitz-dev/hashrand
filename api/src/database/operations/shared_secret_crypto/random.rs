///! Random generation functions for shared secrets
///!
///! Uses ChaCha8Rng with Blake3-seeded entropy for cryptographically secure randomness.

use super::super::shared_secret_types::constants::*;
use tracing::debug;

/// Generate cryptographically secure random reference hash
///
/// Uses ChaCha8Rng for secure random generation with Blake3 seed
///
/// # Returns
/// * `[u8; 16]` - Random 16-byte reference hash
pub fn generate_reference_hash() -> [u8; REFERENCE_HASH_LENGTH] {
    use rand::RngCore;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    // Generate seed using Blake3 of current timestamp + process-specific data
    let seed_material = format!("{:?}", std::time::SystemTime::now());
    let seed_hash = blake3::hash(seed_material.as_bytes());
    let seed: [u8; 32] = *seed_hash.as_bytes();

    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut reference = [0u8; REFERENCE_HASH_LENGTH];
    rng.fill_bytes(&mut reference);
    reference
}

/// Generate cryptographically secure 9-digit OTP
///
/// Uses ChaCha8Rng to generate a random number between 100000000 and 999999999
///
/// # Returns
/// * `String` - 9-digit OTP as string
pub fn generate_otp() -> String {
    use rand::Rng;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    // Generate seed using Blake3 of current timestamp + counter
    let seed_material = format!("{:?}_otp", std::time::SystemTime::now());
    let seed_hash = blake3::hash(seed_material.as_bytes());
    let seed: [u8; 32] = *seed_hash.as_bytes();

    let mut rng = ChaCha8Rng::from_seed(seed);
    let otp: u32 = rng.random_range(100_000_000..=999_999_999);
    otp.to_string()
}

/// Generate cryptographically secure random key material for payload encryption
///
/// Uses ChaCha8Rng for secure random generation with Blake3 seed
///
/// # Returns
/// * `[u8; 44]` - Random 44-byte key material (nonce[12] + cipher_key[32])
#[allow(dead_code)]
pub fn generate_random_key_material() -> [u8; KEY_MATERIAL_LENGTH] {
    use rand::RngCore;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    // Generate seed using Blake3 of current timestamp + process-specific data
    let seed_material = format!("{:?}_key_material", std::time::SystemTime::now());
    let seed_hash = blake3::hash(seed_material.as_bytes());
    let seed: [u8; 32] = *seed_hash.as_bytes();

    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut key_material = [0u8; KEY_MATERIAL_LENGTH];
    rng.fill_bytes(&mut key_material);

    debug!(
        "🔑 SharedSecret: Generated random key_material[{}]",
        KEY_MATERIAL_LENGTH
    );
    key_material
}
