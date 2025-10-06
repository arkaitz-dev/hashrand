#!/usr/bin/env node

/**
 * Test create_signed_request.js output
 * Verifies that the signed request can be verified with the stored public key
 */

const crypto = require('crypto');
const fs = require('fs');

// Read stored keys
const pubKeyHex = fs.readFileSync('.test-magiclink-pubkey', 'utf8').trim();

console.log('Testing SignedRequest creation and verification...\n');
console.log('Public key:', pubKeyHex);

// Create test payload (similar to shared secret creation)
const testPayload = {
    "sender_email": "test@example.com",
    "receiver_email": "receiver@example.com",
    "secret_text": "Test secret",
    "expires_hours": 24,
    "max_reads": 3,
    "ui_host": "localhost"
};

const payloadJson = JSON.stringify(testPayload);
console.log('\nPayload JSON:', payloadJson);

// Use create_signed_request.js to create the signed request
const { execSync } = require('child_process');
const signedRequestStr = execSync(`node scripts/create_signed_request.js '${payloadJson}'`, { encoding: 'utf8' });
const signedRequest = JSON.parse(signedRequestStr);

console.log('\nSigned request payload:', signedRequest.payload.substring(0, 50) + '...');
console.log('Signature:', signedRequest.signature.substring(0, 50) + '...');
console.log('Signature length:', signedRequest.signature.length, 'chars');

// Now verify the signature manually
const base64Payload = signedRequest.payload;
const signatureHex = signedRequest.signature;

// Reconstruct DER public key for verification
const pubDerHeader = Buffer.from([
    0x30, 0x2a, // SEQUENCE (42 bytes)
    0x30, 0x05, // SEQUENCE (5 bytes)
    0x06, 0x03, 0x2b, 0x65, 0x70, // OID 1.3.101.112 (Ed25519)
    0x03, 0x21, // BIT STRING (33 bytes)
    0x00 // No unused bits
]);

const publicKeyBytes = Buffer.from(pubKeyHex, 'hex');
const derPublicKey = Buffer.concat([pubDerHeader, publicKeyBytes]);

// Convert signature hex to bytes
const signatureBytes = Buffer.from(signatureHex, 'hex');

// The message that was signed is the base64 payload string itself
const messageBytes = Buffer.from(base64Payload, 'utf8');

// Verify signature
try {
    const isValid = crypto.verify(null, messageBytes, {
        key: derPublicKey,
        format: 'der',
        type: 'spki'
    }, signatureBytes);

    console.log('\n✅ Signature verification:', isValid ? 'VALID' : 'INVALID');

    if (!isValid) {
        console.log('❌ ERROR: SignedRequest signature is INVALID!');
        console.log('\nDebug info:');
        console.log('- Base64 payload length:', base64Payload.length);
        console.log('- Message bytes length:', messageBytes.length);
        console.log('- Signature bytes length:', signatureBytes.length);
        process.exit(1);
    }
} catch (error) {
    console.log('❌ ERROR during verification:', error.message);
    process.exit(1);
}

console.log('\n✅ SignedRequest is correctly signed and can be verified!');
console.log('✅ This confirms create_signed_request.js works correctly.');
