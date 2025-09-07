#!/usr/bin/env node

/**
 * Generate random 32-byte hash in base58 format
 * Used by final_test.sh for magic link authentication testing
 */

const crypto = require('crypto');

/**
 * Simple base58 encoding (Bitcoin alphabet)
 */
function base58Encode(bytes) {
    const alphabet = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';
    let result = '';
    let num = BigInt(0);

    // Convert bytes to BigInt
    for (let i = 0; i < bytes.length; i++) {
        num = (num << 8n) + BigInt(bytes[i]);
    }

    // Convert to base58
    while (num > 0n) {
        result = alphabet[Number(num % 58n)] + result;
        num = num / 58n;
    }

    // Handle leading zeros
    for (let i = 0; i < bytes.length && bytes[i] === 0; i++) {
        result = '1' + result;
    }

    return result;
}

/**
 * Generate random 32-byte hash in base58 format
 */
function generateRandomHash() {
    const randomBytes = crypto.randomBytes(32);
    return base58Encode(randomBytes);
}

// Generate and output the hash
const hash = generateRandomHash();
console.log(hash);