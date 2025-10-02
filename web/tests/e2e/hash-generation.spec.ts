/**
 * Hash Generation E2E Test
 *
 * Tests authenticated hash generation endpoints:
 * - /api/custom (custom hash with alphabet and length)
 * - /api/password (secure password generation)
 * - /api/api-key (API key generation)
 * - /api/mnemonic (BIP39 mnemonic phrases)
 *
 * All requests must be:
 * 1. Signed with Ed25519 (query params signature)
 * 2. Authenticated with JWT Bearer token
 * 3. Return SignedResponse with server signature
 *
 * Replicates bash test patterns from scripts/final_test.sh
 */

import { test, expect } from '../utils/test-auth-helpers';
import {
	requestMagicLink,
	loginWithMagicLink,
	generateCustomHash,
	generatePassword,
	generateApiKey,
	generateMnemonic
} from '../utils/test-auth-helpers';

test.describe('Hash Generation - Authenticated Endpoints', () => {
	// Setup: Authenticate before each test
	test.beforeEach(async ({ page, request, session }) => {
		console.log('🔑 Setup: Authenticating...');
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);
		console.log('✅ Setup complete: User authenticated');
	});

	test('should generate custom hash with default parameters', async ({ request, session }) => {
		console.log('🧪 TEST: Generate custom hash (default parameters)');
		console.log('='.repeat(60));

		// Generate custom hash with default parameters
		const result = await generateCustomHash(request, session, {});

		// Verify hash format
		expect(result.hash).toBeTruthy();
		expect(result.hash).toHaveLength(64); // Default length = 64 chars
		expect(result.hash).toMatch(/^[0-9a-f]+$/); // Hex alphabet (default)

		// Verify seed format
		expect(result.seed).toBeTruthy();
		expect(result.seed).toHaveLength(64); // Blake3 seed = 32 bytes = 64 hex chars

		// Verify OTP format
		expect(result.otp).toBeTruthy();
		expect(result.otp).toHaveLength(6); // OTP = 6 digits

		console.log(`✅ Generated hash: ${result.hash}`);
		console.log(`✅ Seed: ${result.seed}`);
		console.log(`✅ OTP: ${result.otp}`);
		console.log('🎉 TEST PASSED: Custom hash generation works');
		console.log('='.repeat(60));
	});

	test('should generate custom hash with custom length', async ({ request, session }) => {
		console.log('🧪 TEST: Generate custom hash (length=32)');
		console.log('='.repeat(60));

		const result = await generateCustomHash(request, session, { length: 32 });

		expect(result.hash).toBeTruthy();
		expect(result.hash).toHaveLength(32); // Custom length
		expect(result.hash).toMatch(/^[0-9a-f]+$/); // Hex alphabet

		console.log(`✅ Generated hash (32 chars): ${result.hash}`);
		console.log('🎉 TEST PASSED: Custom length works');
		console.log('='.repeat(60));
	});

	test('should generate custom hash with custom alphabet', async ({ request, session }) => {
		console.log('🧪 TEST: Generate custom hash (alphabet=base58)');
		console.log('='.repeat(60));

		const result = await generateCustomHash(request, session, { alphabet: 'base58' });

		expect(result.hash).toBeTruthy();
		expect(result.hash).toHaveLength(64); // Default length
		// Base58: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz (no 0, O, I, l)
		expect(result.hash).toMatch(/^[1-9A-HJ-NP-Za-km-z]+$/);

		console.log(`✅ Generated hash (base58): ${result.hash}`);
		console.log('🎉 TEST PASSED: Custom alphabet works');
		console.log('='.repeat(60));
	});

	test('should generate secure password with default parameters', async ({ request, session }) => {
		console.log('🧪 TEST: Generate password (default parameters)');
		console.log('='.repeat(60));

		const result = await generatePassword(request, session, {});

		// Verify password format
		expect(result.password).toBeTruthy();
		expect(result.password).toHaveLength(32); // Default length = 32 chars

		// Default: uppercase + lowercase + numbers + symbols
		expect(result.password).toMatch(/[A-Z]/); // Has uppercase
		expect(result.password).toMatch(/[a-z]/); // Has lowercase
		expect(result.password).toMatch(/[0-9]/); // Has numbers
		expect(result.password).toMatch(/[!@#$%^&*()_+\-=[\]{};':"\\|,.<>/?]/); // Has symbols

		// Verify seed and OTP
		expect(result.seed).toBeTruthy();
		expect(result.seed).toHaveLength(64);
		expect(result.otp).toBeTruthy();
		expect(result.otp).toHaveLength(6);

		console.log(`✅ Generated password: ${result.password}`);
		console.log(`✅ Seed: ${result.seed}`);
		console.log(`✅ OTP: ${result.otp}`);
		console.log('🎉 TEST PASSED: Password generation works');
		console.log('='.repeat(60));
	});

	test('should generate password with custom length', async ({ request, session }) => {
		console.log('🧪 TEST: Generate password (length=64)');
		console.log('='.repeat(60));

		const result = await generatePassword(request, session, { length: 64 });

		expect(result.password).toBeTruthy();
		expect(result.password).toHaveLength(64); // Custom length

		console.log(`✅ Generated password (64 chars): ${result.password}`);
		console.log('🎉 TEST PASSED: Custom password length works');
		console.log('='.repeat(60));
	});

	test('should generate password with only lowercase letters', async ({ request, session }) => {
		console.log('🧪 TEST: Generate password (lowercase only)');
		console.log('='.repeat(60));

		const result = await generatePassword(request, session, {
			lowercase: true,
			uppercase: false,
			numbers: false,
			symbols: false
		});

		expect(result.password).toBeTruthy();
		expect(result.password).toMatch(/^[a-z]+$/); // Only lowercase

		console.log(`✅ Generated password (lowercase only): ${result.password}`);
		console.log('🎉 TEST PASSED: Password character set filtering works');
		console.log('='.repeat(60));
	});

	test('should generate API key', async ({ request, session }) => {
		console.log('🧪 TEST: Generate API key');
		console.log('='.repeat(60));

		const result = await generateApiKey(request, session);

		// Verify API key format (typically longer than passwords)
		expect(result.api_key).toBeTruthy();
		expect(result.api_key.length).toBeGreaterThanOrEqual(32);

		// API key should be alphanumeric + special chars
		expect(result.api_key).toMatch(/^[A-Za-z0-9_-]+$/);

		// Verify seed and OTP
		expect(result.seed).toBeTruthy();
		expect(result.seed).toHaveLength(64);
		expect(result.otp).toBeTruthy();
		expect(result.otp).toHaveLength(6);

		console.log(`✅ Generated API key: ${result.api_key}`);
		console.log(`✅ Seed: ${result.seed}`);
		console.log(`✅ OTP: ${result.otp}`);
		console.log('🎉 TEST PASSED: API key generation works');
		console.log('='.repeat(60));
	});

	test('should generate BIP39 mnemonic with default parameters', async ({ request, session }) => {
		console.log('🧪 TEST: Generate BIP39 mnemonic (default 12 words)');
		console.log('='.repeat(60));

		const result = await generateMnemonic(request, session, {});

		// Verify mnemonic format
		expect(result.mnemonic).toBeTruthy();

		const words = result.mnemonic.split(' ');
		expect(words).toHaveLength(12); // Default: 12 words

		// Each word should be lowercase alphanumeric (BIP39 wordlist)
		words.forEach((word) => {
			expect(word).toMatch(/^[a-z]+$/);
			expect(word.length).toBeGreaterThanOrEqual(3); // BIP39 words are at least 3 chars
		});

		// Verify seed and OTP
		expect(result.seed).toBeTruthy();
		expect(result.seed).toHaveLength(64);
		expect(result.otp).toBeTruthy();
		expect(result.otp).toHaveLength(6);

		console.log(`✅ Generated mnemonic: ${result.mnemonic}`);
		console.log(`✅ Word count: ${words.length}`);
		console.log(`✅ Seed: ${result.seed}`);
		console.log(`✅ OTP: ${result.otp}`);
		console.log('🎉 TEST PASSED: BIP39 mnemonic generation works');
		console.log('='.repeat(60));
	});

	test('should generate BIP39 mnemonic with 24 words', async ({ request, session }) => {
		console.log('🧪 TEST: Generate BIP39 mnemonic (24 words)');
		console.log('='.repeat(60));

		const result = await generateMnemonic(request, session, { word_count: 24 });

		expect(result.mnemonic).toBeTruthy();

		const words = result.mnemonic.split(' ');
		expect(words).toHaveLength(24); // Custom word count

		console.log(`✅ Generated mnemonic (24 words): ${result.mnemonic}`);
		console.log('🎉 TEST PASSED: Custom word count works');
		console.log('='.repeat(60));
	});

	test('should return different hashes for multiple requests', async ({ request, session }) => {
		console.log('🧪 TEST: Multiple requests generate different hashes');
		console.log('='.repeat(60));

		// Generate 3 hashes
		const result1 = await generateCustomHash(request, session, {});
		const result2 = await generateCustomHash(request, session, {});
		const result3 = await generateCustomHash(request, session, {});

		// All hashes should be different
		expect(result1.hash).not.toBe(result2.hash);
		expect(result1.hash).not.toBe(result3.hash);
		expect(result2.hash).not.toBe(result3.hash);

		// All seeds should be different
		expect(result1.seed).not.toBe(result2.seed);
		expect(result1.seed).not.toBe(result3.seed);
		expect(result2.seed).not.toBe(result3.seed);

		console.log(`✅ Hash 1: ${result1.hash.substring(0, 20)}...`);
		console.log(`✅ Hash 2: ${result2.hash.substring(0, 20)}...`);
		console.log(`✅ Hash 3: ${result3.hash.substring(0, 20)}...`);
		console.log('🎉 TEST PASSED: Each request generates unique hash');
		console.log('='.repeat(60));
	});
});

test.describe('Hash Generation - Unauthenticated (should fail)', () => {
	test('should reject unauthenticated custom hash request', async ({ request }) => {
		console.log('🧪 TEST: Reject unauthenticated custom hash request');
		console.log('='.repeat(60));

		// Try to generate hash without authentication (no Bearer token)
		const response = await request.get('http://localhost:3000/api/custom?signature=fake');

		// Should return 401 Unauthorized
		expect(response.status()).toBe(401);

		console.log('✅ Request correctly rejected with 401 Unauthorized');
		console.log('🎉 TEST PASSED: Authentication required for hash generation');
		console.log('='.repeat(60));
	});

	test('should reject request with invalid signature', async ({ request, session }) => {
		console.log('🧪 TEST: Reject request with invalid Ed25519 signature');
		console.log('='.repeat(60));

		// First authenticate to get JWT token
		const _magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		// Don't navigate to magic link in browser, just extract token for API request

		// Extract auth data from session
		const authData = await session.getAuthData();

		// Make request with valid JWT but INVALID Ed25519 signature
		const response = await request.get(
			'http://localhost:3000/api/custom?signature=0000000000000000000000000000000000000000000000000000000000000000',
			{
				headers: {
					Authorization: `Bearer ${authData.access_token}`
				}
			}
		);

		// Should return 401 or 400 (invalid signature)
		expect([400, 401]).toContain(response.status());

		console.log(`✅ Request correctly rejected with ${response.status()}`);
		console.log('🎉 TEST PASSED: Ed25519 signature validation works');
		console.log('='.repeat(60));
	});
});
