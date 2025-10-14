/**
 * Session Expiry Manager - Centralized session expiration handling
 *
 * Single Responsibility: Handle session expiration checks and automatic logout
 * CRITICAL: All session expiration scenarios use handleAutoLogout for consistency
 */

import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { getSessionExpiration } from './session-storage';
import { clearLocalAuthData } from './stores/auth/auth-actions';
import { logger } from './utils/logger';

/**
 * Check if current session has expired based on stored timestamp
 *
 * @returns Promise<boolean> - true if session is expired or no timestamp stored
 */
export async function isSessionExpired(): Promise<boolean> {
	try {
		const expiresAt = await getSessionExpiration();

		// No stored expiration = consider expired
		if (!expiresAt) return true;

		const now = Math.floor(Date.now() / 1000); // Convert to seconds
		return now >= expiresAt;
	} catch (error) {
		logger.warn('Failed to check session expiration:', error);
		// On error, assume expired for security
		return true;
	}
}

/**
 * CENTRALIZED AUTO-LOGOUT HANDLER
 *
 * Performs automatic logout on session expiration with consistent behavior:
 * 1. Shows localized flash message "Session expired"
 * 2. Cleans up all auth data (IndexedDB + Ed25519 keys + session timestamp)
 * 3. Redirects to home page
 *
 * Used by:
 * - sessionMonitor (periodic 10s checks)
 * - checkSessionOrAutoLogout (proactive checks before actions)
 *
 * @returns Promise<void>
 */
export async function handleAutoLogout(): Promise<void> {
	if (!browser) return;

	try {
		// 1. Get translated message BEFORE cleanup (i18n needs to be accessible)
		const { flashMessagesStore } = await import('./stores/flashMessages');
		const { _ } = await import('./stores/i18n');
		const { get } = await import('svelte/store');

		const translateFn = get(_);
		const message = translateFn('common.sessionExpired');

		// 2. Show flash message to user
		flashMessagesStore.addMessage(message);

		// 3. Clean up all session data (IndexedDB, Ed25519 keys, timestamp, cache)
		await clearLocalAuthData();

		// 4. Redirect to home page
		await goto('/');
	} catch (error) {
		logger.error('Auto-logout failed:', error);
		// Fallback - force redirect even if cleanup failed
		if (browser) {
			window.location.href = '/';
		}
	}
}

/**
 * SIMPLIFIED SESSION CHECK WITH AUTO-LOGOUT
 *
 * Checks if session is valid and performs automatic logout if expired.
 * This is the ONLY function you should use for session validation.
 *
 * Behavior:
 * - If session is VALID: Returns true, no action taken
 * - If session is EXPIRED: Calls handleAutoLogout() and returns false
 *
 * Usage in generation workflows, result pages, etc:
 * ```
 * const sessionValid = await checkSessionOrAutoLogout();
 * if (!sessionValid) return; // Stop execution, user being logged out
 * // Continue with authenticated operation
 * ```
 *
 * @returns Promise<boolean> - true if session is valid, false if expired (auto-logout triggered)
 */
export async function checkSessionOrAutoLogout(): Promise<boolean> {
	const expired = await isSessionExpired();

	if (!expired) {
		return true; // Session is valid
	}

	// Session expired - trigger automatic logout
	await handleAutoLogout();
	return false;
}
