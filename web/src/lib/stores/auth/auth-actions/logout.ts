/**
 * Logout Module - Session Termination and Cleanup
 *
 * Single Responsibility: Handle logout operations and local data cleanup.
 * Part of auth-actions refactorization to apply SOLID principles.
 *
 * PURPOSE:
 * - Manual logout (user-initiated)
 * - Automatic logout (session expiration)
 * - Unified cleanup operations
 *
 * OPERATIONS:
 * - Clear Ed25519 keypairs (Sistema A)
 * - Clear IndexedDB session data
 * - Clear session expiration timestamp
 * - Clear confirm-read cache database
 *
 * @see login.ts for authentication operations
 */

import { logger } from '../../../utils/logger';

/**
 * Clear all local authentication data (shared between manual and automatic logout)
 *
 * UNIFIED CLEANUP: Used by both:
 * - Manual logout (user clicks logout button)
 * - Automatic logout (session expiration monitor)
 *
 * OPERATIONS:
 * 1. Clear Ed25519 keypairs (security - Sistema A)
 * 2. Clear ALL IndexedDB session data
 * 3. Clear session expiration timestamp
 * 4. Clear confirm-read cache database
 *
 * @returns Promise<void>
 */
export async function clearLocalAuthData(): Promise<void> {
	// Clear Ed25519 keypairs for security (Sistema A - temporary session keys)
	try {
		const { clearAllKeyPairs } = await import('../../../ed25519');
		await clearAllKeyPairs();
	} catch {
		// Failed to clear Ed25519 keypairs
	}

	// Clear ALL IndexedDB session data
	try {
		const { sessionManager } = await import('../../../session-manager');
		await sessionManager.clearSession();
	} catch {
		// Failed to clear IndexedDB session
	}

	// Clear session expiration timestamp
	try {
		const { clearSessionExpiration } = await import('../../../session-storage');
		await clearSessionExpiration();
	} catch (error) {
		logger.warn('Failed to clear session expiration during logout:', error);
		// Non-blocking - logout continues
	}

	// Clear confirm-read cache database (await completion for clean logout)
	try {
		await new Promise<void>((resolve, reject) => {
			const deleteRequest = indexedDB.deleteDatabase('hashrand-confirm-read-cache');

			deleteRequest.onsuccess = () => {
				logger.info('[clearLocalAuthData] Confirm-read cache deleted successfully');
				resolve();
			};

			deleteRequest.onerror = () => {
				logger.warn('[clearLocalAuthData] Failed to delete confirm-read cache', {
					error: deleteRequest.error
				});
				reject(deleteRequest.error);
			};

			deleteRequest.onblocked = () => {
				logger.warn('[clearLocalAuthData] Confirm-read cache deletion blocked');
				// Resolve anyway to not block logout
				resolve();
			};
		});
	} catch (error) {
		logger.warn('Failed to clear confirm-read cache during logout:', error);
		// Non-blocking - logout continues
	}
}

/**
 * Logout user - Client-side only (stateless architecture)
 *
 * This is the MANUAL logout function (user-initiated).
 * For automatic logout (session expiration), use clearLocalAuthData() directly.
 *
 * ARCHITECTURE:
 * - No server call needed (server is stateless)
 * - Calls api.logout() for any server-side cleanup if needed
 * - Performs unified local cleanup via clearLocalAuthData()
 *
 * @returns Promise<void>
 */
export async function logout(): Promise<void> {
	// Call API logout (stateless architecture - minimal server-side operations)
	const { api } = await import('../../../api');
	await api.logout();

	// Perform unified local cleanup
	await clearLocalAuthData();
}
