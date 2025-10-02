/**
 * Cryptographic Validation Tests (API-Only)
 *
 * Tests core cryptographic functions without full authentication flow
 * Perfect for Arch Linux without browser dependencies
 *
 * Validates:
 * - Ed25519 keypair generation
 * - SignedRequest creation
 * - SignedResponse verification
 * - Deterministic JSON serialization
 * - Base64 URL-safe encoding/decoding
 */

import { test, expect } from '@playwright/test';
import { TestSessionManager } from '../utils/test-session-manager';
import {
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair,
	decodePayloadBase64,
	encodePayloadBase64,
	serializePayload,
	sortObjectKeys
} from '../../src/lib/crypto/signedRequest-core';
import {
	generateKeyPairNoble,
	publicKeyBytesToHex,
	privateKeyBytesToHex,
	signMessageWithKeyPair,
	verifySignatureWithPublicKey,
	keyPairToHex,
	keyPairFromHex
} from '../../src/lib/ed25519/ed25519-core';

test.describe('Ed25519 Cryptographic Operations', () => {
	test('should generate Ed25519 keypair', () => {
		console.log('🧪 TEST: Generate Ed25519 keypair');
		console.log('='.repeat(60));

		const keyPair = generateKeyPairNoble();

		// Verify keypair structure
		expect(keyPair.publicKeyBytes).toBeDefined();
		expect(keyPair.publicKeyBytes).toHaveLength(32);

		expect(keyPair.privateKeyBytes).toBeDefined();
		expect(keyPair.privateKeyBytes).toHaveLength(32);

		// Convert to hex
		const pubKeyHex = publicKeyBytesToHex(keyPair.publicKeyBytes);
		const privKeyHex = privateKeyBytesToHex(keyPair.privateKeyBytes!);

		expect(pubKeyHex).toHaveLength(64); // 32 bytes = 64 hex chars
		expect(privKeyHex).toHaveLength(64);

		console.log(`✅ Public key: ${pubKeyHex.substring(0, 20)}...`);
		console.log(`✅ Private key: ${privKeyHex.substring(0, 20)}...`);
		console.log('🎉 TEST PASSED: Keypair generation works');
		console.log('='.repeat(60));
	});

	test('should sign and verify messages', () => {
		console.log('🧪 TEST: Sign and verify Ed25519 messages');
		console.log('='.repeat(60));

		const keyPair = generateKeyPairNoble();
		const message = 'Hello, HashRand!';

		// Sign message
		const signature = signMessageWithKeyPair(message, keyPair);
		expect(signature).toBeDefined();
		expect(signature).toHaveLength(128); // 64 bytes = 128 hex chars

		console.log(`✅ Signature: ${signature.substring(0, 40)}...`);

		// Verify signature
		const isValid = verifySignatureWithPublicKey(message, signature, keyPair.publicKeyBytes);
		expect(isValid).toBe(true);

		console.log('✅ Signature verified successfully');

		// Test invalid signature
		const invalidSignature = '0'.repeat(128);
		const isInvalid = verifySignatureWithPublicKey(
			message,
			invalidSignature,
			keyPair.publicKeyBytes
		);
		expect(isInvalid).toBe(false);

		console.log('✅ Invalid signature rejected');
		console.log('🎉 TEST PASSED: Signing and verification work');
		console.log('='.repeat(60));
	});

	test('should convert keypair to/from hex', () => {
		console.log('🧪 TEST: Keypair hex conversion');
		console.log('='.repeat(60));

		const originalKeyPair = generateKeyPairNoble();
		const hexKeys = keyPairToHex(originalKeyPair);

		expect(hexKeys.privateKeyHex).toHaveLength(64);
		expect(hexKeys.publicKeyHex).toHaveLength(64);

		console.log(`✅ Private key hex: ${hexKeys.privateKeyHex.substring(0, 20)}...`);
		console.log(`✅ Public key hex: ${hexKeys.publicKeyHex.substring(0, 20)}...`);

		// Convert back from hex
		const restoredKeyPair = keyPairFromHex(hexKeys.privateKeyHex, hexKeys.publicKeyHex);

		// Verify restored keypair works
		const message = 'Test message';
		const signature = signMessageWithKeyPair(message, restoredKeyPair);
		const isValid = verifySignatureWithPublicKey(
			message,
			signature,
			restoredKeyPair.publicKeyBytes
		);

		expect(isValid).toBe(true);

		console.log('✅ Keypair restored from hex and works correctly');
		console.log('🎉 TEST PASSED: Hex conversion works');
		console.log('='.repeat(60));
	});
});

test.describe('SignedRequest Creation', () => {
	test('should create SignedRequest with deterministic serialization', () => {
		console.log('🧪 TEST: Create SignedRequest');
		console.log('='.repeat(60));

		const keyPair = generateKeyPairNoble();
		const timestamp = Math.floor(Date.now() / 1000);
		const payload = {
			email: 'me@arkaitz.dev',
			timestamp,
			nested: { z: 'last', a: 'first', m: 'middle' }
		};

		const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

		// Verify structure
		expect(signedRequest.payload).toBeDefined();
		expect(signedRequest.signature).toBeDefined();
		expect(typeof signedRequest.payload).toBe('string');
		expect(typeof signedRequest.signature).toBe('string');

		console.log(`✅ Payload (Base64): ${signedRequest.payload.substring(0, 40)}...`);
		console.log(`✅ Signature: ${signedRequest.signature.substring(0, 40)}...`);

		// Decode and verify serialization is deterministic
		const decoded = decodePayloadBase64(signedRequest.payload);
		const parsedPayload = JSON.parse(decoded);

		// Keys should be sorted
		const keys = Object.keys(parsedPayload);
		expect(keys).toEqual(['email', 'nested', 'timestamp']);

		const nestedKeys = Object.keys(parsedPayload.nested);
		expect(nestedKeys).toEqual(['a', 'm', 'z']);

		console.log('✅ JSON keys sorted deterministically');
		console.log('🎉 TEST PASSED: SignedRequest creation works');
		console.log('='.repeat(60));
	});

	test('should create identical signatures for same payload', () => {
		console.log('🧪 TEST: Deterministic signatures');
		console.log('='.repeat(60));

		const keyPair = generateKeyPairNoble();
		const timestamp = Math.floor(Date.now() / 1000);
		const payload = { email: 'me@arkaitz.dev', timestamp };

		// Create signature twice
		const request1 = createSignedRequestWithKeyPair(payload, keyPair);
		const request2 = createSignedRequestWithKeyPair(payload, keyPair);

		// Payloads should be identical (deterministic serialization)
		expect(request1.payload).toBe(request2.payload);

		// Signatures should be identical (deterministic signing)
		expect(request1.signature).toBe(request2.signature);

		console.log('✅ Payload identical across requests');
		console.log('✅ Signature identical across requests');
		console.log('🎉 TEST PASSED: Deterministic signing works');
		console.log('='.repeat(60));
	});

	test('should sign query parameters', () => {
		console.log('🧪 TEST: Sign query parameters');
		console.log('='.repeat(60));

		const keyPair = generateKeyPairNoble();
		const timestamp = Math.floor(Date.now() / 1000).toString();
		const params = { length: '64', alphabet: 'hex', timestamp };

		const signature = signQueryParamsWithKeyPair(params, keyPair);

		expect(signature).toBeDefined();
		expect(signature).toHaveLength(128); // Ed25519 signature

		console.log(`✅ Query params signature: ${signature.substring(0, 40)}...`);

		// Sign again - should be identical
		const signature2 = signQueryParamsWithKeyPair(params, keyPair);
		expect(signature).toBe(signature2);

		console.log('✅ Signatures are deterministic');
		console.log('🎉 TEST PASSED: Query param signing works');
		console.log('='.repeat(60));
	});
});

test.describe('Base64 and JSON Serialization', () => {
	test('should encode and decode Base64 URL-safe', () => {
		console.log('🧪 TEST: Base64 URL-safe encoding');
		console.log('='.repeat(60));

		const testData = { message: 'Hello, World!', symbols: '+/=' };
		const serialized = serializePayload(testData);

		// Encode
		const encoded = encodePayloadBase64(serialized);
		expect(encoded).toBeDefined();

		// Should not contain +, /, or =
		expect(encoded).not.toContain('+');
		expect(encoded).not.toContain('/');
		expect(encoded).not.toContain('=');

		console.log(`✅ Encoded (no +/=): ${encoded}`);

		// Decode
		const decoded = decodePayloadBase64(encoded);
		expect(decoded).toBe(serialized);

		const parsed = JSON.parse(decoded);
		expect(parsed.message).toBe('Hello, World!');
		expect(parsed.symbols).toBe('+/=');

		console.log('✅ Decoded successfully');
		console.log('🎉 TEST PASSED: Base64 URL-safe encoding works');
		console.log('='.repeat(60));
	});

	test('should sort object keys recursively', () => {
		console.log('🧪 TEST: Recursive key sorting');
		console.log('='.repeat(60));

		const unsorted = {
			z: 'last',
			a: 'first',
			nested: {
				y: 'second last',
				b: 'second first',
				deep: { z: 1, a: 2, m: 3 }
			},
			m: 'middle'
		};

		const sorted = sortObjectKeys(unsorted);
		const keys = Object.keys(sorted as Record<string, unknown>);

		expect(keys).toEqual(['a', 'm', 'nested', 'z']);

		const nestedKeys = Object.keys((sorted as Record<string, Record<string, unknown>>).nested);
		expect(nestedKeys).toEqual(['b', 'deep', 'y']);

		const deepKeys = Object.keys(
			(sorted as Record<string, Record<string, Record<string, unknown>>>).nested.deep
		);
		expect(deepKeys).toEqual(['a', 'm', 'z']);

		console.log('✅ Top-level keys sorted:', keys);
		console.log('✅ Nested keys sorted:', nestedKeys);
		console.log('✅ Deep keys sorted:', deepKeys);
		console.log('🎉 TEST PASSED: Recursive sorting works');
		console.log('='.repeat(60));
	});

	test('should serialize payload deterministically', () => {
		console.log('🧪 TEST: Deterministic serialization');
		console.log('='.repeat(60));

		const payload = {
			z: 'last',
			a: 'first',
			nested: { y: 2, b: 1 }
		};

		const serialized1 = serializePayload(payload);
		const serialized2 = serializePayload(payload);

		// Should be identical
		expect(serialized1).toBe(serialized2);

		// Should have sorted keys
		expect(serialized1).toBe('{"a":"first","nested":{"b":1,"y":2},"z":"last"}');

		console.log(`✅ Serialized: ${serialized1}`);
		console.log('✅ Serialization is deterministic');
		console.log('🎉 TEST PASSED: Deterministic serialization works');
		console.log('='.repeat(60));
	});
});

test.describe('TestSessionManager', () => {
	test('should manage session state in memory', async () => {
		console.log('🧪 TEST: TestSessionManager');
		console.log('='.repeat(60));

		const session = new TestSessionManager();

		// Generate keypair
		const keyPair = await session.generateKeyPair();
		expect(keyPair).toBeDefined();

		console.log('✅ Keypair generated');

		// Store server pub key
		await session.setServerPubKey('abcd1234');
		const serverPubKey = await session.getServerPubKey();
		expect(serverPubKey).toBe('abcd1234');

		console.log('✅ Server pub key stored');

		// Store auth data
		await session.setAuthData('user-123', 'token-abc');
		const authData = await session.getAuthData();
		expect(authData.user?.user_id).toBe('user-123');
		expect(authData.access_token).toBe('token-abc');

		console.log('✅ Auth data stored');

		// Store crypto tokens
		await session.setCryptoTokens('cipher', 'nonce', 'hmac');
		const cryptoTokens = await session.getCryptoTokens();
		expect(cryptoTokens.cipher).toBe('cipher');
		expect(cryptoTokens.nonce).toBe('nonce');
		expect(cryptoTokens.hmac).toBe('hmac');

		console.log('✅ Crypto tokens stored');

		// Clear session
		await session.clear();
		const clearedAuthData = await session.getAuthData();
		expect(clearedAuthData.user).toBeNull();
		expect(clearedAuthData.access_token).toBeNull();

		console.log('✅ Session cleared');
		console.log('🎉 TEST PASSED: TestSessionManager works');
		console.log('='.repeat(60));
	});
});
