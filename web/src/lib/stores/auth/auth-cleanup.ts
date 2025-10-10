/**
 * Auth Cleanup Module - Data Cleanup Operations
 *
 * Single Responsibility: Handle all authentication data cleanup operations
 * Part of auth.ts refactorization to apply SOLID principles
 */

import { clearLocalAuthData } from './auth-actions';

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

		// Clear confirm-read cache database
		try {
			indexedDB.deleteDatabase('hashrand-cache');
		} catch {
			// Failed to clear confirm-read cache
		}

		// Preventive auth data cleanup completed
	} catch {
		// Failed to clear preventive auth data
	}
}

/**
 * Clear sensitive authentication data when session corruption is detected
 *
 * UNIFIED APPROACH: Uses clearLocalAuthData() for complete cleanup
 * Silent version (no flash message) - for programmatic cleanup
 * More aggressive than preventive cleanup - used when tokens are inconsistent
 */
export async function clearSensitiveAuthData(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Use unified cleanup function (complete logout cleanup)
		await clearLocalAuthData();

		// Clear confirm-read cache database
		try {
			indexedDB.deleteDatabase('hashrand-cache');
		} catch {
			// Failed to clear confirm-read cache
		}
	} catch {
		// Failed to clear sensitive auth data
	}
}

/**
 * Clear sensitive authentication data with localized flash message
 *
 * UNIFIED APPROACH: Uses clearLocalAuthData() for complete cleanup
 * Used when session corruption is detected and we want to inform the user
 */
export async function clearSensitiveAuthDataWithMessage(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Use unified cleanup function (complete logout cleanup)
		await clearLocalAuthData();

		// Show localized flash message that session data was cleared
		try {
			const { flashMessagesStore } = await import('../flashMessages');
			const { currentLanguage } = await import('../i18n');
			const { t } = await import('../i18n');

			// Get current language and show translated message
			let currentLang = 'en';
			const unsubscribe = currentLanguage.subscribe((lang) => (currentLang = lang));
			unsubscribe();

			const message = t('auth.sessionDataCleared', currentLang);
			flashMessagesStore.addMessage(message);
		} catch {
			// Failed to show session flash message - fallback to English
			try {
				const { flashMessagesStore } = await import('../flashMessages');
				flashMessagesStore.addMessage('⚠️ Session data cleared for security');
			} catch {
				// Failed to show fallback message
			}
		}
	} catch {
		// Failed to clear sensitive auth data
	}
}
