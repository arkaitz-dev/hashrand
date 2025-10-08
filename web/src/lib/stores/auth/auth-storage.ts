/**
 * Auth Storage Module - IndexedDB Persistence
 *
 * Single Responsibility: Handle all authentication data persistence operations
 * Part of auth.ts refactorization to apply SOLID principles
 */

import type { AuthUser } from '../../types';
import { logger } from '../../utils/logger';

/**
 * Load authentication state from IndexedDB on initialization
 */
export async function loadAuthFromStorage(): Promise<{
	user: AuthUser | null;
	accessToken: string | null;
	cipherToken: string | null;
	nonceToken: string | null;
	hmacKey: string | null;
}> {
	if (typeof window === 'undefined') {
		return {
			user: null,
			accessToken: null,
			cipherToken: null,
			nonceToken: null,
			hmacKey: null
		};
	}

	try {
		const { sessionManager } = await import('../../session-manager');
		const authData = await sessionManager.getAuthData();
		const cryptoTokens = await sessionManager.getCryptoTokens();

		return {
			user: authData.user,
			accessToken: authData.access_token,
			cipherToken: cryptoTokens.cipher,
			nonceToken: cryptoTokens.nonce,
			hmacKey: cryptoTokens.hmac
		};
	} catch {
		return {
			user: null,
			accessToken: null,
			cipherToken: null,
			nonceToken: null,
			hmacKey: null
		};
	}
}

/**
 * Save authentication data to IndexedDB
 */
export async function saveAuthToStorage(user: AuthUser, accessToken: string): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.setAuthData(user, accessToken);
	} catch (error) {
		logger.error('Failed to save auth to IndexedDB:', error);

		// Show translated warning to user (silent failure is bad UX)
		try {
			const { flashMessagesStore } = await import('../flashMessages');
			const { currentLanguage, t } = await import('../i18n');

			// Get current language
			let lang = 'en';
			const unsubscribe = currentLanguage.subscribe((l) => (lang = l));
			unsubscribe();

			// Show translated warning
			const message = t('auth.storageSaveFailed', lang);
			flashMessagesStore.addMessage(message);
		} catch {
			// Fallback to English if translation fails
			const { flashMessagesStore } = await import('../flashMessages');
			flashMessagesStore.addMessage(
				'⚠️ Session may not persist across page reloads (storage issue)'
			);
		}
	}
}

/**
 * Clear all authentication data from storage
 */
export async function clearAuthFromStorage(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.clearSession();
	} catch {
		// Failed to clear auth from IndexedDB
	}
}
