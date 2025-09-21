/**
 * Session Auth Module - Authentication Data Management
 *
 * Single Responsibility: Handle authentication data in session
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

import { sessionDB } from './session-db';

/**
 * Get auth data
 */
export async function getAuthData(): Promise<{
	user: { user_id: string; isAuthenticated: boolean } | null;
	access_token: string | null;
	expires_at: number | null;
}> {
	const session = await sessionDB.getSession();
	return {
		user: session.auth_user,
		access_token: session.access_token,
		expires_at: session.token_expires_at
	};
}

/**
 * Set auth data
 */
export async function setAuthData(
	user: { user_id: string; isAuthenticated: boolean },
	access_token: string,
	expires_at?: number
): Promise<void> {
	await sessionDB.updateSession({
		auth_user: user,
		access_token,
		token_expires_at: expires_at || null
	});
}

/**
 * Check if user is authenticated
 */
export async function isAuthenticated(): Promise<boolean> {
	const authData = await getAuthData();
	return !!(authData.user?.isAuthenticated && authData.access_token);
}

/**
 * Clear auth data only, PRESERVE user preferences (for preventive cleanup)
 */
export async function clearAuthData(): Promise<void> {
	const session = await sessionDB.getSession();

	// Clear ONLY auth-related data, preserve preferences
	session.cipher_token = null;
	session.nonce_token = null;
	session.hmac_key = null;
	session.prehashSeeds = [];
	session.auth_user = null;
	session.access_token = null;
	session.token_expires_at = null;
	session.authFlow.pending_email = null;

	// Keep userPreferences intact for UX
	await sessionDB.saveSession(session);
	// Auth data cleared, preferences preserved
}
