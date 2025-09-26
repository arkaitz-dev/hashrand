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
 */
export async function validateMagicLink(magicToken: string): Promise<LoginResponse> {
	// Use universal signed POST request with magic link payload
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
	return await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
		`${API_BASE}/login/magiclink/`,
		{ magiclink: magicToken },
		false
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
	// Call backend to clear HttpOnly refresh token cookie using signed DELETE request
	try {
		const { httpSignedDELETERequest } = await import('../httpSignedRequests');
		await httpSignedDELETERequest(`${API_BASE}/login`, { credentials: 'include' });
	} catch {
		// Failed to clear refresh token cookie
		// Continue with logout even if cookie clearing fails
	}
}

/**
 * Try to refresh the access token using the HttpOnly refresh token cookie
 */
export async function refreshToken(): Promise<boolean> {
	try {
		// Use universal signed POST request with empty payload and credentials for HttpOnly cookies
		const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
		const data = await httpSignedPOSTRequest<Record<string, never>, LoginResponse>(
			`${API_BASE}/refresh`,
			{},
			false,
			{ credentials: 'include' }
		);

		// Update auth store with new token
		const { authStore } = await import('../stores/auth');

		const user = {
			user_id: data.user_id,
			isAuthenticated: true
		};

		// Update store and IndexedDB
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

		// Note: Crypto tokens are NOT generated during refresh
		// They are only generated during initial login (magic link validation)
		// If tokens are missing, it means session is corrupted and should restart
		const { sessionManager } = await import('../session-manager');
		const tokensExist = await sessionManager.hasCryptoTokens();
		if (!tokensExist) {
			// Crypto tokens missing after refresh - session may be corrupted
		}

		return true;
	} catch (error) {
		// Check for dual token expiry in the error
		if (
			error instanceof Error &&
			error.message.includes('Both access and refresh tokens have expired')
		) {
			// DUAL EXPIRY detected during refresh
			await handleDualTokenExpiry();
		}
		// Token refresh failed
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
