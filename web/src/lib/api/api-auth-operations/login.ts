/**
 * Login and magic link operations
 */

import type { LoginResponse, MagicLinkResponse } from '../../types';
import { API_BASE, getCurrentLanguage } from './utilities';

/**
 * Request magic link for email authentication
 */
export async function requestMagicLink(
	email: string,
	ui_host: string,
	next: string = '/'
): Promise<MagicLinkResponse> {
	// Generate or retrieve Ed25519 keypair
	const { getOrCreateKeyPair, publicKeyToHex } = await import('../../ed25519');
	const keyPair = await getOrCreateKeyPair();
	const pubKeyHex = publicKeyToHex(keyPair.publicKeyBytes);

	// Get current language for email template
	const email_lang = await getCurrentLanguage();

	// Create payload for SignedRequest
	const payload = {
		email,
		ui_host,
		next,
		email_lang,
		pub_key: pubKeyHex
	};

	// Use universal signed POST request
	const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');
	return await httpSignedPOSTRequest<typeof payload, MagicLinkResponse>(
		`${API_BASE}/login/`,
		payload,
		true
	);
}

/**
 * Validate magic link and complete authentication
 * SECURITY: Uses credentials: 'include' to receive HttpOnly refresh token cookie
 */
export async function validateMagicLink(magicToken: string): Promise<LoginResponse> {
	const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');

	console.log(
		'🍪 [SECURITY] validateMagicLink: Sending request WITH credentials to receive cookie'
	);

	return await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
		`${API_BASE}/login/magiclink/`,
		{ magiclink: magicToken },
		false,
		{ credentials: 'include' }
	);
}

/**
 * Logout user and clear server-side session
 */
export async function logout(): Promise<void> {
	try {
		const { httpSignedAuthenticatedDELETE } = await import('../../httpSignedRequests');
		await httpSignedAuthenticatedDELETE<{ message: string }>(`${API_BASE}/login`);
	} catch {
		// Continue with logout even if cookie clearing fails
	}
}

/**
 * Check authentication status
 */
export async function checkAuthStatus(): Promise<boolean> {
	try {
		const { sessionManager } = await import('../../session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.user || !authData.access_token) return false;

		return authData.user.isAuthenticated && !!authData.user.user_id;
	} catch {
		return false;
	}
}
