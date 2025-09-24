// Debug Ed25519 compatibility between Noble curves (frontend) and ed25519-dalek (backend)
// This test will help identify any format compatibility issues

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex;
use rand::rngs::OsRng;
use serde_json::json;

fn main() {
    println!("🔍 Ed25519 Compatibility Diagnostic");
    println!("=====================================");

    // Test 1: Generate key pair with ed25519-dalek (same as backend)
    println!("\n[1] Generating Ed25519 keypair with ed25519-dalek");
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let public_key_bytes = verifying_key.as_bytes();
    let private_key_bytes = signing_key.as_bytes();

    println!("Private key (32 bytes): {}", hex::encode(private_key_bytes));
    println!("Public key (32 bytes): {}", hex::encode(public_key_bytes));
    println!("Public key length: {}", public_key_bytes.len());

    // Test 2: Test message serialization (matching frontend)
    println!("\n[2] Testing message serialization");
    let test_payload = json!({
        "magiclink": "8ukaMHhcnJJSEePzD5UYaoHgWib1tr8rS6ms73pC985s"
    });

    // Serialize using the same deterministic method as backend
    let serialized = serde_json::to_string(&test_payload).unwrap();
    println!("Serialized payload: {}", serialized);
    println!("Message bytes length: {}", serialized.as_bytes().len());

    // Test 3: Sign message with ed25519-dalek
    println!("\n[3] Signing message with ed25519-dalek");
    let message_bytes = serialized.as_bytes();
    let signature = signing_key.sign(message_bytes);
    let signature_bytes = signature.to_bytes();
    let signature_hex = hex::encode(signature_bytes);

    println!("Signature (64 bytes): {}", signature_hex);
    println!("Signature length: {}", signature_bytes.len());

    // Test 4: Verify signature with ed25519-dalek
    println!("\n[4] Verifying signature with ed25519-dalek");
    let verification_result = verifying_key.verify(message_bytes, &signature);
    match verification_result {
        Ok(()) => println!("✅ Signature verification: SUCCESS"),
        Err(e) => println!("❌ Signature verification: FAILED - {}", e),
    }

    // Test 5: Test hex round-trip conversion
    println!("\n[5] Testing hex conversion round-trip");
    let public_key_hex = hex::encode(public_key_bytes);
    let decoded_public_key = hex::decode(&public_key_hex).unwrap();
    let decoded_signature = hex::decode(&signature_hex).unwrap();

    println!("Public key hex: {}", public_key_hex);
    println!("Decoded public key matches: {}", decoded_public_key == public_key_bytes);
    println!("Decoded signature matches: {}", decoded_signature == signature_bytes);

    // Test 6: Recreate VerifyingKey from hex (like backend does)
    println!("\n[6] Testing VerifyingKey recreation from hex");
    let recreated_verifying_key = match VerifyingKey::from_bytes(&decoded_public_key.try_into().unwrap()) {
        Ok(key) => {
            println!("✅ VerifyingKey recreation: SUCCESS");
            key
        },
        Err(e) => {
            println!("❌ VerifyingKey recreation: FAILED - {}", e);
            return;
        }
    };

    // Test 7: Verify with recreated key (simulating backend process)
    println!("\n[7] Verifying with recreated VerifyingKey (backend simulation)");
    let recreated_signature = Signature::from_bytes(&decoded_signature.try_into().unwrap());
    let final_verification = recreated_verifying_key.verify(message_bytes, &recreated_signature);

    match final_verification {
        Ok(()) => println!("✅ Backend simulation verification: SUCCESS"),
        Err(e) => println!("❌ Backend simulation verification: FAILED - {}", e),
    }

    // Test 8: Output data for frontend comparison
    println!("\n[8] Data for frontend comparison");
    println!("=================================");
    println!("Private key hex (for Noble curves): {}", hex::encode(private_key_bytes));
    println!("Public key hex (for transmission): {}", public_key_hex);
    println!("Message to sign: {}", serialized);
    println!("Expected signature hex: {}", signature_hex);
    println!("Expected verification: SUCCESS");

    // Test 9: Test with different byte orderings (little-endian vs big-endian)
    println!("\n[9] Testing byte order consistency");
    println!("Public key bytes (raw): {:?}", public_key_bytes);
    println!("Signature bytes (raw): {:?}", &signature_bytes[..8]); // First 8 bytes only for display
}