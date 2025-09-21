/**
 * Authentication Store - Refactored with SOLID Principles
 *
 * Simplified main store using specialized modules for different responsibilities.
 * Provides reactive authentication state management.
 */

import { writable, get } from 'svelte/store';
import type { AuthUser, LoginResponse, MagicLinkResponse } from '../types';
import {
	loadAuthFromStorage,
	clearPreventiveAuthData,
	checkSessionValidity,
	ensureAuthenticated,
	requestMagicLink as requestMagicLinkAction,
	validateMagicLink as validateMagicLinkAction,
	logout as logoutAction,
	generateCryptoTokens as generateCryptoTokensAction
} from './auth/index';

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
 * Update store state with data from IndexedDB
 */
async function refreshStoreFromStorage(): Promise<void> {
	const authData = await loadAuthFromStorage();
	update((state) => ({
		...state,
		user: authData.user,
		accessToken: authData.accessToken,
		cipherToken: authData.cipherToken,
		nonceToken: authData.nonceToken,
		hmacKey: authData.hmacKey
	}));
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
		await refreshStoreFromStorage();

		// Only check if we need to refresh session for existing users
		// but don't generate crypto tokens here
		await checkSessionValidity();
	},

	/**
	 * Check session validity and handle expired refresh cookies
	 */
	async checkSessionValidity(): Promise<void> {
		await checkSessionValidity();
	},

	/**
	 * Generate crypto tokens - only called after successful login/refresh
	 */
	async generateCryptoTokens(): Promise<void> {
		await generateCryptoTokensAction();
		// Refresh store state to include new crypto tokens
		await refreshStoreFromStorage();
	},

	/**
	 * Ensure authentication by trying refresh only if no access token exists
	 * Returns true if authenticated (or after successful refresh), false if needs login
	 */
	async ensureAuthenticated(): Promise<boolean> {
		const result = await ensureAuthenticated();
		// Refresh store state after potential token changes
		await refreshStoreFromStorage();
		return result;
	},

	/**
	 * Request magic link for email authentication
	 */
	async requestMagicLink(email: string, next: string = '/'): Promise<MagicLinkResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			const response = await requestMagicLinkAction(email, next);
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
	 */
	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			const { user, accessToken, loginResponse } = await validateMagicLinkAction(magicToken);

			// Update store state
			update((state) => ({
				...state,
				user,
				accessToken,
				isLoading: false,
				error: null
			}));

			// Refresh store state to include new crypto tokens
			await refreshStoreFromStorage();

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
	 */
	getAccessToken(): string | null {
		const state = get(authStore);
		return state.accessToken;
	},

	/**
	 * Get cipher token from cache (synced with IndexedDB)
	 */
	getCipherToken(): string | null {
		const state = get(authStore);
		return state.cipherToken;
	},

	/**
	 * Get nonce token from cache (synced with IndexedDB)
	 */
	getNonceToken(): string | null {
		const state = get(authStore);
		return state.nonceToken;
	},

	/**
	 * Get HMAC key from cache (synced with IndexedDB)
	 */
	getHmacKey(): string | null {
		const state = get(authStore);
		return state.hmacKey;
	},

	/**
	 * Logout user and clear all authentication data
	 */
	async logout(): Promise<void> {
		await logoutAction();

		// Clear local state and remaining storage
		set(initialState);
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
		import('./auth/auth-storage')
			.then(({ saveAuthToStorage }) => saveAuthToStorage(user, accessToken))
			.catch(() => {
				// Failed to save auth tokens to IndexedDB
			});

		// Note: Crypto tokens are NOT regenerated during refresh
		// They persist throughout the session for URL parameter encryption consistency

		// Clear pending auth email - no longer needed after successful token update (fire-and-forget)
		import('../session-manager')
			.then(({ sessionManager }) => sessionManager.clearPendingAuthEmail())
			.catch(() => {
				// Failed to clear pending auth email from IndexedDB
			});
	},

	/**
	 * Clear all authentication data preventively before showing login dialog
	 * Ensures clean state regardless of how previous session ended
	 */
	async clearPreventiveAuthData(): Promise<void> {
		await clearPreventiveAuthData();
		// Clear local state
		set(initialState);
	}
};

// Initialize the store when module loads
if (typeof window !== 'undefined') {
	authStore.init().catch(() => {
		// Failed to initialize auth store
	});
}
