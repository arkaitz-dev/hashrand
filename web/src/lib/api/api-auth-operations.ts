/**
 * API Auth Operations Module - Authentication Endpoints
 *
 * Single Responsibility: Handle authentication-related API operations
 * Part of api.ts refactorization to apply SRP and organize by domain
 */

import type { LoginResponse, MagicLinkResponse } from '../types';

const API_BASE = '/api';

/**
 * Request magic link for email authentication
 */
export async function requestMagicLink(
	email: string,
	ui_host: string,
	next: string = '/'
): Promise<MagicLinkResponse> {
	// Generate or retrieve Ed25519 keypair
	const { getOrCreateKeyPair, publicKeyToHex } = await import('../ed25519');
	const keyPair = await getOrCreateKeyPair();
	const pubKeyHex = publicKeyToHex(keyPair.publicKeyBytes);

	// Get current language for email template (REQUIRED)
	let email_lang: string = 'en'; // Default fallback
	try {
		const { currentLanguage } = await import('../stores/i18n');
		const { get } = await import('svelte/store');
		email_lang = get(currentLanguage);
		// Email language from i18n store
	} catch {
		// Fallback to browser language detection
		if (typeof navigator !== 'undefined') {
			email_lang = navigator.language.split('-')[0].toLowerCase();
			// Email language from browser fallback
		} else {
			// Email language using default fallback
		}
	}

	// Create payload for SignedRequest
	const payload = {
		email,
		ui_host,
		next,
		email_lang,
		pub_key: pubKeyHex
	};

	// Use universal signed POST request (first signed response to extract server_pub_key)
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
	return await httpSignedPOSTRequest<typeof payload, MagicLinkResponse>(
		`${API_BASE}/login/`,
		payload,
		true
	);
}

/**
 * Validate magic link and complete authentication with SignedResponse handling
 * SECURITY: Uses credentials: 'include' to receive HttpOnly refresh token cookie
 */
export async function validateMagicLink(magicToken: string): Promise<LoginResponse> {
	// Use universal signed POST request with magic link payload
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');

	console.log(
		'🍪 [SECURITY] validateMagicLink: Sending request WITH credentials to receive cookie'
	);

	return await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
		`${API_BASE}/login/magiclink/`,
		{ magiclink: magicToken },
		false,
		{ credentials: 'include' } // CRITICAL: Must receive HttpOnly refresh cookie
	);
}

/**
 * Check authentication status
 */
export async function checkAuthStatus(): Promise<boolean> {
	// Check if we have both user info and access token in IndexedDB
	try {
		const { sessionManager } = await import('../session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.user || !authData.access_token) return false;

		return authData.user.isAuthenticated && !!authData.user.user_id;
	} catch {
		return false;
	}
}

/**
 * Logout user and clear server-side session
 */
export async function logout(): Promise<void> {
	// Call backend to clear HttpOnly refresh token cookie using authenticated signed DELETE request
	// Backend validates Ed25519 signature and emits SignedResponse (Zero Knowledge complete chain)
	try {
		const { httpSignedAuthenticatedDELETE } = await import('../httpSignedRequests');
		await httpSignedAuthenticatedDELETE<{ message: string }>(`${API_BASE}/login`);
	} catch {
		// Failed to clear refresh token cookie
		// Continue with logout even if cookie clearing fails
	}
}

/**
 * Try to refresh the access token using the HttpOnly refresh token cookie
 *
 * 🔄 KEY ROTATION LOGIC:
 * - ALWAYS generates new Ed25519 keypair before refresh request
 * - Backend determines rotation based on 2/3 time window:
 *   - Tramo 1/3 (0 to 1/3 duration): Returns access_token only, NO server_pub_key → No rotation
 *   - Tramo 2/3 (1/3 to full duration): Returns both tokens + server_pub_key → Full rotation
 * - Frontend rotates keys ONLY if server_pub_key is present in response
 *
 * Token durations: Configured in .env (SPIN_VARIABLE_*_TOKEN_DURATION_MINUTES)
 * Backend: api/src/utils/jwt/config.rs::get_refresh_token_duration_minutes()
 */
export async function refreshToken(): Promise<boolean> {
	// Import all dependencies at the top to avoid redeclarations
	const { flashMessagesStore } = await import('../stores/flashMessages');
	const { sessionManager } = await import('../session-manager');
	const { generateEd25519KeyPairFallback, publicKeyToHex } = await import('../ed25519');
	const { privateKeyBytesToHex } = await import('../ed25519/ed25519-core');
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
	const { authStore } = await import('../stores/auth');
	const { currentLanguage, t } = await import('../stores/i18n');

	// Get current language for translated flash messages
	let lang = 'en';
	const unsubscribe = currentLanguage.subscribe((l) => (lang = l));
	unsubscribe();

	try {
		// Generate NEW Ed25519 keypair for potential rotation
		const newKeyPair = await generateEd25519KeyPairFallback();
		const newPubKeyHex = publicKeyToHex(newKeyPair.publicKeyBytes);
		const newPrivKeyHex = privateKeyBytesToHex(newKeyPair.privateKeyBytes!);

		// Send refresh request with new_pub_key
		const data = await httpSignedPOSTRequest<{ new_pub_key: string }, LoginResponse>(
			`${API_BASE}/refresh`,
			{ new_pub_key: newPubKeyHex },
			false,
			{ credentials: 'include' }
		);

		// Update auth store with new token
		const user = {
			user_id: data.user_id,
			isAuthenticated: true
		};

		authStore.updateTokens(user, data.access_token);

		// Update session expiration timestamp if provided (new refresh cookie issued)
		if (data.expires_at) {
			try {
				const { storeSessionExpiration } = await import('../session-storage');
				await storeSessionExpiration(data.expires_at);
			} catch (error) {
				console.warn('Failed to store session expiration during refresh:', error);
				// Non-blocking - refresh continues without persistent expiration tracking
			}
		}

		// CONDITIONAL KEY ROTATION
		// NOTE: universalSignedResponseHandler already updated server_pub_key in IndexedDB (if present)
		if (data.server_pub_key) {
			// TRAMO 2/3: Backend sent server_pub_key → Full key rotation
			// Update FULL keypair in hashrand-ed25519 DB (used by getOrCreateKeyPair for signing)
			const { storeKeyPair } = await import('../ed25519/ed25519-database');
			await storeKeyPair(newKeyPair);

			// Also update priv_key in hashrand-session DB for logging/debugging
			await sessionManager.setPrivKey(newPrivKeyHex);
		}

		// Note: Crypto tokens are NOT generated during refresh
		// They are only generated during initial login (magic link validation)
		// If tokens are missing, it means session is corrupted and should restart
		const { ensureCryptoTokensExist } = await import('../utils/auth-recovery');
		const tokensValid = await ensureCryptoTokensExist('Token Refresh');
		if (!tokensValid) {
			// Handler already initiated recovery flow (logout + auth dialog)
			// Return false to indicate refresh failed (user must re-authenticate)
			return false;
		}

		flashMessagesStore.addMessage(t('auth.tokenRefreshSuccess', lang));
		return true;
	} catch (error) {
		console.error('Token refresh failed:', error);
		flashMessagesStore.addMessage(t('auth.tokenRefreshError', lang));

		// Check for dual token expiry in the error
		if (
			error instanceof Error &&
			error.message.includes('Both access and refresh tokens have expired')
		) {
			flashMessagesStore.addMessage(t('auth.sessionExpiredRequireLogin', lang));
			await handleDualTokenExpiry();
		}

		return false;
	}
}

// Function removed - was not being used anywhere in the codebase

/**
 * Handle dual token expiry scenario
 */
async function handleDualTokenExpiry(): Promise<void> {
	// DUAL EXPIRY detected - clearing all auth data and requesting new login

	const { authStore } = await import('../stores/auth');
	const { dialogStore } = await import('../stores/dialog');

	// Complete logout with ALL IndexedDB cleanup
	await authStore.logout();

	// Clear all crypto tokens and auth data (defensive security)
	await authStore.clearPreventiveAuthData();

	// Clear session expiration timestamp (defensive security)
	try {
		const { clearSessionExpiration } = await import('../session-storage');
		await clearSessionExpiration();
	} catch (error) {
		console.warn('Failed to clear session expiration during dual token expiry:', error);
		// Non-blocking - continue with auth dialog
	}

	// Show auth dialog to request new email authentication
	const authConfig = {
		destination: { route: '/' }
	};
	dialogStore.show('auth', authConfig);
}
