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
	// Capture current UI host for magic link generation
	const ui_host = typeof window !== 'undefined' ? window.location.origin : '';

	if (!ui_host) {
		throw new Error('UI host is required for magic link generation');
	}

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

	const user: AuthUser = {
		user_id: loginResponse.user_id, // Base58 user_id
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
		flashMessagesStore.addMessage('✅ Magic link validado exitosamente!');
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
 * Logout user and clear all authentication data
 */
export async function logout(): Promise<void> {
	// Call API logout to clear server-side session and refresh token cookie
	const { api } = await import('../../api');
	await api.logout();

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
