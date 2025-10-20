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
	// Generate or retrieve independent Ed25519 and X25519 keypairs
	const { generateKeypairs } = await import('../../crypto/keypair-generation');
	const { storeKeypairs, keypairsExist, getPublicKeyHexStrings } = await import(
		'../../crypto/keypair-storage'
	);

	let ed25519PubKeyHex: string;
	let x25519PubKeyHex: string;

	// Check if keypairs already exist (regeneration case)
	if (await keypairsExist()) {
		const existingKeys = await getPublicKeyHexStrings();
		if (existingKeys) {
			ed25519PubKeyHex = existingKeys.ed25519;
			x25519PubKeyHex = existingKeys.x25519;
		} else {
			// Generate new keypairs if retrieval failed
			const keypairs = await generateKeypairs();
			await storeKeypairs(keypairs);
			ed25519PubKeyHex = keypairs.ed25519.publicKeyHex;
			x25519PubKeyHex = keypairs.x25519.publicKeyHex;
		}
	} else {
		// Generate new keypairs
		const keypairs = await generateKeypairs();
		await storeKeypairs(keypairs);
		ed25519PubKeyHex = keypairs.ed25519.publicKeyHex;
		x25519PubKeyHex = keypairs.x25519.publicKeyHex;
	}

	// Get current language for email template
	const email_lang = await getCurrentLanguage();

	// Create payload for SignedRequest (now with BOTH pub_keys)
	const payload = {
		email,
		ui_host,
		next,
		email_lang,
		ed25519_pub_key: ed25519PubKeyHex,
		x25519_pub_key: x25519PubKeyHex
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
	const { logger } = await import('../../utils/logger');
	logger.debug('[validateMagicLink] Starting magic link validation:', {
		tokenLength: magicToken.length,
		tokenPrefix: magicToken.substring(0, 20) + '...'
	});

	const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');

	try {
		logger.debug('[validateMagicLink] Sending POST request to /api/login/magiclink/');
		const response = await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
			`${API_BASE}/login/magiclink/`,
			{ magiclink: magicToken },
			false,
			{ credentials: 'include' }
		);
		logger.debug('[validateMagicLink] Received successful response from backend');
		return response;
	} catch (error) {
		logger.error('[validateMagicLink] Request failed:', error);
		throw error;
	}
}

/**
 * Logout user - Client-side only (stateless architecture)
 *
 * PHILOSOPHY:
 * - Logout is a CLIENT action, not requiring server coordination
 * - Server is stateless (no session state to clear)
 * - Refresh token cookie expires automatically (configured duration)
 * - Cookie alone is useless without IndexedDB keypair (Ed25519)
 * - Simplicity: fewer failure points, better UX
 *
 * SECURITY:
 * - Local cleanup always succeeds (IndexedDB + Ed25519 keys)
 * - No network dependency = reliable logout even offline
 * - Cookie expiration handled by browser (Max-Age)
 */
export async function logout(): Promise<void> {
	// No server call needed - logout is purely local
	// All cleanup handled by auth-actions.ts::logout()
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
