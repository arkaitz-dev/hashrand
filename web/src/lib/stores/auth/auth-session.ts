/**
 * Auth Session Module - Session Management and Validation
 *
 * REFACTORED: Moved from proactive (HTTP calls "just in case") to reactive (HTTP only when server requires)
 * Single Responsibility: Handle session validation and local token checking
 * Part of auth.ts refactorization to apply SOLID principles
 */

import { hasCryptoTokens, hasValidRefreshCookie } from './auth-crypto-tokens';
import { clearSensitiveAuthDataWithMessage } from './auth-cleanup';

/**
 * Check if user has local auth tokens (NO HTTP calls, NO validation)
 *
 * CORRECTED APPROACH: Frontend CANNOT validate tokens - only check existence
 * Only backend can determine if tokens are valid/expired/invalid
 * UI shows "authenticated" based on local existence only
 *
 * @returns Promise<boolean> - true if we have local access token and user data (existence only)
 */
export async function hasLocalAuthTokens(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('../../session-manager');
		const authData = await sessionManager.getAuthData();

		// Only check EXISTENCE - never validity (that's server's job)
		return !!(authData.access_token && authData.user?.user_id);
	} catch {
		// Failed to read from IndexedDB - assume no tokens
		return false;
	}
}

/**
 * Check session validity and handle expired refresh cookies
 */
export async function checkSessionValidity(): Promise<void> {
	// Check if crypto tokens are missing when we should have them
	const hasTokens = await hasCryptoTokens();
	if (!hasTokens) {
		// Check if we have any stored session data at all
		try {
			const { sessionManager } = await import('../../session-manager');
			const authData = await sessionManager.getAuthData();
			const hasAnySessionData = authData.access_token || authData.user;

			if (hasAnySessionData) {
				// We have session data but missing crypto tokens - this is corruption
				// Check if refresh cookie is still valid to attempt recovery
				const hasValidCookie = await hasValidRefreshCookie();

				if (!hasValidCookie) {
					// Both crypto tokens missing AND refresh cookie invalid - clear corrupted session
					await clearSensitiveAuthDataWithMessage();
				}
				// If valid cookie exists, crypto tokens will be regenerated on next API call
			}
			// If no session data exists at all, this is a clean browser - do nothing
		} catch {
			// Failed to check session data - clear to be safe
			await clearSensitiveAuthDataWithMessage();
		}
	}
}

/**
 * DEPRECATED: ensureAuthenticated() - REMOVED for reactive architecture
 *
 * OLD PROACTIVE APPROACH (inefficient):
 * - Made HTTP calls "just in case" before every operation
 * - Caused unnecessary /api/refresh calls
 * - Frontend tried to predict if tokens were valid (impossible)
 *
 * NEW REACTIVE APPROACH (efficient):
 * - UI: Use hasLocalAuthTokens() - no HTTP, just existence check
 * - API: Make normal HTTP calls with existing tokens
 * - Server: Returns 401 "expired" → automatic refresh → retry
 * - Server: Returns 401 "invalid" → clear session → login dialog
 *
 * This function is REMOVED because frontend cannot and should not
 * validate tokens proactively. Only server knows if tokens are valid.
 */

// export async function ensureAuthenticated(): Promise<boolean> {
// 	// REMOVED - This pattern was inefficient and conceptually wrong
// 	// See above comment for the new reactive approach
// }
