/**
 * Session Auth Module - Authentication Data Management
 *
 * Single Responsibility: Handle authentication data in session
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

import { sessionDB } from './session-db';

/**
 * Get auth data
 */
export async function getAuthData(): Promise<{
	user: { user_id: string; email: string; isAuthenticated: boolean } | null;
	access_token: string | null;
	server_pub_key: string | null;
	server_x25519_pub_key: string | null;
}> {
	const session = await sessionDB.getSession();
	return {
		user: session.auth_user,
		access_token: session.access_token,
		server_pub_key: session.server_pub_key,
		server_x25519_pub_key: session.server_x25519_pub_key
	};
}

/**
 * Set auth data
 */
export async function setAuthData(
	user: { user_id: string; email: string; isAuthenticated: boolean },
	access_token: string
): Promise<void> {
	await sessionDB.updateSession({
		auth_user: user,
		access_token
	});
}

/**
 * Get user email (for UX display in forms)
 * Returns null if user is not authenticated
 */
export async function getUserEmail(): Promise<string | null> {
	const session = await sessionDB.getSession();
	return session.auth_user?.email ?? null;
}

/**
 * Check if user is authenticated
 */
export async function isAuthenticated(): Promise<boolean> {
	const authData = await getAuthData();
	return !!(authData.user?.isAuthenticated && authData.access_token);
}

/**
 * Set server public key for signed response validation
 */
export async function setServerPubKey(serverPubKey: string): Promise<void> {
	await sessionDB.updateSession({
		server_pub_key: serverPubKey
	});
}

/**
 * Get server public key for signed response validation
 */
export async function getServerPubKey(): Promise<string | null> {
	const session = await sessionDB.getSession();
	return session.server_pub_key;
}

/**
 * Clear server public key (called during logout)
 */
export async function clearServerPubKey(): Promise<void> {
	await sessionDB.updateSession({
		server_pub_key: null
	});
}

/**
 * Set server X25519 public key for ECDH (E2E encryption)
 */
export async function setServerX25519PubKey(serverX25519PubKey: string): Promise<void> {
	await sessionDB.updateSession({
		server_x25519_pub_key: serverX25519PubKey
	});
}

/**
 * Get server X25519 public key for ECDH (E2E encryption)
 */
export async function getServerX25519PubKey(): Promise<string | null> {
	const session = await sessionDB.getSession();
	return session.server_x25519_pub_key;
}

/**
 * Clear server X25519 public key (called during logout)
 */
export async function clearServerX25519PubKey(): Promise<void> {
	await sessionDB.updateSession({
		server_x25519_pub_key: null
	});
}

/**
 * Get client private key (for Ed25519 signing and key rotation)
 */
export async function getPrivKey(): Promise<string | null> {
	const session = await sessionDB.getSession();
	return session.priv_key;
}

/**
 * Set client private key (for Ed25519 key rotation)
 */
export async function setPrivKey(privKey: string): Promise<void> {
	await sessionDB.updateSession({
		priv_key: privKey
	});
}

/**
 * Clear client private key (called during logout)
 */
export async function clearPrivKey(): Promise<void> {
	await sessionDB.updateSession({
		priv_key: null
	});
}

/**
 * Clear auth data only, PRESERVE user preferences (for preventive cleanup)
 */
export async function clearAuthData(): Promise<void> {
	const session = await sessionDB.getSession();

	// Clear ONLY auth-related data, preserve preferences
	session.cipher_token = null;
	session.nonce_token = null;
	session.hmac_key = null;
	session.prehashSeeds = [];
	session.auth_user = null;
	session.access_token = null;
	session.server_pub_key = null; // Clear server Ed25519 public key on logout
	session.server_x25519_pub_key = null; // Clear server X25519 public key on logout
	session.priv_key = null; // Clear client private key on logout
	session.authFlow.pending_email = null;

	// Keep userPreferences intact for UX
	await sessionDB.saveSession(session);
	// Auth data cleared, preferences preserved
}
