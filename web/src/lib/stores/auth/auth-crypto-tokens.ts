/**
 * Auth Crypto Tokens Module - Cryptographic Token Management
 *
 * Single Responsibility: Handle all cryptographic token generation and validation
 * Part of auth.ts refactorization to apply SOLID principles
 */

import type { LoginResponse } from '../../types/index.js';

/**
 * Generate cryptographic tokens for URL parameter encryption
 * Called after successful login or when crypto tokens are missing
 */
export async function generateCryptoTokens(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Generate three 64-byte cryptographically secure tokens
		const cipherToken = generateSecureToken();
		const nonceToken = generateSecureToken();
		const hmacKey = generateSecureToken();

		// Store in IndexedDB via session manager
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.setCryptoTokens(cipherToken, nonceToken, hmacKey);

		// Log successful generation for debugging
		// Generated crypto tokens for URL parameter encryption
	} catch {
		// Failed to generate crypto tokens
	}
}

/**
 * Check if crypto tokens exist in IndexedDB
 */
export async function hasCryptoTokens(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('../../session-manager');
		return await sessionManager.hasCryptoTokens();
	} catch {
		return false;
	}
}

/**
 * Generate a cryptographically secure 64-byte token
 * @returns Base64 encoded string
 */
function generateSecureToken(): string {
	const array = new Uint8Array(64);
	crypto.getRandomValues(array);
	return btoa(String.fromCharCode(...array));
}

/**
 * Validate if refresh cookie is still valid by making a test request
 * @deprecated Use hasLikelyRefreshCookie() to avoid unnecessary HTTP calls
 */
export async function hasValidRefreshCookie(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		// Use universal signed POST request to check if refresh cookie is valid
		const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');
		await httpSignedPOSTRequest<Record<string, never>, LoginResponse>('/api/refresh', {}, false, {
			credentials: 'include'
		});

		// If request succeeds, cookie is valid
		return true;
	} catch {
		// If request fails (401), cookie is expired/invalid
		return false;
	}
}

/**
 * Check if refresh cookie likely exists without making HTTP requests
 *
 * Uses heuristic approach to determine if we should attempt refresh:
 * - Checks if we have session data in IndexedDB (user_id, access_token history)
 * - Verifies crypto tokens existence (generated after successful login)
 * - More efficient than hasValidRefreshCookie() as it avoids unnecessary HTTP calls
 */
export async function hasLikelyRefreshCookie(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('../../session-manager');
		const authData = await sessionManager.getAuthData();

		// If we have current access token and user, we're likely still authenticated
		if (authData.access_token && authData.user?.user_id) {
			return true;
		}

		// Check if we have evidence of a previous valid session
		// If we have crypto tokens, it means we successfully authenticated before
		const hasTokens = await hasCryptoTokens();
		if (hasTokens) {
			// We have crypto tokens but no access token
			// This suggests tokens expired but refresh cookie might still be valid
			return true;
		}

		// If we have stored user data but no access token or crypto tokens,
		// it's possible we had a session that partially corrupted
		if (authData.user?.user_id) {
			return true;
		}

		// No evidence of previous authentication - likely guest user
		return false;
	} catch {
		// Failed to check session data - assume no valid cookie to be safe
		return false;
	}
}
