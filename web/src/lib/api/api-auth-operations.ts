/**
 * API Auth Operations Module - Authentication Endpoints
 *
 * Single Responsibility: Handle authentication-related API operations
 * Part of api.ts refactorization to apply SRP and organize by domain
 */

import type { LoginResponse, MagicLinkResponse, AuthError } from '../types';
import { ApiError } from './api-helpers';

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

	// Create signed request with universal signature
	const { createSignedRequest } = await import('../signedRequest');
	const signedRequest = await createSignedRequest(payload);

	// Created signed magic link request

	const response = await fetch(`${API_BASE}/login/`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(signedRequest)
	});

	if (!response.ok) {
		const errorData = (await response.json()) as AuthError;
		throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
	}

	return response.json() as Promise<MagicLinkResponse>;
}

/**
 * Validate magic link and complete authentication
 */
export async function validateMagicLink(magicToken: string): Promise<LoginResponse> {
	// Initiating validateMagicLink
	// Create unified SignedRequest structure with magic link payload
	const { createSignedRequest } = await import('../signedRequest');
	const signedRequest = await createSignedRequest({ magiclink: magicToken });
	// SignedRequest created with Ed25519 signature

	// Sending request to backend
	const response = await fetch(`${API_BASE}/login/magiclink/`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(signedRequest)
	});

	// Backend response received

	if (!response.ok) {
		const errorData = (await response.json()) as AuthError;
		// Backend error occurred
		throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
	}

	// The refresh token will be set as HttpOnly cookie by the server
	const result = (await response.json()) as LoginResponse;
	// LoginResponse successful
	return result;
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
	// Call backend to clear HttpOnly refresh token cookie
	try {
		await fetch(`${API_BASE}/login`, {
			method: 'DELETE',
			credentials: 'include' // Include HttpOnly cookies for deletion
		});
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
		// Frontend: Attempting token refresh
		const response = await fetch(`${API_BASE}/refresh`, {
			method: 'POST',
			credentials: 'include' // Include HttpOnly cookies
		});

		// Frontend: Refresh response status received

		// Check for dual token expiry in refresh response
		if (response.status === 401) {
			const isDualExpiry = await isDualTokenExpiry(response);
			if (isDualExpiry) {
				// DUAL EXPIRY detected during refresh - both tokens expired
				await handleDualTokenExpiry();
				return false;
			}
		}

		if (response.ok) {
			const data = await response.json();
			// Frontend: Refresh successful

			// Update auth store with new token
			const { authStore } = await import('../stores/auth');

			const user = {
				user_id: data.user_id,
				isAuthenticated: true
			};

			// Update store and IndexedDB
			authStore.updateTokens(user, data.access_token);

			// Note: Crypto tokens are NOT generated during refresh
			// They are only generated during initial login (magic link validation)
			// If tokens are missing, it means session is corrupted and should restart
			const { sessionManager } = await import('../session-manager');
			const tokensExist = await sessionManager.hasCryptoTokens();
			if (!tokensExist) {
				// Crypto tokens missing after refresh - session may be corrupted
			}

			return true;
		}
		return false;
	} catch {
		// Token refresh failed
		return false;
	}
}

/**
 * Check if a 401 response indicates dual token expiry (both access and refresh tokens expired)
 */
async function isDualTokenExpiry(response: Response): Promise<boolean> {
	if (response.status !== 401) return false;

	try {
		// Clone response to read body without consuming it
		const responseClone = response.clone();
		const errorData = await responseClone.json();

		// Check for dual expiry message from backend
		return !!(
			errorData.error && errorData.error.includes('Both access and refresh tokens have expired')
		);
	} catch {
		// If parsing fails, it's not a dual expiry response
		return false;
	}
}

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

	// Show auth dialog to request new email authentication
	const authConfig = {
		destination: { route: '/' }
	};
	dialogStore.show('auth', authConfig);
}
