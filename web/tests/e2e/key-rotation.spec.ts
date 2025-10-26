/**
 * Ed25519 Key Rotation E2E Test
 *
 * Tests the complete 2/3 key rotation system:
 * - PERIOD 1/3 (0-40s): Token refresh only, no key rotation
 * - PERIOD 2/3 (40-120s): Full key rotation with new Ed25519 keypair
 * - PERIOD 3/3 (120s+): Both tokens expired, requires re-authentication
 *
 * CRITICAL TEST - Validates v1.6.23 bug fix:
 * - Refresh token now correctly contains client pub_key
 * - Backend validates old pub_key and issues new tokens with new pub_key
 * - Client seamlessly switches to new keypair after rotation
 *
 * TIMING REQUIREMENTS:
 * - Wait ~110s to enter PERIOD 2/3 window (40s-120s)
 * - Complete key rotation before 120s refresh token expiration
 * - This test takes approximately 3 minutes to complete
 *
 * Replicates bash test patterns from scripts/test_2_3_system.sh (Test 3)
 */

import { test, expect } from '../utils/test-auth-helpers';
import {
	requestMagicLink,
	loginWithMagicLink,
	generateCustomHash,
	waitForSeconds
} from '../utils/test-auth-helpers';
import { publicKeyBytesToHex } from '../../src/lib/ed25519/ed25519-core';

test.describe('Ed25519 Key Rotation System', () => {
	test('should rotate Ed25519 keys in PERIOD 2/3 window', async ({ page, request, session }) => {
		console.log('🧪 TEST: Ed25519 Key Rotation (PERIOD 2/3 System)');
		console.log('='.repeat(60));
		console.log('⏰ PERIOD 1/3 (0-40s): Token refresh only');
		console.log('⏰ PERIOD 2/3 (40-120s): KEY ROTATION window');
		console.log('⏰ PERIOD 3/3 (120s+): Both tokens expired');
		console.log('='.repeat(60));
		console.log('⚠️  WARNING: This test takes ~3 minutes to complete');
		console.log('='.repeat(60));

		// PHASE 1: Initial authentication (t=0s)
		console.log('\n📍 PHASE 1: Initial authentication (t=0s)');
		console.log('-'.repeat(60));

		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		const initialAuthData = await session.getAuthData();
		const initialAccessToken = initialAuthData.access_token!;
		const initialUserId = initialAuthData.user!.user_id;

		// Store initial keypair for comparison
		const initialKeyPair = await session.getKeyPair();
		expect(initialKeyPair).toBeTruthy();
		expect(initialKeyPair?.publicKeyBytes).toBeTruthy();

		const initialPubKeyHex = publicKeyBytesToHex(initialKeyPair!.publicKeyBytes);

		console.log(`✅ User ID: ${initialUserId.substring(0, 20)}...`);
		console.log(`✅ Initial access token: ${initialAccessToken.substring(0, 30)}...`);
		console.log(`✅ Initial pub_key: ${initialPubKeyHex.substring(0, 20)}...`);

		// PHASE 2: Generate hash with initial keypair (t=0s)
		console.log('\n📍 PHASE 2: Verify initial keypair works (t=0s)');
		console.log('-'.repeat(60));

		const hash1 = await generateCustomHash(request, session, {});
		expect(hash1.hash).toBeTruthy();

		console.log(`✅ Hash generated with initial keypair: ${hash1.hash.substring(0, 20)}...`);

		// PHASE 3: Wait for PERIOD 2/3 window (110s total)
		console.log('\n📍 PHASE 3: Waiting for PERIOD 2/3 key rotation window...');
		console.log('-'.repeat(60));
		console.log('⏰ PERIOD 1/3 ends at 40s (2/3 of 60s = 40s)');
		console.log('⏰ PERIOD 2/3 active from 40s to 120s');
		console.log('⏰ Waiting 110s to be well within PERIOD 2/3 window');
		console.log('');
		console.log('⏳ This will take approximately 2 minutes...');
		console.log('☕ Good time for coffee! ☕');

		await waitForSeconds(110, 'PERIOD 2/3 key rotation window');

		console.log('✅ Wait complete - now in PERIOD 2/3 window (t=110s)');

		// PHASE 4: Trigger key rotation by using authenticated feature
		console.log('\n📍 PHASE 4: Trigger key rotation');
		console.log('-'.repeat(60));
		console.log('🔄 Navigating to generator page to trigger key rotation...');

		// Navigate to generator page (requires authentication)
		await page.goto('http://localhost:5173/generator');

		// Wait for page to load and handle key rotation
		await page.waitForLoadState('networkidle');

		// Give frontend time to complete rotation (2-3 seconds)
		await page.waitForTimeout(3000);

		console.log('✅ Navigation complete - key rotation should have occurred');

		// PHASE 5: Extract new keypair and verify rotation
		console.log('\n📍 PHASE 5: Verify key rotation occurred');
		console.log('-'.repeat(60));

		// Extract new auth data from browser IndexedDB
		const rotatedAuthData = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return {
				authData: await sessionManager.getAuthData(),
				keyPairHex: await sessionManager.getKeyPairHex()
			};
		});

		expect(rotatedAuthData.authData.user).toBeTruthy();
		expect(rotatedAuthData.authData.access_token).toBeTruthy();
		expect(rotatedAuthData.keyPairHex).toBeTruthy();

		const newAccessToken = rotatedAuthData.authData.access_token!;
		const newUserId = rotatedAuthData.authData.user!.user_id;
		const newPubKeyHex = rotatedAuthData.keyPairHex!.publicKeyHex;

		// Verify access token was refreshed (should be different)
		expect(newAccessToken).not.toBe(initialAccessToken);
		console.log(`✅ Access token refreshed: ${newAccessToken.substring(0, 30)}...`);

		// Verify user_id remains the same (no re-authentication)
		expect(newUserId).toBe(initialUserId);
		console.log(`✅ User ID unchanged: ${newUserId.substring(0, 20)}...`);

		// CRITICAL: Verify pub_key changed (key rotation occurred!)
		expect(newPubKeyHex).not.toBe(initialPubKeyHex);
		console.log(`✅ NEW pub_key: ${newPubKeyHex.substring(0, 20)}...`);
		console.log(`✅ OLD pub_key: ${initialPubKeyHex.substring(0, 20)}...`);
		console.log('✅ KEY ROTATION CONFIRMED - pub_key changed!');

		// Update test session with new keypair and tokens
		await session.setAuthData(newUserId, newAccessToken);
		await session.setKeyPairFromHex(
			rotatedAuthData.keyPairHex!.privateKeyHex,
			rotatedAuthData.keyPairHex!.publicKeyHex
		);

		// PHASE 6: Verify new keypair works for signing requests
		console.log('\n📍 PHASE 6: Verify new keypair works for API requests');
		console.log('-'.repeat(60));

		const hash2 = await generateCustomHash(request, session, {});
		expect(hash2.hash).toBeTruthy();
		expect(hash2.seed).toBeTruthy();
		expect(hash2.otp).toBeTruthy();

		// Hash should be different (different keypair = different signatures)
		expect(hash2.hash).not.toBe(hash1.hash);
		expect(hash2.seed).not.toBe(hash1.seed);

		console.log(`✅ Hash generated with NEW keypair: ${hash2.hash.substring(0, 20)}...`);
		console.log('✅ New keypair successfully signs requests');

		// PHASE 7: Verify multiple requests work with rotated keypair
		console.log('\n📍 PHASE 7: Verify sustained operation with new keypair');
		console.log('-'.repeat(60));

		// Generate 3 more hashes to ensure stability
		const additionalHashes: string[] = [];
		for (let i = 1; i <= 3; i++) {
			const result = await generateCustomHash(request, session, {});
			additionalHashes.push(result.hash);
			console.log(`✅ Hash ${i + 2}: ${result.hash.substring(0, 20)}...`);
		}

		// All hashes should be unique
		const allHashes = [hash1.hash, hash2.hash, ...additionalHashes];
		const uniqueHashes = new Set(allHashes);
		expect(uniqueHashes.size).toBe(5);

		console.log('✅ All 5 hashes are unique');
		console.log('✅ Rotated keypair works consistently');

		// PHASE 8: Test complete summary
		console.log('\n🎉 TEST PASSED: Ed25519 Key Rotation Complete!');
		console.log('='.repeat(60));
		console.log('📊 ROTATION SUMMARY:');
		console.log(`   - Initial pub_key: ${initialPubKeyHex.substring(0, 30)}...`);
		console.log(`   - New pub_key:     ${newPubKeyHex.substring(0, 30)}...`);
		console.log(`   - Initial token:   ${initialAccessToken.substring(0, 20)}...`);
		console.log(`   - New token:       ${newAccessToken.substring(0, 20)}...`);
		console.log(`   - User ID:         ${initialUserId.substring(0, 20)}... (unchanged)`);
		console.log('');
		console.log('✅ Key rotation occurred at t=110s (PERIOD 2/3)');
		console.log('✅ Old pub_key → New pub_key');
		console.log('✅ Old access token → New access token');
		console.log('✅ User ID preserved (no re-authentication)');
		console.log('✅ New keypair signs requests successfully');
		console.log('✅ v1.6.23 bug fix validated - pub_key rotation works!');
		console.log('='.repeat(60));
	});

	test('should maintain authentication across multiple refresh cycles', async ({
		page,
		request,
		session
	}) => {
		console.log('🧪 TEST: Multiple refresh cycles without re-authentication');
		console.log('='.repeat(60));
		console.log('⏰ This test validates sustained authentication over 50s');
		console.log('='.repeat(60));

		// Authenticate
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		const initialUserId = (await session.getAuthData()).user!.user_id;
		console.log(`✅ Initial user_id: ${initialUserId.substring(0, 20)}...`);

		// Generate hash at t=0s
		const hash1 = await generateCustomHash(request, session, {});
		console.log(`✅ Hash 1 (t=0s): ${hash1.hash.substring(0, 20)}...`);

		// Wait 25s (first token refresh - PERIOD 1/3)
		await waitForSeconds(25, 'First token refresh window');

		// Navigate to trigger refresh
		await page.goto('http://localhost:5173/generator');
		await page.waitForLoadState('networkidle');

		// Update session with new token
		const authData1 = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return await sessionManager.getAuthData();
		});
		await session.setAuthData(authData1.user!.user_id, authData1.access_token!);

		// Generate hash after first refresh
		const hash2 = await generateCustomHash(request, session, {});
		console.log(`✅ Hash 2 (t=25s, after 1st refresh): ${hash2.hash.substring(0, 20)}...`);

		// Wait another 25s (second token refresh - still PERIOD 1/3)
		await waitForSeconds(25, 'Second token refresh window');

		// Navigate again to trigger second refresh
		await page.goto('http://localhost:5173/');
		await page.waitForLoadState('networkidle');

		// Update session with new token
		const authData2 = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return await sessionManager.getAuthData();
		});
		await session.setAuthData(authData2.user!.user_id, authData2.access_token!);

		// Generate hash after second refresh
		const hash3 = await generateCustomHash(request, session, {});
		console.log(`✅ Hash 3 (t=50s, after 2nd refresh): ${hash3.hash.substring(0, 20)}...`);

		// Verify user_id remained constant
		expect(authData1.user!.user_id).toBe(initialUserId);
		expect(authData2.user!.user_id).toBe(initialUserId);

		// All hashes should be unique
		expect(hash1.hash).not.toBe(hash2.hash);
		expect(hash2.hash).not.toBe(hash3.hash);
		expect(hash1.hash).not.toBe(hash3.hash);

		console.log('✅ User remained authenticated across 2 refresh cycles');
		console.log('✅ All hashes unique (different seeds)');
		console.log('🎉 TEST PASSED: Sustained authentication works');
		console.log('='.repeat(60));
	});
});

test.describe('Key Rotation Edge Cases', () => {
	test('should handle UI interaction during key rotation', async ({ page, request, session }) => {
		console.log('🧪 TEST: UI interaction during key rotation');
		console.log('='.repeat(60));

		// Authenticate
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		// Wait for PERIOD 2/3 window
		console.log('⏳ Waiting for key rotation window (110s)...');
		await waitForSeconds(110, 'Key rotation window');

		// Try to generate multiple hashes during rotation
		console.log('🔄 Generating hashes during rotation window...');

		await page.goto('http://localhost:5173/generator');
		await page.waitForLoadState('networkidle');

		// Update session
		const authData = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return {
				authData: await sessionManager.getAuthData(),
				keyPairHex: await sessionManager.getKeyPairHex()
			};
		});

		await session.setAuthData(authData.authData.user!.user_id, authData.authData.access_token!);
		if (authData.keyPairHex) {
			await session.setKeyPairFromHex(
				authData.keyPairHex.privateKeyHex,
				authData.keyPairHex.publicKeyHex
			);
		}

		// Generate hashes - should work with rotated keypair
		const hash1 = await generateCustomHash(request, session, {});
		const hash2 = await generateCustomHash(request, session, {});

		expect(hash1.hash).toBeTruthy();
		expect(hash2.hash).toBeTruthy();
		expect(hash1.hash).not.toBe(hash2.hash);

		console.log(`✅ Hash 1: ${hash1.hash.substring(0, 20)}...`);
		console.log(`✅ Hash 2: ${hash2.hash.substring(0, 20)}...`);
		console.log('🎉 TEST PASSED: UI works during rotation');
		console.log('='.repeat(60));
	});
});
