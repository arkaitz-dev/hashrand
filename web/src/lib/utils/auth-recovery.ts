/**
 * Auth Recovery Utility - Universal Handler for Session Corruption
 *
 * Single Responsibility: Handle crypto tokens missing and session corruption scenarios
 * SOLID: Separated from auth logic, reusable across different contexts
 * DRY: Universal solution for Cases 1 & 3 (crypto tokens missing)
 */

/**
 * Check if crypto tokens are missing and handle recovery flow
 *
 * This function implements a UNIVERSAL recovery strategy:
 * 1. If no session exists → Initiate login flow (expected behavior)
 * 2. If session exists BUT crypto tokens missing → Session corrupted → Force logout + login
 * 3. Always show translated flash message to user (no silent failures)
 *
 * @param context - Description of where the check occurred (for logging/messages)
 * @returns Promise<boolean> - true if crypto tokens exist and valid, false otherwise
 */
export async function ensureCryptoTokensExist(context: string): Promise<boolean> {
	const { sessionManager } = await import('../session-manager');

	// Check if crypto tokens exist
	const tokensExist = await sessionManager.hasCryptoTokens();

	if (tokensExist) {
		// Tokens exist - all good
		return true;
	}

	// Tokens missing - determine if this is expected (no session) or corruption (has session)
	const hasSession = await sessionManager.isAuthenticated();

	if (!hasSession) {
		// Expected: No session, no tokens - user needs to login
		await initiateAuthenticationFlow('auth.requiresAuthentication');
		return false;
	} else {
		// Unexpected: Has session BUT missing crypto tokens - session corrupted
		console.error(`🚨 [AUTH-RECOVERY] ${context}: Session corrupted - forcing logout`);
		await handleSessionCorruption('auth.sessionCorrupted');
		return false;
	}
}

/**
 * Handle session corruption by forcing complete logout and initiating re-authentication
 *
 * UNIFIED APPROACH: Uses authStore.logout() which calls clearLocalAuthData()
 * No need for additional cleanup - logout handles everything
 *
 * @param translationKey - i18n key for flash message
 */
async function handleSessionCorruption(translationKey: string): Promise<void> {
	// Show translated flash message
	await showTranslatedFlashMessage(translationKey);

	// Logout handles ALL cleanup:
	// - Ed25519 keypairs
	// - IndexedDB session data
	// - Session expiration timestamp
	const { authStore } = await import('../stores/auth');
	await authStore.logout();

	// Initiate authentication flow
	await initiateAuthenticationFlow('auth.sessionCorrupted');
}

/**
 * Initiate authentication flow by showing auth dialog
 *
 * @param translationKey - i18n key for flash message
 */
async function initiateAuthenticationFlow(translationKey: string): Promise<void> {
	// Show translated flash message
	await showTranslatedFlashMessage(translationKey);

	// Show auth dialog to request new email authentication
	const { dialogStore } = await import('../stores/dialog');
	const authConfig = {
		destination: { route: '/' }
	};
	dialogStore.show('auth', authConfig);
}

/**
 * Show translated flash message to user
 *
 * @param translationKey - i18n key for the message
 */
async function showTranslatedFlashMessage(translationKey: string): Promise<void> {
	try {
		const { flashMessagesStore } = await import('../stores/flashMessages');
		const { currentLanguage, t } = await import('../stores/i18n');

		// Get current language
		let lang = 'en';
		const unsubscribe = currentLanguage.subscribe((l) => (lang = l));
		unsubscribe();

		// Get translated message
		const message = t(translationKey, lang);
		flashMessagesStore.addMessage(message);
	} catch {
		// Fallback to English if translation fails
		const { flashMessagesStore } = await import('../stores/flashMessages');
		flashMessagesStore.addMessage(translationKey); // Show key as fallback
	}
}
