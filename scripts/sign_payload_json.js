#!/usr/bin/env node

/**
 * Sign JSON payload with Ed25519 private key for SignedRequest testing
 * Used by test scripts for JSON payload signature generation (not simple string concatenation)
 */

const crypto = require('crypto');
const fs = require('fs');

/**
 * Recursively sort object keys for deterministic serialization
 * Matches frontend sortObjectKeys function exactly
 */
function sortObjectKeys(obj) {
    if (obj === null || typeof obj !== 'object') {
        return obj;
    }

    if (Array.isArray(obj)) {
        return obj.map(sortObjectKeys);
    }

    const sorted = {};
    const keys = Object.keys(obj).sort();

    for (const key of keys) {
        sorted[key] = sortObjectKeys(obj[key]);
    }

    return sorted;
}

/**
 * Serialize payload deterministically (matching frontend)
 */
function serializePayload(payload) {
    const sortedPayload = sortObjectKeys(payload);
    return JSON.stringify(sortedPayload);
}

/**
 * Sign serialized payload with stored Ed25519 private key
 * @param {object} payload - JavaScript object payload to sign
 * @returns {string} - Ed25519 signature as hex string (128 chars = 64 bytes)
 */
function signPayload(payload) {
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
        const derHeader = Buffer.from([
            0x30, 0x2e, // SEQUENCE (46 bytes)
            0x02, 0x01, 0x00, // INTEGER 0 (version)
            0x30, 0x05, // SEQUENCE (5 bytes) - algorithm identifier
            0x06, 0x03, 0x2b, 0x65, 0x70, // OID 1.3.101.112 (Ed25519)
            0x04, 0x22, // OCTET STRING (34 bytes)
            0x04, 0x20 // OCTET STRING (32 bytes) - private key
        ]);

        const derPrivateKey = Buffer.concat([derHeader, privateKeyBytes]);

        // Serialize payload deterministically (matching frontend)
        const serializedPayload = serializePayload(payload);
        const messageBytes = Buffer.from(serializedPayload, 'utf8');

        // Sign serialized payload
        const signature = crypto.sign(null, messageBytes, {
            key: derPrivateKey,
            format: 'der',
            type: 'pkcs8'
        });

        // Return signature as hex (128 chars = 64 bytes)
        return signature.toString('hex');
    } catch (error) {
        console.error('Error signing payload:', error.message);
        process.exit(1);
    }
}

// Get payload from command line arguments as JSON
const payloadJson = process.argv[2];

if (!payloadJson) {
    console.error('Usage: node sign_payload_json.js \'{"email":"test@example.com","email_lang":"en",...}\'');
    console.error('Example: node sign_payload_json.js \'{"email":"test@example.com","email_lang":"en","next":"/","pub_key":"abc123"}\'');
    process.exit(1);
}

let payload;
try {
    payload = JSON.parse(payloadJson);
} catch (error) {
    console.error('Error parsing JSON payload:', error.message);
    process.exit(1);
}

// Sign payload and output signature
const signature = signPayload(payload);
console.log(signature);