#!/usr/bin/env node

/**
 * Test Ed25519 keypair verification
 * Verifies that the stored keypair can sign and verify correctly
 */

const crypto = require('crypto');
const fs = require('fs');

// Read stored keys
const pubKeyHex = fs.readFileSync('.test-magiclink-pubkey', 'utf8').trim();
const privKeyHex = fs.readFileSync('.test-ed25519-private-key', 'utf8').trim();

console.log('Public key (hex):', pubKeyHex);
console.log('Private key (hex):', privKeyHex);

// Test message
const testMessage = "Hello, Ed25519!";
const messageBytes = Buffer.from(testMessage, 'utf8');

// Reconstruct DER private key
const derHeader = Buffer.from([
    0x30, 0x2e, // SEQUENCE (46 bytes)
    0x02, 0x01, 0x00, // INTEGER 0 (version)
    0x30, 0x05, // SEQUENCE (5 bytes) - algorithm identifier
    0x06, 0x03, 0x2b, 0x65, 0x70, // OID 1.3.101.112 (Ed25519)
    0x04, 0x22, // OCTET STRING (34 bytes)
    0x04, 0x20 // OCTET STRING (32 bytes) - private key
]);

const privateKeyBytes = Buffer.from(privKeyHex, 'hex');
const derPrivateKey = Buffer.concat([derHeader, privateKeyBytes]);

// Sign message
const signature = crypto.sign(null, messageBytes, {
    key: derPrivateKey,
    format: 'der',
    type: 'pkcs8'
});

const signatureHex = signature.toString('hex');
console.log('\nSignature (hex):', signatureHex);
console.log('Signature length:', signatureHex.length, 'chars (should be 128)');

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

// Verify signature
try {
    const isValid = crypto.verify(null, messageBytes, {
        key: derPublicKey,
        format: 'der',
        type: 'spki'
    }, signature);

    console.log('\n✅ Signature verification:', isValid ? 'VALID' : 'INVALID');

    if (!isValid) {
        console.log('❌ ERROR: Keypair does NOT work correctly!');
        process.exit(1);
    }
} catch (error) {
    console.log('❌ ERROR during verification:', error.message);
    process.exit(1);
}

console.log('\n✅ Keypair works correctly! Sign and verify are matching.');
