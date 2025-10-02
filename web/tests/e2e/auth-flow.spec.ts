/**
 * Authentication Flow E2E Test
 *
 * Tests the complete magic link authentication flow:
 * 1. Request magic link with Ed25519 signature
 * 2. Extract dev_magic_link from signed response
 * 3. Navigate to magic link
 * 4. Verify redirect to home page
 * 5. Verify authentication state (user_id, access_token)
 *
 * Replicates bash test pattern from scripts/final_test.sh
 */

import { test, expect } from '../utils/test-auth-helpers';
import { requestMagicLink, loginWithMagicLink } from '../utils/test-auth-helpers';

test.describe('Authentication Flow', () => {
	test('should complete full magic link authentication flow', async ({
		page,
		request,
		session
	}) => {
		console.log('🧪 TEST: Full magic link authentication flow');
		console.log('='.repeat(60));

		// Step 1: Request magic link
		console.log('\n📧 Step 1: Requesting magic link...');
		const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');

		expect(magicLink).toBeTruthy();
		expect(magicLink).toContain('http://localhost:5173/auth/callback');
		console.log(`✅ Magic link received: ${magicLink.substring(0, 60)}...`);

		// Step 2: Complete login by navigating to magic link
		console.log('\n🔗 Step 2: Navigating to magic link...');
		await loginWithMagicLink(page, session, magicLink);

		// Step 3: Verify authentication state
		console.log('\n✅ Step 3: Verifying authentication state...');
		const authData = await session.getAuthData();

		expect(authData.user).toBeTruthy();
		expect(authData.user?.isAuthenticated).toBe(true);
		expect(authData.user?.user_id).toBeTruthy();
		expect(authData.user?.user_id).toHaveLength(64); // Blake3 hash = 32 bytes = 64 hex chars

		expect(authData.access_token).toBeTruthy();
		expect(authData.access_token).toContain('.'); // JWT format: header.payload.signature

		console.log(`✅ Authenticated as user: ${authData.user!.user_id.substring(0, 20)}...`);
		console.log(`✅ Access token: ${authData.access_token!.substring(0, 30)}...`);

		// Step 4: Verify keypair exists
		console.log('\n🔑 Step 4: Verifying Ed25519 keypair...');
		const keyPair = await session.getKeyPair();

		expect(keyPair).toBeTruthy();
		expect(keyPair?.publicKeyBytes).toBeTruthy();
		expect(keyPair?.publicKeyBytes).toHaveLength(32); // Ed25519 public key = 32 bytes

		// Private key should exist (for signing requests)
		expect(keyPair?.privateKeyBytes).toBeTruthy();
		expect(keyPair?.privateKeyBytes).toHaveLength(32); // Ed25519 private key = 32 bytes

		console.log('✅ Ed25519 keypair verified');

		// Step 5: Verify server public key exists
		console.log('\n🔐 Step 5: Verifying server public key...');
		const serverPubKey = await session.getServerPubKey();

		expect(serverPubKey).toBeTruthy();
		expect(serverPubKey).toHaveLength(64); // Ed25519 server public key = 32 bytes = 64 hex chars

		console.log(`✅ Server public key: ${serverPubKey!.substring(0, 20)}...`);

		// Step 6: Verify we can see authenticated UI elements
		console.log('\n🎨 Step 6: Verifying authenticated UI...');

		// Check that we're on the home page
		await expect(page).toHaveURL('http://localhost:5173/');

		// Wait for auth status button to be visible
		const authStatusButton = page.locator('button:has-text("Logged")');
		await expect(authStatusButton).toBeVisible({ timeout: 5000 });

		console.log('✅ Authenticated UI verified');

		// Step 7: Verify session persistence (reload page)
		console.log('\n🔄 Step 7: Testing session persistence...');
		await page.reload();

		// Wait for page to load
		await page.waitForLoadState('networkidle');

		// Auth status button should still be visible
		await expect(authStatusButton).toBeVisible({ timeout: 5000 });

		// Extract auth data from IndexedDB after reload
		const authDataAfterReload = await page.evaluate(async () => {
			// @ts-expect-error - Dynamic import path works in browser runtime via Vite
			const { sessionManager } = await import('/src/lib/session-manager');
			return await sessionManager.getAuthData();
		});

		expect(authDataAfterReload.user).toBeTruthy();
		expect(authDataAfterReload.user?.user_id).toBe(authData.user!.user_id);
		expect(authDataAfterReload.access_token).toBeTruthy();

		console.log('✅ Session persisted after page reload');

		console.log('\n🎉 TEST PASSED: Full authentication flow completed successfully');
		console.log('='.repeat(60));
	});

	test('should handle invalid magic link gracefully', async ({ page }) => {
		console.log('🧪 TEST: Invalid magic link handling');
		console.log('='.repeat(60));

		// Navigate to invalid magic link (missing token)
		await page.goto('http://localhost:5173/auth/callback?invalid=true');

		// Should redirect to home page
		await page.waitForURL('http://localhost:5173/', { timeout: 10000 });

		// Should not be authenticated (no "Logged" button visible)
		const authStatusButton = page.locator('button:has-text("Logged")');
		await expect(authStatusButton).not.toBeVisible();

		console.log('✅ Invalid magic link handled correctly (redirected to home)');
		console.log('🎉 TEST PASSED: Invalid magic link handling works');
		console.log('='.repeat(60));
	});

	test('should handle expired magic link gracefully', async ({ page }) => {
		console.log('🧪 TEST: Expired magic link handling');
		console.log('='.repeat(60));

		// Navigate to magic link with expired token (using obviously expired JWT)
		const expiredToken =
			'eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSJ9.eyJleHAiOjE2MDAwMDAwMDAsInVzZXJfaWQiOiJ0ZXN0In0.invalid';
		await page.goto(`http://localhost:5173/auth/callback?token=${expiredToken}`);

		// Should redirect to home page
		await page.waitForURL('http://localhost:5173/', { timeout: 10000 });

		// Should not be authenticated
		const authStatusButton = page.locator('button:has-text("Logged")');
		await expect(authStatusButton).not.toBeVisible();

		console.log('✅ Expired magic link handled correctly (redirected to home)');
		console.log('🎉 TEST PASSED: Expired magic link handling works');
		console.log('='.repeat(60));
	});
});
