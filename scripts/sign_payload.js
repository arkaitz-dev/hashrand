#!/usr/bin/env node

/**
 * Sign message with Ed25519 private key for magic link authentication testing
 * Used by final_test.sh for Ed25519 signature-based authentication
 */

const crypto = require('crypto');
const fs = require('fs');

/**
 * Sign message with stored Ed25519 private key
 * @param {string} message - Message to sign
 * @returns {string} - Ed25519 signature as hex string (128 chars = 64 bytes)
 */
function signMessage(message) {
    try {
        // Read stored private key
        if (!fs.existsSync('.test-ed25519-private-key')) {
            throw new Error('Private key not found. Run generate_hash.js first.');
        }

        const privateKeyHex = fs.readFileSync('.test-ed25519-private-key', 'utf8').trim();

        if (privateKeyHex.length !== 64) {
            throw new Error(`Invalid private key length: ${privateKeyHex.length}, expected 64`);
        }

        // Convert hex to bytes
        const privateKeyBytes = Buffer.from(privateKeyHex, 'hex');

        // Create DER-encoded private key for Node.js crypto
        // PKCS#8 DER encoding for Ed25519 private key
        const derHeader = Buffer.from([
            0x30, 0x2e, // SEQUENCE (46 bytes)
            0x02, 0x01, 0x00, // INTEGER 0 (version)
            0x30, 0x05, // SEQUENCE (5 bytes) - algorithm identifier
            0x06, 0x03, 0x2b, 0x65, 0x70, // OID 1.3.101.112 (Ed25519)
            0x04, 0x22, // OCTET STRING (34 bytes)
            0x04, 0x20 // OCTET STRING (32 bytes) - private key
        ]);

        const derPrivateKey = Buffer.concat([derHeader, privateKeyBytes]);

        // Sign message
        const messageBytes = Buffer.from(message, 'utf8');
        const signature = crypto.sign(null, messageBytes, {
            key: derPrivateKey,
            format: 'der',
            type: 'pkcs8'
        });

        // Return signature as hex (128 chars = 64 bytes)
        return signature.toString('hex');
    } catch (error) {
        console.error('Error signing message:', error.message);
        process.exit(1);
    }
}

// Get message from command line arguments
const message = process.argv[2];

if (!message) {
    console.error('Usage: node sign_payload.js <message>');
    console.error('Example: node sign_payload.js "email@example.com1234567890abcdef"');
    process.exit(1);
}

// Sign message and output signature
const signature = signMessage(message);
console.log(signature);