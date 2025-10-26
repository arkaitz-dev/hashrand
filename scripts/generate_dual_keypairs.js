#!/usr/bin/env node

/**
 * Generate dual keypairs (Ed25519 + X25519) for magic link authentication testing
 * Used by final_test.sh for dual-key system authentication
 *
 * DUAL-KEY SYSTEM:
 * - Ed25519: Digital signatures (requestMagicLink signature verification)
 * - X25519: ECDH key exchange (privkey_context decryption)
 *
 * Output: JSON object with both public keys
 * {
 *   "ed25519_pub_key": "64-char-hex",
 *   "x25519_pub_key": "64-char-hex"
 * }
 */

const crypto = require('crypto');
const fs = require('fs');

/**
 * Generate Ed25519 keypair for signatures
 * @returns {Object} { publicKeyHex, privateKeyHex }
 */
function generateEd25519KeyPair() {
    try {
        const { publicKey, privateKey } = crypto.generateKeyPairSync('ed25519', {
            publicKeyEncoding: { type: 'spki', format: 'der' },
            privateKeyEncoding: { type: 'pkcs8', format: 'der' }
        });

        // Extract raw key bytes (last 32 bytes of DER format)
        const publicKeyRaw = publicKey.slice(-32);
        const privateKeyRaw = privateKey.slice(-32);

        return {
            publicKeyHex: publicKeyRaw.toString('hex'),
            privateKeyHex: privateKeyRaw.toString('hex')
        };
    } catch (error) {
        console.error('Error generating Ed25519 keypair:', error.message);
        process.exit(1);
    }
}

/**
 * Generate X25519 keypair for ECDH
 * @returns {Object} { publicKeyHex, privateKeyHex }
 */
function generateX25519KeyPair() {
    try {
        const { publicKey, privateKey } = crypto.generateKeyPairSync('x25519', {
            publicKeyEncoding: { type: 'spki', format: 'der' },
            privateKeyEncoding: { type: 'pkcs8', format: 'der' }
        });

        // Extract raw key bytes (last 32 bytes of DER format)
        const publicKeyRaw = publicKey.slice(-32);
        const privateKeyRaw = privateKey.slice(-32);

        return {
            publicKeyHex: publicKeyRaw.toString('hex'),
            privateKeyHex: privateKeyRaw.toString('hex')
        };
    } catch (error) {
        console.error('Error generating X25519 keypair:', error.message);
        process.exit(1);
    }
}

/**
 * Generate both keypairs and save private keys for later use
 */
function generateDualKeypairs() {
    // Generate Ed25519 keypair (for signatures)
    const ed25519 = generateEd25519KeyPair();

    // Generate X25519 keypair (for ECDH)
    const x25519 = generateX25519KeyPair();

    // Store private keys for signing/decryption in tests
    fs.writeFileSync('.test-ed25519-private-key', ed25519.privateKeyHex);
    fs.writeFileSync('.test-x25519-private-key', x25519.privateKeyHex);

    // Return both public keys as JSON
    return {
        ed25519_pub_key: ed25519.publicKeyHex,
        x25519_pub_key: x25519.publicKeyHex
    };
}

// Generate dual keypairs and output as JSON
const keypairs = generateDualKeypairs();
console.log(JSON.stringify(keypairs));
