#!/usr/bin/env node

/**
 * Sign query parameters with Ed25519 private key for GET request testing
 * Used by test scripts for query parameter signature generation
 * Matches backend SignedRequestValidator::validate_query_params() logic
 */

const crypto = require('crypto');
const fs = require('fs');
const bs58 = require('bs58').default || require('bs58');

/**
 * Sort object keys recursively for deterministic serialization
 * This must match the backend's sort_json_keys() function
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
 * Sign query parameters with stored Ed25519 private key
 * @param {object} params - Query parameters object
 * @param {string} publicKeyHex - Public key to include in params
 * @returns {string} - Ed25519 signature as base58 string (~88 chars, 64 bytes)
 */
function signQueryParams(params, publicKeyHex) {
    try {
        // Read stored private key
        if (!fs.existsSync('.test-ed25519-private-key')) {
            throw new Error('Private key not found. Run generate_hash.js first.');
        }

        const privateKeyHex = fs.readFileSync('.test-ed25519-private-key', 'utf8').trim();

        if (privateKeyHex.length !== 64) {
            throw new Error(`Invalid private key length: ${privateKeyHex.length}, expected 64`);
        }

        // DON'T add public key to params - backend extracts it from JWT Bearer token
        // Sort JSON keys recursively for deterministic serialization (matching backend)
        const sortedParams = sortObjectKeys(params);

        // Serialize to JSON string (matching backend serialize_query_params_deterministic)
        const jsonString = JSON.stringify(sortedParams);

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

        // Sign the JSON string (matching backend)
        const messageBytes = Buffer.from(jsonString, 'utf8');
        const signature = crypto.sign(null, messageBytes, {
            key: derPrivateKey,
            format: 'der',
            type: 'pkcs8'
        });

        // Return signature as base58 (~88 chars, 64 bytes)
        return bs58.encode(signature);
    } catch (error) {
        console.error('Error signing query params:', error.message);
        process.exit(1);
    }
}

// Parse command line arguments
const publicKeyHex = process.argv[2];
const paramsJson = process.argv[3];

if (!publicKeyHex || !paramsJson) {
    console.error('Usage: node sign_query_params.js <public_key_hex> \'{"param1":"value1","param2":"value2"}\'');
    console.error('Example: node sign_query_params.js "abc123..." \'{"length":"12","alphabet":"base58"}\'');
    process.exit(1);
}

let params;
try {
    params = JSON.parse(paramsJson);
} catch (error) {
    console.error('Error parsing JSON params:', error.message);
    process.exit(1);
}

// Sign query params and output signature
const signature = signQueryParams(params, publicKeyHex);
console.log(signature);