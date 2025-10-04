/**
 * Auth Actions Module - Authentication Actions
 *
 * Single Responsibility: Handle magic link authentication actions
 * Part of auth.ts refactorization to apply SOLID principles
 */

import type { AuthUser, LoginResponse, MagicLinkResponse } from '../../types';
import { saveAuthToStorage } from './auth-storage';
import { generateCryptoTokens, hasCryptoTokens } from './auth-crypto-tokens';

/**
 * Request magic link for email authentication
 *
 * @param email - User email address
 * @param next - Optional Base58-encoded parameters to include in magic link URL
 * @returns Promise<MagicLinkResponse>
 */
export async function requestMagicLink(
	email: string,
	next: string = '/'
): Promise<MagicLinkResponse> {
	// Capture current UI host (domain only) for magic link generation and cookie Domain attribute
	const { extractDomain } = await import('../../utils/domain-extractor');
	const ui_host = extractDomain();

	if (!ui_host) {
		throw new Error('UI host is required for magic link generation');
	}

	// Save email to IndexedDB for Zero Knowledge UX (will be retrieved during validateMagicLink)
	const { sessionManager } = await import('../../session-manager');
	await sessionManager.setPendingAuthEmail(email);

	const { api } = await import('../../api');
	return await api.requestMagicLink(email, ui_host, next);
}

/**
 * Validate magic link and complete authentication
 *
 * @param magicToken - Magic link token from URL parameter (Ed25519 verified by backend)
 * @returns Promise<{ user: AuthUser; accessToken: string; loginResponse: LoginResponse }>
 */
export async function validateMagicLink(magicToken: string): Promise<{
	user: AuthUser;
	accessToken: string;
	loginResponse: LoginResponse;
}> {
	const { api } = await import('../../api');
	const loginResponse = await api.validateMagicLink(magicToken);

	// Get pending email before clearing it (needed for Zero Knowledge UX)
	const { sessionManager } = await import('../../session-manager');
	const userEmail = (await sessionManager.getPendingAuthEmail()) ?? '';

	const user: AuthUser = {
		user_id: loginResponse.user_id, // Base58 user_id
		email: userEmail, // User email for UX display (Zero Knowledge compliant)
		isAuthenticated: true
	};

	// Save to IndexedDB
	await saveAuthToStorage(user, loginResponse.access_token);

	// Store session expiration timestamp if provided (new refresh cookie)
	if (loginResponse.expires_at) {
		try {
			const { storeSessionExpiration } = await import('../../session-storage');
			await storeSessionExpiration(loginResponse.expires_at);
		} catch (error) {
			console.warn('Failed to store session expiration:', error);
			// Non-blocking - auth continues without persistent expiration tracking
		}
	}

	// Show flash message for successful magic link validation
	try {
		const { flashMessagesStore } = await import('../flashMessages');
		const { currentLanguage, t } = await import('../i18n');

		// Get current language
		let lang = 'en';
		const unsubscribe = currentLanguage.subscribe((l) => (lang = l));
		unsubscribe();

		// Show translated message
		const message = t('auth.magicLinkValidatedSuccess', lang);
		flashMessagesStore.addMessage(message);
	} catch {
		// Failed to show magic link success flash message
	}

	// Generate crypto tokens ONLY if they don't exist yet
	const tokensExist = await hasCryptoTokens();
	if (!tokensExist) {
		// Magic link: No crypto tokens found, generating
		await generateCryptoTokens();
	} else {
		// Magic link: Crypto tokens already exist
	}

	// Clear pending auth email - no longer needed after successful authentication
	try {
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.clearPendingAuthEmail();
	} catch {
		// Failed to clear pending auth email from IndexedDB
	}

	return { user, accessToken: loginResponse.access_token, loginResponse };
}

/**
 * Clear all local authentication data (shared between manual and automatic logout)
 *
 * UNIFIED CLEANUP: Used by both:
 * - Manual logout (user clicks logout button)
 * - Automatic logout (session expiration monitor)
 *
 * OPERATIONS:
 * 1. Clear Ed25519 keypairs (security)
 * 2. Clear ALL IndexedDB session data
 * 3. Clear session expiration timestamp
 *
 * @returns Promise<void>
 */
export async function clearLocalAuthData(): Promise<void> {
	// Clear Ed25519 keypairs for security
	try {
		const { clearAllKeyPairs } = await import('../../ed25519');
		await clearAllKeyPairs();
	} catch {
		// Failed to clear Ed25519 keypairs
	}

	// Clear ALL IndexedDB session data
	try {
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.clearSession();
	} catch {
		// Failed to clear IndexedDB session
	}

	// Clear session expiration timestamp
	try {
		const { clearSessionExpiration } = await import('../../session-storage');
		await clearSessionExpiration();
	} catch (error) {
		console.warn('Failed to clear session expiration during logout:', error);
		// Non-blocking - logout continues
	}
}

/**
 * Logout user - Client-side only (stateless architecture)
 *
 * This is the MANUAL logout function (user-initiated)
 * For automatic logout (session expiration), use clearLocalAuthData() directly
 */
export async function logout(): Promise<void> {
	// No server call needed - server is stateless
	// (See api/api-auth-operations/login.ts::logout() for philosophy)
	const { api } = await import('../../api');
	await api.logout();

	// Perform unified local cleanup
	await clearLocalAuthData();
}
