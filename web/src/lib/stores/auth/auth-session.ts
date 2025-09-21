/**
 * Auth Session Module - Session Management and Validation
 *
 * Single Responsibility: Handle session validation and automatic refresh logic
 * Part of auth.ts refactorization to apply SOLID principles
 */

import { hasCryptoTokens, hasValidRefreshCookie } from './auth-crypto-tokens';
import { clearSensitiveAuthData } from './auth-cleanup';

/**
 * Check session validity and handle expired refresh cookies
 */
export async function checkSessionValidity(): Promise<void> {
	// Check if crypto tokens are missing when we should have them
	const hasTokens = await hasCryptoTokens();
	if (!hasTokens) {
		// Check if refresh cookie is still valid
		const hasValidCookie = await hasValidRefreshCookie();

		if (!hasValidCookie) {
			// Refresh cookie expired/invalid, clear sensitive data
			await clearSensitiveAuthData();
		}
		// If valid cookie exists, crypto tokens will be generated on next API call via refresh
	}
}

/**
 * Ensure authentication by trying refresh only if no access token exists
 * Returns true if authenticated (or after successful refresh), false if needs login
 */
export async function ensureAuthenticated(): Promise<boolean> {
	// Check if we already have access token in IndexedDB
	try {
		const { sessionManager } = await import('../../session-manager');
		const authData = await sessionManager.getAuthData();

		if (authData.access_token && authData.user) {
			// We have tokens - backend will validate expiration
			if (authData.user.isAuthenticated && authData.user.user_id) {
				return true; // Valid session exists - NO refresh needed
			}
		}
	} catch {
		// Failed to load auth data from IndexedDB
		// Clear invalid data and continue to refresh
		await clearSensitiveAuthData();
	}

	// No valid access token found, attempting automatic refresh

	try {
		// Import api to avoid circular dependencies
		const { api } = await import('../../api');
		const refreshSuccess = await api.refreshToken();

		if (refreshSuccess) {
			// Automatic refresh successful
			return true;
		} else {
			// Automatic refresh failed - login required
			return false;
		}
	} catch {
		// Refresh attempt failed
		return false;
	}
}
