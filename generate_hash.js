#!/usr/bin/env node

/**
 * Generate a cryptographically secure random hash for testing purposes
 * This generates a 32-byte random value encoded in Base58 for API testing
 */

const crypto = require('crypto');

// Base58 alphabet (Bitcoin style)
const BASE58_ALPHABET = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

function toBase58(bytes) {
    let result = '';
    let num = BigInt('0x' + bytes.toString('hex'));
    
    while (num > 0) {
        const remainder = num % 58n;
        result = BASE58_ALPHABET[Number(remainder)] + result;
        num = num / 58n;
    }
    
    // Add leading zeros
    for (let i = 0; i < bytes.length && bytes[i] === 0; i++) {
        result = '1' + result;
    }
    
    return result;
}

// Generate 32 random bytes (256 bits)
const randomBytes = crypto.randomBytes(32);

// Convert to Base58
const base58Hash = toBase58(randomBytes);

// Output the hash
console.log(base58Hash);