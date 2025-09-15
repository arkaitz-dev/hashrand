#!/usr/bin/env node

/**
 * Generate Ed25519 keypair for magic link authentication testing
 * Used by final_test.sh for Ed25519 signature-based authentication
 */

const crypto = require('crypto');
const fs = require('fs');

/**
 * Generate Ed25519 keypair and save private key for later signing
 */
function generateEd25519KeyPair() {
    try {
        // Generate Ed25519 keypair using Node.js crypto
        const { publicKey, privateKey } = crypto.generateKeyPairSync('ed25519', {
            publicKeyEncoding: {
                type: 'spki',
                format: 'der'
            },
            privateKeyEncoding: {
                type: 'pkcs8',
                format: 'der'
            }
        });

        // Extract raw public key bytes (last 32 bytes of DER format)
        const publicKeyRaw = publicKey.slice(-32);

        // Extract raw private key bytes (last 32 bytes of DER format)
        const privateKeyRaw = privateKey.slice(-32);

        // Convert to hex strings
        const publicKeyHex = publicKeyRaw.toString('hex');
        const privateKeyHex = privateKeyRaw.toString('hex');

        // Store private key for signing
        fs.writeFileSync('.test-ed25519-private-key', privateKeyHex);

        // Return public key hex (64 chars = 32 bytes)
        return publicKeyHex;
    } catch (error) {
        console.error('Error generating Ed25519 keypair:', error.message);
        process.exit(1);
    }
}

// Generate keypair and output public key
const publicKeyHex = generateEd25519KeyPair();
console.log(publicKeyHex);