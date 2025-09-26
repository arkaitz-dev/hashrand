/**
 * Session Expiry Manager - Proactive session expiration handling
 *
 * Single Responsibility: Handle session expiration checks and cleanup flows
 * Used by AuthStatusButton, Generate buttons, and result route for proactive auth management
 */

import { getSessionExpiration, clearSessionExpiration } from './session-storage';

/**
 * Check if current session has expired based on stored timestamp
 *
 * @returns Promise<boolean> - true if session is expired or no timestamp stored
 */
export async function isSessionExpired(): Promise<boolean> {
	try {
		const expiresAt = await getSessionExpiration();

		// No stored expiration = consider expired
		if (!expiresAt) return true;

		const now = Math.floor(Date.now() / 1000); // Convert to seconds
		return now >= expiresAt;
	} catch (error) {
		console.warn('Failed to check session expiration:', error);
		// On error, assume expired for security
		return true;
	}
}

/**
 * Handle expired session cleanup - clear all auth data from IndexedDB
 *
 * @returns Promise<void>
 */
export async function handleExpiredSession(): Promise<void> {
	try {
		console.log('🕐 Session expired - performing complete cleanup');

		// Clear session expiration timestamp
		await clearSessionExpiration();

		// Clear all IndexedDB session data
		try {
			const { sessionManager } = await import('./session-manager');
			await sessionManager.clearSession();
		} catch (error) {
			console.warn('Failed to clear session data:', error);
		}

		// Clear Ed25519 keypairs for security
		try {
			const { clearAllKeyPairs } = await import('./ed25519');
			await clearAllKeyPairs();
		} catch (error) {
			console.warn('Failed to clear Ed25519 keypairs:', error);
		}

		console.log('✅ Expired session cleanup completed');
	} catch (error) {
		console.error('Failed to handle expired session:', error);
		// Even if cleanup fails, continue with auth flow
	}
}

/**
 * Launch magic link authentication dialog with optional next parameter
 *
 * @param next - Optional next URL to redirect after authentication
 * @returns Promise<void>
 */
export async function launchMagicLinkFlow(next?: string): Promise<void> {
	try {
		const { dialogStore } = await import('./stores/dialog');

		const authConfig = {
			destination: {
				route: next || '/'
			}
		};

		dialogStore.show('auth', authConfig);
		console.log('🔐 Magic link dialog launched', next ? `with next: ${next}` : '');
	} catch (error) {
		console.error('Failed to launch magic link flow:', error);
		// Fallback - redirect to home if dialog fails
		if (typeof window !== 'undefined') {
			window.location.href = '/';
		}
	}
}

/**
 * Complete expired session flow: cleanup + magic link dialog
 * Used when user interaction triggers the expiration check
 *
 * @param next - Optional next URL for post-auth redirect
 * @returns Promise<void>
 */
export async function handleExpiredSessionWithAuth(next?: string): Promise<void> {
	// First cleanup expired session
	await handleExpiredSession();

	// Then launch magic link flow
	await launchMagicLinkFlow(next);
}

/**
 * Check session expiration and handle accordingly
 * Returns true if session is valid, false if expired (and handled)
 *
 * @param options - Configuration options
 * @param options.onExpired - Callback when session is expired ('cleanup-only' | 'launch-auth')
 * @param options.next - Optional next URL for auth flow
 * @returns Promise<boolean> - true if session is valid, false if expired
 */
export async function checkSessionAndHandle(options: {
	onExpired: 'cleanup-only' | 'launch-auth';
	next?: string;
}): Promise<boolean> {
	const expired = await isSessionExpired();

	if (!expired) return true;

	// Session is expired - handle according to options
	if (options.onExpired === 'cleanup-only') {
		await handleExpiredSession();
	} else {
		await handleExpiredSessionWithAuth(options.next);
	}

	return false;
}
