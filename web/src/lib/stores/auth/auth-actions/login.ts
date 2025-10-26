/**
 * Login Module - Magic Link Authentication Flow
 *
 * Single Responsibility: Handle magic link authentication operations.
 * Part of auth-actions refactorization to apply SOLID principles.
 *
 * PURPOSE:
 * - Request magic link for email authentication
 * - Validate magic link and complete authentication
 * - Derive and store permanent user keys (Sistema B)
 * - Generate temporary crypto tokens (Sistema A)
 * - Publish permanent keys to backend
 *
 * FLOW:
 * 1. User requests magic link → requestMagicLink()
 * 2. User clicks link → validateMagicLink()
 * 3. Keys derived → Sistema A + Sistema B
 * 4. Keys published → /api/keys/rotate
 *
 * @see logout.ts for session termination
 * @see key-rotation.ts for key publication
 */

import type { AuthUser, LoginResponse, MagicLinkResponse } from '../../../types';
import { saveAuthToStorage } from '../auth-storage';
import { generateCryptoTokens, hasCryptoTokens } from '../auth-crypto-tokens';
import { logger } from '../../../utils/logger';
import { publishPermanentKeys } from './key-rotation';

/**
 * Request magic link for email authentication
 *
 * Initiates the authentication flow by requesting a magic link to be sent to user's email.
 * The magic link contains a token that will be validated in validateMagicLink().
 *
 * OPERATIONS:
 * 1. Extract UI domain for magic link generation
 * 2. Save email to IndexedDB (Zero Knowledge UX)
 * 3. Call API to send magic link email
 *
 * @param email - User email address
 * @param next - Optional Base58-encoded parameters to include in magic link URL
 * @returns Promise<MagicLinkResponse>
 */
export async function requestMagicLink(
	email: string,
	next: string = '/'
): Promise<MagicLinkResponse> {
	// Capture current UI host (domain only) for magic link generation and cookie Domain attribute
	const { extractDomain } = await import('../../../utils/domain-extractor');
	const ui_host = extractDomain();

	if (!ui_host) {
		throw new Error('UI host is required for magic link generation');
	}

	// Save email to IndexedDB for Zero Knowledge UX (will be retrieved during validateMagicLink)
	const { sessionManager } = await import('../../../session-manager');
	await sessionManager.setPendingAuthEmail(email);

	logger.debug('[login] requestMagicLink called', { email, ui_host, next });

	const { api } = await import('../../../api');
	return await api.requestMagicLink(email, ui_host, next);
}

/**
 * Validate magic link and complete authentication
 *
 * Completes the authentication flow by validating the magic link token.
 * This is the most complex function in the auth flow.
 *
 * OPERATIONS:
 * 1. Validate magic link token with backend
 * 2. Decrypt user private key context (Sistema B seed)
 * 3. Derive permanent user keys (Sistema B - Ed25519/X25519)
 * 4. Store derived keys in IndexedDB
 * 5. Save user and access token
 * 6. Store session expiration timestamp
 * 7. Show success flash message
 * 8. Generate temporary crypto tokens (Sistema A)
 * 9. Publish permanent keys to backend
 * 10. Initialize confirm-read cache
 * 11. Clear pending auth email
 *
 * @param magicToken - Magic link token from URL parameter (Ed25519 verified by backend)
 * @returns Promise<{ user: AuthUser; accessToken: string; loginResponse: LoginResponse }>
 */
export async function validateMagicLink(magicToken: string): Promise<{
	user: AuthUser;
	accessToken: string;
	loginResponse: LoginResponse;
}> {
	logger.debug('[login] validateMagicLink called with token:', {
		tokenLength: magicToken.length,
		tokenPrefix: magicToken.substring(0, 20) + '...'
	});

	const { api } = await import('../../../api');
	const loginResponse = await api.validateMagicLink(magicToken);
	logger.debug('[login] Magic link validation successful, processing response');

	// =========================================================================
	// STEP 1: Decrypt user private key context (REQUIRED in magic link validation)
	// =========================================================================
	if (!loginResponse.encrypted_privkey_context) {
		throw new Error('Missing encrypted_privkey_context in magic link response');
	}
	if (!loginResponse.server_x25519_pub_key) {
		throw new Error('Missing server_x25519_pub_key in magic link response');
	}

	let privkeyContext: Uint8Array;
	try {
		const { decryptPrivkeyContext } = await import('../../../crypto/shared-secret-crypto');
		privkeyContext = await decryptPrivkeyContext(
			loginResponse.encrypted_privkey_context,
			loginResponse.server_x25519_pub_key
		);

		logger.debug('[login] ✅ Privkey context decrypted successfully:', {
			size: privkeyContext.length,
			first_8_bytes: Array.from(privkeyContext.slice(0, 8)),
			last_8_bytes: Array.from(privkeyContext.slice(-8))
		});
	} catch (error) {
		logger.error('[login] ❌ Failed to decrypt privkey_context:', error);
		throw new Error(`Privkey context decryption failed: ${error}`);
	}

	// =========================================================================
	// STEP 2: Derive user's permanent Ed25519/X25519 keypairs from privkey_context (Sistema B)
	// =========================================================================
	let derivedKeys: Awaited<ReturnType<typeof import('../../../crypto/user-key-derivation').deriveUserKeys>> | null = null;

	try {
		const { deriveUserKeys } = await import('../../../crypto/user-key-derivation');

		// Get pending email for key derivation (Zero Knowledge UX)
		const { sessionManager } = await import('../../../session-manager');
		const userEmail = (await sessionManager.getPendingAuthEmail()) ?? '';

		if (!userEmail) {
			throw new Error('User email not available for key derivation');
		}

		logger.debug('[login] 🔑 Deriving user keypairs from privkey_context:', {
			email: userEmail,
			privkeyContextLength: privkeyContext.length
		});

		// Derive deterministic keypairs (Sistema B - permanent keys)
		derivedKeys = await deriveUserKeys(userEmail, privkeyContext);

		logger.info('[login] 🔐 Derived user public keys (Sistema B):', {
			ed25519: derivedKeys.ed25519.publicKeyHex,
			x25519: derivedKeys.x25519.publicKeyHex
		});

		// NOTE: User's public keys are not returned in LoginResponse
		// They are deterministically derived from privkey_context + email
		logger.debug('[login] ℹ️ User public keys are deterministically derived from privkey_context');

		// Store derived keys in IndexedDB for future use
		const { storeDerivedUserKeys } = await import('../../../crypto/keypair-storage');
		await storeDerivedUserKeys(derivedKeys);
		logger.debug('[login] ✅ Derived user keys (Sistema B) stored in IndexedDB');

		// NOTE: Store derivedKeys for later publication (after JWT + crypto tokens exist)
		// Publication happens AFTER saveAuthToStorage + generateCryptoTokens
	} catch (error) {
		logger.error('[login] ❌ Failed to derive user keys:', error);
		// Non-blocking - authentication continues even if key derivation fails
	}

	// =========================================================================
	// STEP 3: Create AuthUser object and save to IndexedDB
	// =========================================================================
	const { sessionManager } = await import('../../../session-manager');
	const userEmail = (await sessionManager.getPendingAuthEmail()) ?? '';

	const user: AuthUser = {
		user_id: loginResponse.user_id, // Base58 user_id
		email: userEmail, // User email for UX display (Zero Knowledge compliant)
		isAuthenticated: true
	};

	// Save to IndexedDB
	await saveAuthToStorage(user, loginResponse.access_token);

	// =========================================================================
	// STEP 4: Store session expiration timestamp if provided (new refresh cookie)
	// =========================================================================
	if (loginResponse.expires_at) {
		try {
			const { storeSessionExpiration } = await import('../../../session-storage');
			await storeSessionExpiration(loginResponse.expires_at);
		} catch (error) {
			logger.warn('Failed to store session expiration:', error);
			// Non-blocking - auth continues without persistent expiration tracking
		}
	}

	// =========================================================================
	// STEP 5: Show flash message for successful magic link validation
	// =========================================================================
	try {
		const { flashMessagesStore } = await import('../../flashMessages');
		const { currentLanguage, t } = await import('../../i18n');

		// Get current language
		let lang = 'en';
		const unsubscribe = currentLanguage.subscribe((l: string) => (lang = l));
		unsubscribe();

		// Show translated message
		const message = t('auth.magicLinkValidatedSuccess', lang);
		flashMessagesStore.addMessage(message);
	} catch {
		// Failed to show magic link success flash message
	}

	// =========================================================================
	// STEP 6: Generate crypto tokens ONLY if they don't exist yet (Sistema A)
	// =========================================================================
	const tokensExist = await hasCryptoTokens();
	if (!tokensExist) {
		// Magic link: No crypto tokens found, generating (Sistema A - temporary session keys)
		await generateCryptoTokens();
	} else {
		// Magic link: Crypto tokens already exist (Sistema A)
	}

	// =========================================================================
	// STEP 7: Publish permanent public keys to backend (Sistema B - E2EE)
	// =========================================================================
	// NOW it's safe: JWT exists (saveAuthToStorage) + crypto tokens exist (generateCryptoTokens)
	if (derivedKeys) {
		try {
			await publishPermanentKeys(derivedKeys);
		} catch (publishError) {
			logger.error('[login] ❌ Failed to publish permanent keys:', publishError);
			// Non-blocking - authentication continues even if publication fails
			// Keys will be republished on next login attempt
		}
	} else {
		logger.warn('[login] ⚠️ No derived keys available for publication (key derivation may have failed)');
	}

	// =========================================================================
	// STEP 8: Initialize confirm-read cache database (ensures DB is ready before first use)
	// =========================================================================
	try {
		const { initConfirmReadCache } = await import('../../../utils/confirm-read-cache');
		await initConfirmReadCache();
	} catch {
		// Failed to initialize confirm-read cache (non-blocking)
	}

	// =========================================================================
	// STEP 9: Clear pending auth email - no longer needed after successful authentication
	// =========================================================================
	try {
		const { sessionManager } = await import('../../../session-manager');
		await sessionManager.clearPendingAuthEmail();
	} catch {
		// Failed to clear pending auth email from IndexedDB
	}

	return { user, accessToken: loginResponse.access_token, loginResponse };
}
