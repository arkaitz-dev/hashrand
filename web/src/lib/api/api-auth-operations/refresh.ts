/**
 * Token refresh and key rotation operations
 */

import type { LoginResponse } from '../../types';
import { API_BASE, getCurrentLanguage } from './utilities';
import { logger } from '../../utils/logger';

/**
 * Try to refresh the access token using the HttpOnly refresh token cookie
 *
 * 🔄 KEY ROTATION LOGIC:
 * - ALWAYS generates new Ed25519 AND X25519 keypairs before refresh request
 * - Backend determines rotation based on 2/3 time window:
 *   - Period 1/3 (0 to 1/3 duration): Returns access_token only, NO server_pub_key → No rotation
 *   - Period 2/3 (1/3 to full duration): Returns both tokens + server_pub_key → Full rotation
 * - Frontend rotates BOTH keys ONLY if server_pub_key is present in response
 */
export async function refreshToken(): Promise<boolean> {
	// Import dependencies
	const { flashMessagesStore } = await import('../../stores/flashMessages');
	const { sessionManager } = await import('../../session-manager');
	const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');
	const { authStore } = await import('../../stores/auth');
	const { t } = await import('../../stores/i18n');
	const { generateKeypairs } = await import('../../crypto/keypair-generation');
	const { storeKeypairs } = await import('../../crypto/keypair-storage');

	// Get current language for translated flash messages
	const lang = await getCurrentLanguage();

	try {
		// Generate NEW Ed25519 AND X25519 keypairs for potential rotation
		const newKeypairs = await generateKeypairs();
		const newEd25519PubKeyHex = newKeypairs.ed25519.publicKeyHex;
		const newX25519PubKeyHex = newKeypairs.x25519.publicKeyHex;

		// Send refresh request with BOTH new pub_keys
		const data = await httpSignedPOSTRequest<
			{ new_ed25519_pub_key: string; new_x25519_pub_key: string },
			LoginResponse
		>(
			`${API_BASE}/refresh`,
			{
				new_ed25519_pub_key: newEd25519PubKeyHex,
				new_x25519_pub_key: newX25519PubKeyHex
			},
			false,
			{ credentials: 'include' }
		);

		// Get current user email from session (preserve it during refresh - Zero Knowledge)
		const currentAuthData = await sessionManager.getAuthData();
		const userEmail = currentAuthData.user?.email ?? '';

		// Update auth store with new token
		const user = {
			user_id: data.user_id,
			email: userEmail,
			isAuthenticated: true
		};

		authStore.updateTokens(user, data.access_token);

		// Update session expiration timestamp if provided
		if (data.expires_at) {
			try {
				const { storeSessionExpiration } = await import('../../session-storage');
				await storeSessionExpiration(data.expires_at);
			} catch (error) {
				logger.warn('Failed to store session expiration during refresh:', error);
			}
		}

		// CONDITIONAL KEY ROTATION
		if (data.server_pub_key) {
			// PERIOD 2/3: Backend sent server_pub_key → Full key rotation for BOTH Ed25519 and X25519
			await storeKeypairs(newKeypairs);
		}

		// Ensure crypto tokens exist
		const { ensureCryptoTokensExist } = await import('../../utils/auth-recovery');
		const tokensValid = await ensureCryptoTokensExist('Token Refresh');
		if (!tokensValid) {
			return false;
		}

		return true;
	} catch (error) {
		logger.error('Token refresh failed:', error);
		flashMessagesStore.addMessage(t('auth.tokenRefreshError', lang));

		// Check for dual token expiry
		if (
			error instanceof Error &&
			error.message.includes('Both access and refresh tokens have expired')
		) {
			flashMessagesStore.addMessage(t('auth.sessionExpiredRequireLogin', lang));
			await handleDualTokenExpiry();
		}

		return false;
	}
}

/**
 * Handle dual token expiry scenario
 *
 * UNIFIED APPROACH: Uses authStore.logout() which calls clearLocalAuthData()
 * No need for additional cleanup - logout handles everything
 */
async function handleDualTokenExpiry(): Promise<void> {
	const { authStore } = await import('../../stores/auth');
	const { dialogStore } = await import('../../stores/dialog');

	// Logout handles ALL cleanup:
	// - Ed25519 keypairs
	// - IndexedDB session data
	// - Session expiration timestamp
	await authStore.logout();

	// Show auth dialog
	const authConfig = {
		destination: { route: '/' }
	};
	dialogStore.show('auth', authConfig);
}
