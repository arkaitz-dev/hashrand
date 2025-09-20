/**
 * Authentication store for managing user login state
 *
 * Provides reactive authentication state management with JWT token handling,
 * magic link authentication flow, and automatic token refresh capabilities.
 */

import { writable, get } from 'svelte/store';
import type { AuthUser, LoginResponse, MagicLinkResponse } from '../types';
import { api } from '../api';

interface AuthState {
	user: AuthUser | null;
	isLoading: boolean;
	isRefreshing: boolean;
	error: string | null;
	accessToken: string | null;
	// Cache for crypto tokens (synced with IndexedDB)
	cipherToken: string | null;
	nonceToken: string | null;
	hmacKey: string | null;
}

const initialState: AuthState = {
	user: null,
	isLoading: false,
	isRefreshing: false,
	error: null,
	accessToken: null,
	cipherToken: null,
	nonceToken: null,
	hmacKey: null
};

// Create the main auth store
const { subscribe, set, update } = writable<AuthState>(initialState);

/**
 * Load authentication state from IndexedDB on initialization
 */
async function loadAuthFromStorage(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		const { sessionManager } = await import('../session-manager');
		const authData = await sessionManager.getAuthData();
		const cryptoTokens = await sessionManager.getCryptoTokens();

		// Update store with all data from IndexedDB
		update((state) => ({
			...state,
			user: authData.user,
			accessToken: authData.access_token,
			cipherToken: cryptoTokens.cipher,
			nonceToken: cryptoTokens.nonce,
			hmacKey: cryptoTokens.hmac
		}));
	} catch (error) {
		// Failed to load auth from IndexedDB
		await clearSensitiveAuthData();
	}
}

/**
 * Save authentication state to IndexedDB
 */
async function saveAuthToStorage(user: AuthUser, accessToken: string): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		const { sessionManager } = await import('../session-manager');
		await sessionManager.setAuthData(user, accessToken);
	} catch (error) {
		// Failed to save auth to IndexedDB
	}
}

/**
 * Clear ALL authentication data preventively (before asking for email)
 * Preserves only user preferences (language, theme) for UX
 */
async function clearPreventiveAuthData(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Clear auth data only, PRESERVE user preferences for UX
		const { sessionManager } = await import('../session-manager');
		await sessionManager.clearAuthData();

		// Clear cache in store
		update((state) => ({
			...state,
			user: null,
			accessToken: null,
			cipherToken: null,
			nonceToken: null,
			hmacKey: null
		}));

		// Preserve user preferences for UX (language and theme are kept)
	} catch (error) {
		// Failed to clear preventive auth data from IndexedDB
	}
}

/**
 * Clear sensitive authentication data only (for token expiration/errors)
 */
async function clearSensitiveAuthData(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Clear IndexedDB session data
		const { sessionManager } = await import('../session-manager');
		await sessionManager.clearSession();

		// Clear cache in store
		update((state) => ({
			...state,
			user: null,
			accessToken: null,
			cipherToken: null,
			nonceToken: null,
			hmacKey: null
		}));

		// Note: pending_auth_email is cleared immediately on successful auth,
		// not here, to preserve ongoing magic link flows during token errors
	} catch (error) {
		// Failed to clear sensitive auth data from IndexedDB
	}
}

/**
 * Clear store state (called after IndexedDB cleanup in logout)
 */
async function clearAuthFromStorage(): Promise<void> {
	if (typeof window === 'undefined') return;

	// Note: sessionManager.clearSession() already cleared ALL IndexedDB data
	// including auth tokens, crypto tokens, and user preferences

	// Only need to reset store to initial state
	set(initialState);
}

/**
 * Generate cryptographically secure cipher, nonce and HMAC key tokens
 */
async function generateCryptoTokens(): Promise<void> {
	if (typeof window === 'undefined') {
		// generateCryptoTokens() skipped - not in browser environment
		return;
	}

	// Starting crypto tokens generation

	// Starting crypto tokens generation

	// Check if tokens already exist
	try {
		const { sessionManager } = await import('../session-manager');
		const existingTokens = await sessionManager.getCryptoTokens();
		if (existingTokens.cipher && existingTokens.nonce && existingTokens.hmac) {
				// Crypto tokens already exist, skipping generation

			// Crypto tokens already exist, skipping generation
			return;
		}
	} catch (error) {
			// Failed to check existing crypto tokens, proceeding with generation

		// Error checking existing tokens, continuing
	}

	try {
			// Generating new 32-byte crypto tokens

		// Generate 32-byte tokens using Web Crypto API
		const cipherToken = new Uint8Array(32);
		const nonceToken = new Uint8Array(32);
		const hmacToken = new Uint8Array(32);

		crypto.getRandomValues(cipherToken);
		crypto.getRandomValues(nonceToken);
		crypto.getRandomValues(hmacToken);

		// Converting tokens to base64
		// Convert to base64 for storage
		const cipherB64 = btoa(String.fromCharCode(...cipherToken));
		const nonceB64 = btoa(String.fromCharCode(...nonceToken));
		const hmacB64 = btoa(String.fromCharCode(...hmacToken));

		// Store in IndexedDB
		const { sessionManager } = await import('../session-manager');
		await sessionManager.setCryptoTokens(cipherB64, nonceB64, hmacB64);

		// Update cache in store
		update((state) => ({
			...state,
			cipherToken: cipherB64,
			nonceToken: nonceB64,
			hmacKey: hmacB64
		}));

		// Crypto tokens generated successfully
	} catch (error) {
		// Failed to generate crypto tokens
	}
}

/**
 * Check if crypto tokens exist in IndexedDB
 */
async function hasCryptoTokens(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('../session-manager');
		return await sessionManager.hasCryptoTokens();
	} catch (error) {
		// Failed to check crypto tokens in IndexedDB
		return false;
	}
}

/**
 * Check if refresh cookie exists and is valid
 */
async function hasValidRefreshCookie(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		// Try to refresh token to see if refresh cookie is valid
		const response = await fetch('/api/refresh', {
			method: 'POST',
			credentials: 'include'
		});

		return response.ok;
	} catch {
		return false;
	}
}

/**
 * Authentication store with actions
 */
export const authStore = {
	subscribe,

	/**
	 * Initialize the auth store by loading from IndexedDB
	 */
	async init(): Promise<void> {
		// Initialize SessionManager (with automatic migration from sessionStorage)
		const { sessionManager } = await import('../session-manager');
		await sessionManager.init();

		// Load existing session data
		await loadAuthFromStorage();

		// Only check if we need to refresh session for existing users
		// but don't generate crypto tokens here
		await this.checkSessionValidity();
	},

	/**
	 * Check session validity and handle expired refresh cookies
	 */
	async checkSessionValidity(): Promise<void> {
		// If we have access token but no crypto tokens, something might be wrong
		const state = get(authStore);
		if (state.accessToken && !(await hasCryptoTokens())) {
			// Check if refresh cookie is still valid
			const hasValidCookie = await hasValidRefreshCookie();

			if (!hasValidCookie) {
				// Refresh cookie expired/invalid, clear sensitive data
				await clearSensitiveAuthData();
			}
			// If valid cookie exists, crypto tokens will be generated on next API call via refresh
		}
	},

	/**
	 * Generate crypto tokens - only called after successful login/refresh
	 */
	async generateCryptoTokens(): Promise<void> {
		await generateCryptoTokens();
	},

	/**
	 * Ensure authentication by trying refresh only if no access token exists
	 * Returns true if authenticated (or after successful refresh), false if needs login
	 */
	async ensureAuthenticated(): Promise<boolean> {
		// Check if we already have access token in IndexedDB
		try {
			const { sessionManager } = await import('../session-manager');
			const authData = await sessionManager.getAuthData();

			if (authData.access_token && authData.user) {
				// We have tokens - backend will validate expiration
				if (authData.user.isAuthenticated && authData.user.user_id) {
					return true; // Valid session exists - NO refresh needed
				}
			}
		} catch (error) {
			// Failed to load auth data from IndexedDB
			// Clear invalid data and continue to refresh
			await clearSensitiveAuthData();
		}

		// No valid access token in sessionStorage, try to refresh using cookie
		// No valid access token found, attempting automatic refresh

		// Set refreshing state
		update((state) => ({ ...state, isRefreshing: true }));

		try {
			// Import api to avoid circular dependencies
			const { api } = await import('../api');
			const refreshSuccess = await api.refreshToken();

			if (refreshSuccess) {
				// Automatic refresh successful
				return true;
			} else {
				// Automatic refresh failed - login required
				return false;
			}
		} catch (error) {
			// Refresh attempt failed
			return false;
		} finally {
			// Always clear refreshing state
			update((state) => ({ ...state, isRefreshing: false }));
		}
	},

	/**
	 * Request magic link for email authentication
	 *
	 * @param email - User email address
	 * @param next - Optional Base58-encoded parameters to include in magic link URL
	 * @returns Promise<MagicLinkResponse>
	 */
	async requestMagicLink(email: string, next: string = "/"): Promise<MagicLinkResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			// Capture current UI host for magic link generation
			const ui_host = typeof window !== 'undefined' ? window.location.origin : '';

			if (!ui_host) {
				throw new Error('UI host is required for magic link generation');
			}

			const response = await api.requestMagicLink(email, ui_host, next);

			update((state) => ({ ...state, isLoading: false }));
			return response;
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Failed to request magic link';
			update((state) => ({
				...state,
				isLoading: false,
				error: errorMessage
			}));
			throw error;
		}
	},

	/**
	 * Validate magic link and complete authentication
	 *
	 * @param magicToken - Magic link token from URL parameter (Ed25519 verified by backend)
	 * @returns Promise<LoginResponse>
	 */
	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			const loginResponse = await api.validateMagicLink(magicToken);

			const user: AuthUser = {
				user_id: loginResponse.user_id, // Base58 user_id
				isAuthenticated: true
			};

			// Update store state
			update((state) => ({
				...state,
				user,
				accessToken: loginResponse.access_token,
				isLoading: false,
				error: null
			}));

			// Save to IndexedDB
			await saveAuthToStorage(user, loginResponse.access_token);

			// Show flash message for successful magic link validation
			try {
				const { flashMessagesStore } = await import('../stores/flashMessages');
				flashMessagesStore.addMessage('✅ Magic link validado exitosamente!');
			} catch (flashError) {
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
				const { sessionManager } = await import('../session-manager');
				await sessionManager.clearPendingAuthEmail();
			} catch (error) {
				// Failed to clear pending auth email from IndexedDB
			}

			return loginResponse;
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'Authentication failed';
			update((state) => ({
				...state,
				isLoading: false,
				error: errorMessage
			}));
			throw error;
		}
	},

	/**
	 * Check if user is currently authenticated
	 *
	 * @returns boolean
	 */
	async isAuthenticated(): Promise<boolean> {
		// Get current state directly without subscription
		const state = get(authStore);

		if (!state.user || !state.accessToken) {
			return false;
		}

		// Backend handles token expiration
		return true;
	},

	/**
	 * Get current access token
	 *
	 * @returns string | null
	 */
	getAccessToken(): string | null {
		const state = get(authStore);
		return state.accessToken;
	},

	/**
	 * Get cipher token from cache (synced with IndexedDB)
	 *
	 * @returns string | null
	 */
	getCipherToken(): string | null {
		const state = get(authStore);
		return state.cipherToken;
	},

	/**
	 * Get nonce token from cache (synced with IndexedDB)
	 *
	 * @returns string | null
	 */
	getNonceToken(): string | null {
		const state = get(authStore);
		return state.nonceToken;
	},

	/**
	 * Get HMAC key from cache (synced with IndexedDB)
	 *
	 * @returns string | null
	 */
	getHmacKey(): string | null {
		const state = get(authStore);
		return state.hmacKey;
	},

	/**
	 * Logout user and clear all authentication data
	 */
	async logout(): Promise<void> {
		// Call API logout to clear server-side session and refresh token cookie
		await api.logout();

		// Clear Ed25519 keypairs for security
		try {
			const { clearAllKeyPairs } = await import('../ed25519');
			await clearAllKeyPairs();
		} catch (error) {
			// Failed to clear Ed25519 keypairs
		}

		// Clear ALL IndexedDB session data
		try {
			const { sessionManager } = await import('../session-manager');
			await sessionManager.clearSession();
		} catch (error) {
			// Failed to clear IndexedDB session
		}

		// Clear local state and remaining storage
		set(initialState);
		await clearAuthFromStorage();
	},

	/**
	 * Clear any error state
	 */
	clearError(): void {
		update((state) => ({ ...state, error: null }));
	},

	/**
	 * Update tokens after refresh (internal method)
	 */
	updateTokens(user: AuthUser, accessToken: string): void {
		update((state) => ({
			...state,
			user,
			accessToken,
			error: null
		}));

		// Save to IndexedDB asynchronously (no await to maintain sync interface)
		saveAuthToStorage(user, accessToken).catch((error) => {
			// Failed to save auth tokens to IndexedDB
		});

		// Note: Crypto tokens are NOT regenerated during refresh
		// They persist throughout the session for URL parameter encryption consistency
		// updateTokens() - preserving existing crypto tokens during refresh

		// Show flash message that tokens were refreshed (but crypto tokens preserved)
		import('../stores/flashMessages')
			.then(() => {
				// Access token updated (crypto tokens preserved)
			})
			.catch(() => {});

		// Clear pending auth email - no longer needed after successful token update (fire-and-forget)
		import('../session-manager')
			.then(({ sessionManager }) => sessionManager.clearPendingAuthEmail())
			.catch((error) => {
				// Failed to clear pending auth email from IndexedDB
			});
	},

	/**
	 * Clear all authentication data preventively before showing login dialog
	 * Ensures clean state regardless of how previous session ended
	 */
	async clearPreventiveAuthData(): Promise<void> {
		await clearPreventiveAuthData();
	}
};

// Initialize the store when module loads
if (typeof window !== 'undefined') {
	authStore.init().catch((error) => {
		// Failed to initialize auth store
	});
}
