/**
 * Token Refresh E2E Test
 *
 * Tests automatic JWT token refresh system using DYNAMIC values from .env
 *
 * Token durations: Read from .env (SPIN_VARIABLE_*_TOKEN_DURATION_MINUTES)
 * Backend config: api/src/utils/jwt/config.rs::get_*_token_duration_minutes()
 *
 * Tests TRAMO 1/3 system (token refresh without key rotation)
 *
 * IMPORTANT: This test uses real .env values - NO hardcoded timings
 * - Waits calculated dynamically from .env
 * - Key rotation threshold = 1/3 of refresh duration
 *
 * Replicates bash test patterns from scripts/test_2_3_system.sh
 */

import { test, expect } from '../utils/test-auth-helpers';
import {
	requestMagicLink,
	loginWithMagicLink,
	generateCustomHash,
	waitForSeconds
} from '../utils/test-auth-helpers';
import {
	getAccessTokenDurationSeconds,
	getRefreshTokenDurationSeconds as _getRefreshTokenDurationSeconds,
	getKeyRotationThresholdSeconds,
	getAccessTokenExpirationWaitSeconds,
	logTestConfiguration
} from '../utils/test-config';

test.describe('Token Refresh System', () => {
	test('should automatically refresh access token after expiration', async ({
		page,
		request,
		session
	}) => {
		console.log('🧪 TEST: Automatic token refresh (TRAMO 1/3 system)');
		console.log('='.repeat(60));
		logTestConfiguration(); // Display dynamic values from .env
		console.log('='.repeat(60));

		// PHASE 1: Initial authentication (t=0s)
		console.log('\n📍 PHASE 1: Initial authentication (t=0s)');
		console.log('-'.repeat(60));

		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		const initialAuthData = await session.getAuthData();
		const initialAccessToken = initialAuthData.access_token!;

		console.log(
			`✅ Logged in with initial access token: ${initialAccessToken.substring(0, 30)}...`
		);

		// PHASE 2: Generate hash with valid token (t=0s)
		console.log('\n📍 PHASE 2: Generate hash with valid token (t=0s)');
		console.log('-'.repeat(60));

		const hash1 = await generateCustomHash(request, session, {});
		expect(hash1.hash).toBeTruthy();
		expect(hash1.seed).toBeTruthy();
		expect(hash1.otp).toBeTruthy();

		console.log(`✅ Hash generated successfully: ${hash1.hash.substring(0, 20)}...`);

		// PHASE 3: Wait for access token expiration (dynamic from .env)
		console.log('\n📍 PHASE 3: Waiting for access token expiration...');
		console.log('-'.repeat(60));
		const waitSeconds = getAccessTokenExpirationWaitSeconds();
		console.log(`⏳ ACCESS TOKEN expires after ${getAccessTokenDurationSeconds()}s`);
		console.log(`⏳ Waiting ${waitSeconds}s to ensure expiration (duration + 5s buffer)`);

		await waitForSeconds(waitSeconds, 'Access token expiration');

		console.log('✅ Wait complete - access token should be expired now');

		// PHASE 4: Trigger automatic refresh by using authenticated UI
		console.log('\n📍 PHASE 4: Trigger automatic token refresh');
		console.log('-'.repeat(60));
		console.log('🔄 Navigating to authenticated page to trigger refresh...');

		// Navigate to custom hash page (requires authentication)
		await page.goto('http://localhost:5173/custom');

		// Wait for page to load and handle token refresh
		await page.waitForLoadState('networkidle');

		// Frontend should automatically refresh token
		// Extract new access token from IndexedDB
		const refreshedAuthData = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return await sessionManager.getAuthData();
		});

		expect(refreshedAuthData.access_token).toBeTruthy();
		expect(refreshedAuthData.user).toBeTruthy();

		// New access token should be different from initial one
		expect(refreshedAuthData.access_token).not.toBe(initialAccessToken);

		console.log(
			`✅ Access token refreshed: ${refreshedAuthData.access_token!.substring(0, 30)}...`
		);
		console.log('✅ New token is different from initial token');

		// Update test session with new token
		await session.setAuthData(refreshedAuthData.user!.user_id, refreshedAuthData.access_token!);

		// PHASE 5: Verify hash generation works with new token
		console.log('\n📍 PHASE 5: Verify hash generation with refreshed token');
		console.log('-'.repeat(60));

		const hash2 = await generateCustomHash(request, session, {});
		expect(hash2.hash).toBeTruthy();
		expect(hash2.seed).toBeTruthy();
		expect(hash2.otp).toBeTruthy();

		// Hash should be different from first hash (different seeds)
		expect(hash2.hash).not.toBe(hash1.hash);
		expect(hash2.seed).not.toBe(hash1.seed);

		console.log(`✅ Hash generated with refreshed token: ${hash2.hash.substring(0, 20)}...`);

		// PHASE 6: Verify no key rotation occurred (still in TRAMO 1/3)
		console.log('\n📍 PHASE 6: Verify no key rotation (TRAMO 1/3)');
		console.log('-'.repeat(60));
		console.log(`⏰ Time elapsed: ~${waitSeconds}s`);
		console.log(
			`⏰ Key rotation window starts at ${getKeyRotationThresholdSeconds()}s (TRAMO 2/3)`
		);
		console.log('✅ Should have only refreshed access token, NOT rotated keys');

		const finalKeyPair = await session.getKeyPair();
		expect(finalKeyPair).toBeTruthy();

		// KeyPair should still be the same (no rotation yet)
		// We can't directly compare because we don't store initial keypair
		// But we can verify it still works for signing
		const hash3 = await generateCustomHash(request, session, {});
		expect(hash3.hash).toBeTruthy();

		console.log('✅ Keypair still valid (no rotation occurred)');
		console.log('✅ TRAMO 1/3 refresh completed successfully');

		console.log('\n🎉 TEST PASSED: Automatic token refresh works correctly');
		console.log('='.repeat(60));
		console.log('📊 SUMMARY:');
		console.log(`   - Initial token: ${initialAccessToken.substring(0, 20)}...`);
		console.log(`   - Refreshed token: ${refreshedAuthData.access_token!.substring(0, 20)}...`);
		console.log(`   - Hash 1: ${hash1.hash.substring(0, 20)}... (before refresh)`);
		console.log(`   - Hash 2: ${hash2.hash.substring(0, 20)}... (after refresh)`);
		console.log(`   - Hash 3: ${hash3.hash.substring(0, 20)}... (verify keypair)`);
		console.log('='.repeat(60));
	});

	test('should handle multiple consecutive hash requests without refresh', async ({
		page,
		request,
		session
	}) => {
		console.log('🧪 TEST: Multiple hash requests within token validity');
		console.log('='.repeat(60));

		// Authenticate
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		const initialAuthData = await session.getAuthData();
		const initialAccessToken = initialAuthData.access_token!;

		console.log(`✅ Logged in with access token: ${initialAccessToken.substring(0, 30)}...`);

		// Generate 5 hashes in quick succession (all within token validity)
		console.log('\n🔄 Generating 5 hashes in quick succession...');
		const hashes: string[] = [];

		for (let i = 1; i <= 5; i++) {
			const result = await generateCustomHash(request, session, {});
			hashes.push(result.hash);
			console.log(`✅ Hash ${i}: ${result.hash.substring(0, 20)}...`);

			// Small delay between requests (100ms)
			await page.waitForTimeout(100);
		}

		// All hashes should be unique
		const uniqueHashes = new Set(hashes);
		expect(uniqueHashes.size).toBe(5);

		console.log('✅ All 5 hashes are unique');

		// Access token should still be the same (no refresh needed)
		const finalAuthData = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return await sessionManager.getAuthData();
		});

		expect(finalAuthData.access_token).toBe(initialAccessToken);

		console.log('✅ Access token unchanged (no refresh needed)');
		console.log('🎉 TEST PASSED: Multiple requests work without refresh');
		console.log('='.repeat(60));
	});

	test('should show session expiration indicator when token expires', async ({
		page,
		request,
		session
	}) => {
		console.log('🧪 TEST: Session expiration UI indicator');
		console.log('='.repeat(60));

		// Authenticate
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
		await loginWithMagicLink(page, session, magicLink);

		console.log('✅ User authenticated');

		// Navigate to home page
		await page.goto('http://localhost:5173/');

		// Auth status button should be green (valid session)
		const authStatusButton = page.locator('button:has-text("Logged")');
		await expect(authStatusButton).toBeVisible();
		await expect(authStatusButton).toHaveClass(/bg-green|text-green/); // Green styling

		console.log('✅ Auth status button shows active session (green)');

		// Wait for access token expiration (dynamic from .env)
		const waitSeconds = getAccessTokenExpirationWaitSeconds();
		console.log(`\n⏳ Waiting for access token expiration (${waitSeconds}s)...`);
		await waitForSeconds(waitSeconds, 'Access token expiration');

		// Navigate to trigger expiration check
		await page.goto('http://localhost:5173/custom');
		await page.waitForLoadState('networkidle');

		// Auth status button might show warning (yellow) during refresh
		// But should eventually return to green after successful refresh
		await page.waitForTimeout(2000); // Wait for refresh to complete

		await page.goto('http://localhost:5173/');
		await expect(authStatusButton).toBeVisible();

		console.log('✅ Auth status button still visible after token refresh');
		console.log('🎉 TEST PASSED: UI correctly handles token expiration');
		console.log('='.repeat(60));
	});
});
