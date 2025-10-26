#!/usr/bin/env node

/**
 * Encrypt shared secret for E2EE testing (ChaCha20-Poly1305 + ECDH)
 *
 * E2E Encryption Flow:
 * 1. Generate random key_material[44] (nonce[12] + cipher_key[32])
 * 2. Encrypt secret_text with ChaCha20-Poly1305 using key_material
 * 3. Encrypt key_material with ECDH (sender X25519 private + backend X25519 public)
 *
 * Usage:
 *   node encrypt_shared_secret.js <secret_text> <backend_x25519_pub_hex>
 *
 * Input:
 *   - secret_text: Plaintext secret message
 *   - backend_x25519_pub_hex: Backend's X25519 public key (hex, 64 chars)
 *
 * Output (JSON):
 *   {
 *     "encrypted_secret": "base64...",
 *     "encrypted_key_material": "base64..."
 *   }
 *
 * Requirements:
 *   - .test-x25519-private-key file must exist (sender's X25519 private key in hex)
 */

const crypto = require('crypto');
const fs = require('fs');

// ============================================================================
// CONSTANTS
// ============================================================================

const KEY_MATERIAL_LENGTH = 44;
const NONCE_LENGTH = 12;
const CIPHER_KEY_LENGTH = 32;
const ECDH_KDF_CONTEXT = 'SharedSecretKeyMaterial_v1';

// ============================================================================
// STEP 1: Generate random key_material[44]
// ============================================================================

/**
 * Generate random key material (nonce[12] + cipher_key[32])
 * @returns {Buffer} 44-byte buffer
 */
function generateKeyMaterial() {
    return crypto.randomBytes(KEY_MATERIAL_LENGTH);
}

// ============================================================================
// STEP 2: Encrypt secret_text with ChaCha20-Poly1305
// ============================================================================

/**
 * Extract nonce and cipher_key from key_material
 * @param {Buffer} keyMaterial - 44-byte key material
 * @returns {Object} { nonce: Buffer(12), cipherKey: Buffer(32) }
 */
function extractKeyMaterial(keyMaterial) {
    const nonce = keyMaterial.slice(0, NONCE_LENGTH);
    const cipherKey = keyMaterial.slice(NONCE_LENGTH, KEY_MATERIAL_LENGTH);
    return { nonce, cipherKey };
}

/**
 * Encrypt plaintext with ChaCha20-Poly1305 using key_material
 * @param {string} plaintext - Secret text to encrypt
 * @param {Buffer} keyMaterial - 44-byte key material
 * @returns {Buffer} Encrypted data (plaintext.length + 16 bytes MAC)
 */
function encryptWithKeyMaterial(plaintext, keyMaterial) {
    const { nonce, cipherKey } = extractKeyMaterial(keyMaterial);

    // Create ChaCha20-Poly1305 cipher
    const cipher = crypto.createCipheriv('chacha20-poly1305', cipherKey, nonce, {
        authTagLength: 16
    });

    // Encrypt
    const encrypted = Buffer.concat([
        cipher.update(plaintext, 'utf8'),
        cipher.final()
    ]);

    // Get authentication tag (16 bytes)
    const authTag = cipher.getAuthTag();

    // Return ciphertext + authTag
    return Buffer.concat([encrypted, authTag]);
}

// ============================================================================
// STEP 3: Encrypt key_material with ECDH (X25519 + Blake3 KDF)
// ============================================================================

/**
 * Derive cipher_key[32] + nonce[12] from X25519 shared secret using Blake3 KDF
 *
 * Matches backend implementation:
 * - Blake3 keyed hash with shared_secret as key
 * - Context: "SharedSecretKeyMaterial_v1"
 * - XOF generates 44 bytes → cipher_key[32] + nonce[12]
 *
 * @param {Buffer} sharedSecret - X25519 shared secret (32 bytes)
 * @returns {Object} { cipherKey: Buffer(32), nonce: Buffer(12) }
 */
function deriveEcdhCipherAndNonce(sharedSecret) {
    // Use Blake3 KDF to match backend exactly
    const { blake3 } = require('@noble/hashes/blake3.js');

    // Create keyed hasher with shared_secret as key
    const context = Buffer.from(ECDH_KDF_CONTEXT);

    // Blake3 keyed hash: hash(key, message)
    // Create 44 bytes output using XOF (extendable output function)
    const hasher = blake3.create({ key: sharedSecret, dkLen: 44 });
    hasher.update(context);
    const derived = hasher.digest();

    const cipherKey = Buffer.from(derived.slice(0, CIPHER_KEY_LENGTH));
    const nonce = Buffer.from(derived.slice(CIPHER_KEY_LENGTH, 44));

    return { cipherKey, nonce };
}

/**
 * Encrypt key_material with ECDH (sender X25519 private + backend X25519 public)
 *
 * Process:
 * 1. Compute shared_secret = x25519(sender_private, backend_public)
 * 2. Derive cipher_key[32] + nonce[12] using Blake3 KDF
 * 3. Encrypt key_material with ChaCha20-Poly1305
 *
 * @param {Buffer} keyMaterial - 44-byte key material to encrypt
 * @param {string} senderPrivateHex - Sender's X25519 private key (hex, 64 chars)
 * @param {string} backendPublicHex - Backend's X25519 public key (hex, 64 chars)
 * @returns {Buffer} Encrypted key_material (44 + 16 = 60 bytes)
 */
function encryptKeyMaterialWithEcdh(keyMaterial, senderPrivateHex, backendPublicHex) {
    // Convert hex keys to raw 32-byte buffers
    const senderPrivate = Buffer.from(senderPrivateHex, 'hex');
    const backendPublic = Buffer.from(backendPublicHex, 'hex');

    if (senderPrivate.length !== 32) {
        throw new Error(`Invalid sender private key length: expected 32, got ${senderPrivate.length}`);
    }
    if (backendPublic.length !== 32) {
        throw new Error(`Invalid backend public key length: expected 32, got ${backendPublic.length}`);
    }

    // Create X25519 KeyObject from raw bytes
    // We need to wrap raw bytes in PKCS8/SPKI DER format for Node.js crypto

    // PKCS8 wrapper for X25519 private key (48 bytes total)
    const pkcs8Prefix = Buffer.from([
        0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06,
        0x03, 0x2b, 0x65, 0x6e, 0x04, 0x22, 0x04, 0x20
    ]);
    const pkcs8PrivateKey = Buffer.concat([pkcs8Prefix, senderPrivate]);

    // SPKI wrapper for X25519 public key (44 bytes total)
    const spkiPrefix = Buffer.from([
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65,
        0x6e, 0x03, 0x21, 0x00
    ]);
    const spkiPublicKey = Buffer.concat([spkiPrefix, backendPublic]);

    // Create KeyObjects
    const privateKey = crypto.createPrivateKey({
        key: pkcs8PrivateKey,
        format: 'der',
        type: 'pkcs8'
    });

    const publicKey = crypto.createPublicKey({
        key: spkiPublicKey,
        format: 'der',
        type: 'spki'
    });

    // Compute X25519 shared secret
    const sharedSecret = crypto.diffieHellman({
        privateKey: privateKey,
        publicKey: publicKey
    });

    // Derive cipher_key + nonce with Blake3 KDF
    const { cipherKey, nonce } = deriveEcdhCipherAndNonce(sharedSecret);

    // Encrypt key_material with ChaCha20-Poly1305
    const cipher = crypto.createCipheriv('chacha20-poly1305', cipherKey, nonce, {
        authTagLength: 16
    });

    const encrypted = Buffer.concat([
        cipher.update(keyMaterial),
        cipher.final()
    ]);

    const authTag = cipher.getAuthTag();

    return Buffer.concat([encrypted, authTag]);
}

// ============================================================================
// MAIN EXECUTION
// ============================================================================

function main() {
    // Parse command line arguments
    const args = process.argv.slice(2);

    if (args.length !== 2) {
        console.error('Usage: node encrypt_shared_secret.js <secret_text> <backend_x25519_pub_hex>');
        process.exit(1);
    }

    const secretText = args[0];
    const backendX25519PubHex = args[1];

    // Read sender's X25519 private key from file
    if (!fs.existsSync('.test-x25519-private-key')) {
        console.error('Error: .test-x25519-private-key file not found');
        console.error('Run generate_dual_keypairs.js first to create test keypairs');
        process.exit(1);
    }

    const senderX25519PrivateHex = fs.readFileSync('.test-x25519-private-key', 'utf8').trim();

    // Validate inputs
    if (backendX25519PubHex.length !== 64) {
        console.error(`Invalid backend X25519 public key: expected 64 hex chars, got ${backendX25519PubHex.length}`);
        process.exit(1);
    }

    if (senderX25519PrivateHex.length !== 64) {
        console.error(`Invalid sender X25519 private key: expected 64 hex chars, got ${senderX25519PrivateHex.length}`);
        process.exit(1);
    }

    try {
        // Step 1: Generate random key_material[44]
        const keyMaterial = generateKeyMaterial();

        // Step 2: Encrypt secret_text with ChaCha20-Poly1305
        const encryptedSecret = encryptWithKeyMaterial(secretText, keyMaterial);

        // Step 3: Encrypt key_material with ECDH
        const encryptedKeyMaterial = encryptKeyMaterialWithEcdh(
            keyMaterial,
            senderX25519PrivateHex,
            backendX25519PubHex
        );

        // Output JSON with base64-encoded values
        const result = {
            encrypted_secret: encryptedSecret.toString('base64'),
            encrypted_key_material: encryptedKeyMaterial.toString('base64')
        };

        console.log(JSON.stringify(result));

    } catch (error) {
        console.error('Encryption error:', error.message);
        process.exit(1);
    }
}

main();
