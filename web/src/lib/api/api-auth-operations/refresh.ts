/**
 * Token refresh and key rotation operations
 */

import type { LoginResponse } from '../../types';
import { API_BASE, getCurrentLanguage } from './utilities';

/**
 * Try to refresh the access token using the HttpOnly refresh token cookie
 *
 * 🔄 KEY ROTATION LOGIC:
 * - ALWAYS generates new Ed25519 keypair before refresh request
 * - Backend determines rotation based on 2/3 time window:
 *   - Tramo 1/3 (0 to 1/3 duration): Returns access_token only, NO server_pub_key → No rotation
 *   - Tramo 2/3 (1/3 to full duration): Returns both tokens + server_pub_key → Full rotation
 * - Frontend rotates keys ONLY if server_pub_key is present in response
 */
export async function refreshToken(): Promise<boolean> {
	// Import dependencies
	const { flashMessagesStore } = await import('../../stores/flashMessages');
	const { sessionManager } = await import('../../session-manager');
	const { generateEd25519KeyPairFallback, publicKeyToHex } = await import('../../ed25519');
	const { privateKeyBytesToHex } = await import('../../ed25519/ed25519-core');
	const { httpSignedPOSTRequest } = await import('../../httpSignedRequests');
	const { authStore } = await import('../../stores/auth');
	const { t } = await import('../../stores/i18n');

	// Get current language for translated flash messages
	const lang = await getCurrentLanguage();

	try {
		// Generate NEW Ed25519 keypair for potential rotation
		const newKeyPair = await generateEd25519KeyPairFallback();
		const newPubKeyHex = publicKeyToHex(newKeyPair.publicKeyBytes);
		const newPrivKeyHex = privateKeyBytesToHex(newKeyPair.privateKeyBytes!);

		// Send refresh request with new_pub_key
		const data = await httpSignedPOSTRequest<{ new_pub_key: string }, LoginResponse>(
			`${API_BASE}/refresh`,
			{ new_pub_key: newPubKeyHex },
			false,
			{ credentials: 'include' }
		);

		// Update auth store with new token
		const user = {
			user_id: data.user_id,
			isAuthenticated: true
		};

		authStore.updateTokens(user, data.access_token);

		// Update session expiration timestamp if provided
		if (data.expires_at) {
			try {
				const { storeSessionExpiration } = await import('../../session-storage');
				await storeSessionExpiration(data.expires_at);
			} catch (error) {
				console.warn('Failed to store session expiration during refresh:', error);
			}
		}

		// CONDITIONAL KEY ROTATION
		if (data.server_pub_key) {
			// TRAMO 2/3: Backend sent server_pub_key → Full key rotation
			const { storeKeyPair } = await import('../../ed25519/ed25519-database');
			await storeKeyPair(newKeyPair);
			await sessionManager.setPrivKey(newPrivKeyHex);
		}

		// Ensure crypto tokens exist
		const { ensureCryptoTokensExist } = await import('../../utils/auth-recovery');
		const tokensValid = await ensureCryptoTokensExist('Token Refresh');
		if (!tokensValid) {
			return false;
		}

		return true;
	} catch (error) {
		console.error('Token refresh failed:', error);
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
