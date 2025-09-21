/**
 * Auth Cleanup Module - Data Cleanup Operations
 *
 * Single Responsibility: Handle all authentication data cleanup operations
 * Part of auth.ts refactorization to apply SOLID principles
 */

/**
 * Clear all authentication data preventively before showing login dialog
 * Ensures clean state regardless of how previous session ended
 */
export async function clearPreventiveAuthData(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Clear IndexedDB auth data while preserving user preferences
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.clearAuthData();

		// Clear Ed25519 keypairs for security
		try {
			const { clearAllKeyPairs } = await import('../../ed25519');
			await clearAllKeyPairs();
		} catch {
			// Failed to clear Ed25519 keypairs
		}

		// Preventive auth data cleanup completed
	} catch {
		// Failed to clear preventive auth data
	}
}

/**
 * Clear sensitive authentication data when session corruption is detected
 * More aggressive than preventive cleanup - used when tokens are inconsistent
 */
export async function clearSensitiveAuthData(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Clear all session data including preferences (defensive security)
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.clearSession();

		// Clear Ed25519 keypairs
		try {
			const { clearAllKeyPairs } = await import('../../ed25519');
			await clearAllKeyPairs();
		} catch {
			// Failed to clear Ed25519 keypairs during sensitive cleanup
		}

		// Show flash message that session was corrupted and cleared
		try {
			const { flashMessagesStore } = await import('../flashMessages');
			flashMessagesStore.addMessage('⚠️ Session corrupted, cleared for security');
		} catch {
			// Failed to show session corruption flash message
		}

		// Sensitive auth data cleanup completed
	} catch {
		// Failed to clear sensitive auth data
	}
}
