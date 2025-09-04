//! Test module for Argon2id implementation
//! 
//! Verifies that the Argon2id scheme works correctly with specified parameters

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Algorithm, Version, Params
};
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use sha3::{Digest, Sha3_256};
use rand_chacha::ChaCha8Rng;
use rand::{SeedableRng, RngCore};

/// Test the complete Argon2id scheme with specified parameters
#[cfg(test)]
pub fn test_argon2id_scheme() -> Result<[u8; 32], String> {
    // Parameters as specified
    let mem_cost = 19456;  // Memory cost in KB
    let time_cost = 2;     // Time cost (iterations)
    let parallelism = 1;   // Lanes/parallelism
    let hash_length = Some(32); // Output length in bytes

    // Test data
    let test_email = "me@arkaitz.dev";
    let test_fixed_salt = b"fixed-salt-for-hmac-32-bytes-long12"; // Exactly 32 bytes

    // Step 1: SHA3-256(data)
    let mut hasher = Sha3_256::new();
    hasher.update(test_email.as_bytes());
    let sha3_result = hasher.finalize();

    // Step 2: HMAC-SHA3-256(sha3_result, fixed_salt)
    let mut mac = Hmac::<Sha3_256>::new_from_slice(test_fixed_salt)
        .map_err(|_| "Invalid fixed salt for HMAC".to_string())?;
    Mac::update(&mut mac, &sha3_result);
    let hmac_result = mac.finalize().into_bytes();

    // Step 3: ChaCha8Rng from HMAC result → 32 bytes salt
    let mut chacha_seed = [0u8; 32];
    chacha_seed.copy_from_slice(&hmac_result[..32]);
    let mut rng = ChaCha8Rng::from_seed(chacha_seed);
    let mut dynamic_salt = [0u8; 32];
    rng.fill_bytes(&mut dynamic_salt);

    // Step 4: Argon2id with specified parameters
    let params = Params::new(mem_cost, time_cost, parallelism, hash_length)
        .map_err(|e| format!("Invalid Argon2id parameters: {}", e))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    // Create salt string for argon2 crate
    let salt_string = SaltString::encode_b64(&dynamic_salt)
        .map_err(|e| format!("Failed to encode salt: {}", e))?;
    
    // Hash the SHA3 result with Argon2id
    let password_hash = argon2.hash_password(&sha3_result, &salt_string)
        .map_err(|e| format!("Argon2id hashing failed: {}", e))?;
    let hash_string = password_hash.to_string();

    // Step 5: Extract hash part after last '$' and decode from base64
    let hash_parts: Vec<&str> = hash_string.split('$').collect();
    if hash_parts.len() < 6 {
        return Err("Invalid Argon2id hash format".to_string());
    }
    
    let base64_hash = hash_parts[hash_parts.len() - 1];
    
    // Decode base64 to get raw bytes (Argon2 uses base64 without padding)
    let decoded_hash = general_purpose::STANDARD_NO_PAD.decode(base64_hash)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    // Convert to [u8; 32]
    if decoded_hash.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", decoded_hash.len()));
    }
    
    let mut final_result = [0u8; 32];
    final_result.copy_from_slice(&decoded_hash);

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2id_implementation() {
        let result = test_argon2id_scheme();
        assert!(result.is_ok(), "Argon2id scheme failed: {:?}", result.err());
        
        let hash_bytes = result.unwrap();
        assert_eq!(hash_bytes.len(), 32, "Expected 32 bytes output");
        
        // Test deterministic behavior - same input should give same output
        let result2 = test_argon2id_scheme();
        assert!(result2.is_ok());
        assert_eq!(hash_bytes, result2.unwrap(), "Argon2id should be deterministic");
    }
}