/**
 * Session Manager - Refactored with SOLID Principles
 *
 * Simplified main manager using specialized modules for different responsibilities.
 * Provides unified session management interface.
 */

import {
	sessionDB,
	type AppSessionData,
	getCryptoTokens,
	setCryptoTokens,
	hasCryptoTokens,
	addPrehashSeed,
	getPrehashSeed,
	getAuthData,
	setAuthData,
	isAuthenticated,
	clearAuthData,
	getUserPreferences,
	setLanguagePreference,
	setThemePreference,
	setUserPreferences,
	setPendingAuthEmail,
	getPendingAuthEmail,
	clearPendingAuthEmail
} from './session/index';

/**
 * Unified Session Manager with simplified interface
 */
class SessionManager {
	/**
	 * Get current session data from IndexedDB
	 */
	async getSession(): Promise<AppSessionData> {
		return await sessionDB.getSession();
	}

	/**
	 * Save session data to IndexedDB
	 */
	async saveSession(session: AppSessionData): Promise<void> {
		return await sessionDB.saveSession(session);
	}

	/**
	 * Update specific part of session
	 */
	async updateSession(updates: Partial<AppSessionData>): Promise<void> {
		return await sessionDB.updateSession(updates);
	}

	/**
	 * Clear auth data only, PRESERVE user preferences (for preventive cleanup)
	 */
	async clearAuthData(): Promise<void> {
		return await clearAuthData();
	}

	/**
	 * Clear ALL session data including preferences (complete logout)
	 */
	async clearSession(): Promise<void> {
		return await sessionDB.clearSession();
	}

	/**
	 * Initialize session manager (no migration needed in development)
	 */
	async init(): Promise<void> {
		// No initialization needed - session management is ready to use
	}

	// ============================================================================
	// CRYPTO TOKENS METHODS
	// ============================================================================

	/**
	 * Get crypto tokens for URL encryption
	 */
	async getCryptoTokens(): Promise<{
		cipher: string | null;
		nonce: string | null;
		hmac: string | null;
	}> {
		return await getCryptoTokens();
	}

	/**
	 * Set crypto tokens for URL encryption
	 */
	async setCryptoTokens(cipher: string, nonce: string, hmac: string): Promise<void> {
		return await setCryptoTokens(cipher, nonce, hmac);
	}

	/**
	 * Check if crypto tokens exist
	 */
	async hasCryptoTokens(): Promise<boolean> {
		return await hasCryptoTokens();
	}

	/**
	 * Add prehash seed to FIFO store
	 */
	async addPrehashSeed(key: string, prehashSeed: string): Promise<void> {
		return await addPrehashSeed(key, prehashSeed);
	}

	/**
	 * Get prehash seed by key
	 */
	async getPrehashSeed(key: string): Promise<string | null> {
		return await getPrehashSeed(key);
	}

	// ============================================================================
	// AUTH DATA METHODS
	// ============================================================================

	/**
	 * Get auth data
	 */
	async getAuthData(): Promise<{
		user: { user_id: string; isAuthenticated: boolean } | null;
		access_token: string | null;
		expires_at: number | null;
	}> {
		return await getAuthData();
	}

	/**
	 * Set auth data
	 */
	async setAuthData(
		user: { user_id: string; isAuthenticated: boolean },
		access_token: string,
		expires_at?: number
	): Promise<void> {
		return await setAuthData(user, access_token, expires_at);
	}

	/**
	 * Check if user is authenticated
	 */
	async isAuthenticated(): Promise<boolean> {
		return await isAuthenticated();
	}

	// ============================================================================
	// USER PREFERENCES METHODS
	// ============================================================================

	/**
	 * Get user preferences from IndexedDB
	 */
	async getUserPreferences(): Promise<{
		language: string | null;
		theme: 'light' | 'dark' | null;
	}> {
		return await getUserPreferences();
	}

	/**
	 * Set language preference
	 */
	async setLanguagePreference(language: string | null): Promise<void> {
		return await setLanguagePreference(language);
	}

	/**
	 * Set theme preference
	 */
	async setThemePreference(theme: 'light' | 'dark' | null): Promise<void> {
		return await setThemePreference(theme);
	}

	/**
	 * Set user preferences (batch update)
	 */
	async setUserPreferences(preferences: {
		language?: string | null;
		theme?: 'light' | 'dark' | null;
	}): Promise<void> {
		return await setUserPreferences(preferences);
	}

	// ============================================================================
	// AUTH FLOW METHODS (temporary data)
	// ============================================================================

	/**
	 * Set pending auth email (during magic link flow)
	 */
	async setPendingAuthEmail(email: string | null): Promise<void> {
		return await setPendingAuthEmail(email);
	}

	/**
	 * Get pending auth email
	 */
	async getPendingAuthEmail(): Promise<string | null> {
		return await getPendingAuthEmail();
	}

	/**
	 * Clear pending auth email (after auth completion)
	 */
	async clearPendingAuthEmail(): Promise<void> {
		return await clearPendingAuthEmail();
	}
}

// Export singleton instance
export const sessionManager = new SessionManager();
