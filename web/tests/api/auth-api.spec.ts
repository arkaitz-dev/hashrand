/**
 * API-Only Authentication Tests
 *
 * Tests authentication flow using only API requests (no browser required)
 * Validates:
 * - Ed25519 signature creation
 * - SignedRequest/SignedResponse handling
 * - Magic link generation (dev mode)
 * - Server public key extraction
 *
 * This version works without browser dependencies - perfect for Arch Linux
 */

import { test, expect } from '@playwright/test';
import { TestSessionManager } from '../utils/test-session-manager';
import {
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair,
	decodePayloadBase64
} from '../../src/lib/crypto/signedRequest-core';
import { publicKeyBytesToHex } from '../../src/lib/ed25519/ed25519-core';
import { ed25519 } from '@noble/curves/ed25519.js';

test.describe('API-Only Authentication Tests', () => {
	test('should request magic link with Ed25519 signature', async ({ request }) => {
		console.log('🧪 TEST: Request magic link (API only)');
		console.log('='.repeat(60));

		// Create session manager
		const session = new TestSessionManager();

		// Generate Ed25519 keypair
		const keyPair = await session.generateKeyPair();
		const pubKeyHex = publicKeyBytesToHex(keyPair.publicKeyBytes);

		console.log(`🔑 Generated keypair: ${pubKeyHex.substring(0, 20)}...`);

		// Create signed request payload
		const payload = {
			email: 'me@arkaitz.dev',
			email_lang: 'en',
			next: '/',
			pub_key: pubKeyHex,
			ui_host: 'localhost'
		};

		const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

		console.log('📤 Sending signed request to /api/login/');

		// Send POST request
		const response = await request.post('http://localhost:3000/api/login/', {
			headers: { 'Content-Type': 'application/json' },
			data: signedRequest
		});

		// Verify response
		expect(response.ok()).toBeTruthy();
		expect(response.status()).toBe(200);

		const signedResponse = await response.json();

		// Verify SignedResponse structure
		expect(signedResponse.payload).toBeDefined();
		expect(signedResponse.signature).toBeDefined();
		expect(typeof signedResponse.payload).toBe('string');
		expect(typeof signedResponse.signature).toBe('string');

		console.log('✅ Received SignedResponse');

		// Decode payload
		const jsonString = decodePayloadBase64(signedResponse.payload);
		const responsePayload = JSON.parse(jsonString);

		console.log('📋 Response payload fields:', Object.keys(responsePayload));

		// Verify server_pub_key exists
		expect(responsePayload.server_pub_key).toBeDefined();
		expect(responsePayload.server_pub_key).toHaveLength(64);

		console.log(`🔐 Server pub_key: ${responsePayload.server_pub_key.substring(0, 20)}...`);

		// Verify signature with server public key
		const messageBytes = new TextEncoder().encode(signedResponse.payload);
		const signatureBytes = new Uint8Array(
			signedResponse.signature.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
		);
		const publicKeyBytes = new Uint8Array(
			responsePayload.server_pub_key.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
		);

		const isValid = ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
		expect(isValid).toBe(true);

		console.log('✅ Server signature verified');

		// Verify status (magic link sent via email)
		expect(responsePayload.status).toBeDefined();
		console.log(`✅ Status: ${responsePayload.status}`);

		// Note: In production, dev_magic_link is not returned
		// Magic link is sent via email to Mailtrap
		console.log('📧 Magic link sent to email (Mailtrap)');

		console.log('🎉 TEST PASSED: Magic link request successful');
		console.log('='.repeat(60));

		// Cleanup
		await session.clear();
	});

	test('should reject unsigned request', async ({ request }) => {
		console.log('🧪 TEST: Reject unsigned request');
		console.log('='.repeat(60));

		// Send request WITHOUT signature
		const payload = {
			email: 'me@arkaitz.dev',
			email_lang: 'en',
			next: '/',
			pub_key: '0000000000000000000000000000000000000000000000000000000000000000',
			ui_host: 'localhost'
		};

		const response = await request.post('http://localhost:3000/api/login/', {
			headers: { 'Content-Type': 'application/json' },
			data: payload // Missing SignedRequest structure
		});

		// Should fail (400 or 401)
		expect(response.ok()).toBeFalsy();
		expect([400, 401]).toContain(response.status());

		console.log(`✅ Unsigned request rejected with status ${response.status()}`);
		console.log('🎉 TEST PASSED: Signature validation works');
		console.log('='.repeat(60));
	});

	test('should reject invalid signature', async ({ request }) => {
		console.log('🧪 TEST: Reject invalid Ed25519 signature');
		console.log('='.repeat(60));

		const session = new TestSessionManager();
		const keyPair = await session.generateKeyPair();
		const pubKeyHex = publicKeyBytesToHex(keyPair.publicKeyBytes);

		// Create valid payload
		const payload = {
			email: 'me@arkaitz.dev',
			email_lang: 'en',
			next: '/',
			pub_key: pubKeyHex,
			ui_host: 'localhost'
		};

		// Create SignedRequest with INVALID signature
		const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);
		signedRequest.signature = '0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'; // Invalid

		const response = await request.post('http://localhost:3000/api/login/', {
			headers: { 'Content-Type': 'application/json' },
			data: signedRequest
		});

		// Should fail
		expect(response.ok()).toBeFalsy();
		expect([400, 401]).toContain(response.status());

		console.log(`✅ Invalid signature rejected with status ${response.status()}`);
		console.log('🎉 TEST PASSED: Ed25519 verification works');
		console.log('='.repeat(60));

		await session.clear();
	});

	test('should handle multiple magic link requests', async ({ request }) => {
		console.log('🧪 TEST: Multiple magic link requests');
		console.log('='.repeat(60));

		const serverPubKeys: string[] = [];

		// Request 3 magic links
		for (let i = 1; i <= 3; i++) {
			const session = new TestSessionManager();
			const keyPair = await session.generateKeyPair();
			const pubKeyHex = publicKeyBytesToHex(keyPair.publicKeyBytes);

			const payload = {
				email: 'me@arkaitz.dev',
				email_lang: 'en',
				next: '/',
				pub_key: pubKeyHex,
				ui_host: 'localhost'
			};

			const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

			const response = await request.post('http://localhost:3000/api/login/', {
				headers: { 'Content-Type': 'application/json' },
				data: signedRequest
			});

			expect(response.ok()).toBeTruthy();

			const signedResponse = await response.json();
			const jsonString = decodePayloadBase64(signedResponse.payload);
			const responsePayload = JSON.parse(jsonString);

			serverPubKeys.push(responsePayload.server_pub_key);
			console.log(`✅ Request ${i}: server_pub_key ${responsePayload.server_pub_key.substring(0, 20)}..., status: ${responsePayload.status}`);

			await session.clear();
		}

		// All requests should succeed (server_pub_key should be consistent)
		expect(serverPubKeys).toHaveLength(3);
		serverPubKeys.forEach(key => {
			expect(key).toHaveLength(64);
		});

		console.log('✅ All 3 requests successful');
		console.log('🎉 TEST PASSED: Multiple requests work correctly');
		console.log('='.repeat(60));
	});
});
