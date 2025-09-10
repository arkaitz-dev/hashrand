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
}

const initialState: AuthState = {
	user: null,
	isLoading: false,
	isRefreshing: false,
	error: null,
	accessToken: null
};

// Create the main auth store
const { subscribe, set, update } = writable<AuthState>(initialState);

/**
 * Load authentication state from sessionStorage on initialization
 */
function loadAuthFromStorage(): void {
	if (typeof window === 'undefined') return;

	try {
		const storedAuth = sessionStorage.getItem('auth_user');
		const storedToken = sessionStorage.getItem('access_token');

		if (storedAuth && storedToken) {
			const user = JSON.parse(storedAuth);

			// Check if token is still valid
			if (user.expiresAt && new Date(user.expiresAt) > new Date()) {
				update((state) => ({
					...state,
					user,
					accessToken: storedToken
				}));
			} else {
				// Token expired, clear storage
				clearAuthFromStorage();
			}
		}
	} catch (error) {
		console.warn('Failed to load auth from storage:', error);
		clearAuthFromStorage();
	}
}

/**
 * Save authentication state to sessionStorage
 */
function saveAuthToStorage(user: AuthUser, accessToken: string): void {
	if (typeof window === 'undefined') return;

	try {
		sessionStorage.setItem('auth_user', JSON.stringify(user));
		sessionStorage.setItem('access_token', accessToken);
	} catch (error) {
		console.warn('Failed to save auth to storage:', error);
	}
}

/**
 * Clear authentication state from sessionStorage
 */
function clearAuthFromStorage(): void {
	if (typeof window === 'undefined') return;

	sessionStorage.removeItem('auth_user');
	sessionStorage.removeItem('access_token');
	// Also clear crypto tokens when clearing auth
	sessionStorage.removeItem('cipher_token');
	sessionStorage.removeItem('nonce_token');
	sessionStorage.removeItem('hmac_key');
}

/**
 * Generate cryptographically secure cipher, nonce and HMAC key tokens
 */
function generateCryptoTokens(): void {
	if (typeof window === 'undefined') return;

	try {
		// Generate 32-byte tokens using Web Crypto API
		const cipherToken = new Uint8Array(32);
		const nonceToken = new Uint8Array(32);
		const hmacToken = new Uint8Array(32);
		
		crypto.getRandomValues(cipherToken);
		crypto.getRandomValues(nonceToken);
		crypto.getRandomValues(hmacToken);

		// Convert to base64 for storage
		const cipherB64 = btoa(String.fromCharCode(...cipherToken));
		const nonceB64 = btoa(String.fromCharCode(...nonceToken));
		const hmacB64 = btoa(String.fromCharCode(...hmacToken));

		sessionStorage.setItem('cipher_token', cipherB64);
		sessionStorage.setItem('nonce_token', nonceB64);
		sessionStorage.setItem('hmac_key', hmacB64);

		// Show tokens in flash messages for debugging
		import('./flashMessages').then(({ flashMessagesStore }) => {
			flashMessagesStore.addMessages([
				`🔐 Cipher Token: ${cipherB64.substring(0, 16)}...${cipherB64.slice(-8)}`,
				`🎲 Nonce Token: ${nonceB64.substring(0, 16)}...${nonceB64.slice(-8)}`,
				`🔑 HMAC Key: ${hmacB64.substring(0, 16)}...${hmacB64.slice(-8)}`
			]);
		});
	} catch (error) {
		console.warn('Failed to generate crypto tokens:', error);
	}
}

/**
 * Check if cipher, nonce and HMAC key tokens exist in sessionStorage
 */
function hasCryptoTokens(): boolean {
	if (typeof window === 'undefined') return false;
	
	return !!(sessionStorage.getItem('cipher_token') && sessionStorage.getItem('nonce_token') && sessionStorage.getItem('hmac_key'));
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
			credentials: 'include',
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
	 * Initialize the auth store by loading from sessionStorage
	 */
	async init(): Promise<void> {
		// Load existing session data
		loadAuthFromStorage();
		
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
		if (state.accessToken && !hasCryptoTokens()) {
			// Check if refresh cookie is still valid
			const hasValidCookie = await hasValidRefreshCookie();
			
			if (!hasValidCookie) {
				// Refresh cookie expired/invalid, clear everything
				clearAuthFromStorage();
			}
			// If valid cookie exists, crypto tokens will be generated on next API call via refresh
		}
	},

	/**
	 * Generate crypto tokens - only called after successful login/refresh
	 */
	generateCryptoTokens(): void {
		generateCryptoTokens();
	},

	/**
	 * Ensure authentication by trying refresh only if no access token exists
	 * Returns true if authenticated (or after successful refresh), false if needs login
	 */
	async ensureAuthenticated(): Promise<boolean> {
		// Check if we already have access token in sessionStorage
		const hasToken = typeof window !== 'undefined' && sessionStorage.getItem('access_token');
		const hasUser = typeof window !== 'undefined' && sessionStorage.getItem('auth_user');

		if (hasToken && hasUser) {
			// We have tokens, verify they're not expired
			try {
				const user = JSON.parse(hasUser);
				if (user.expiresAt && new Date(user.expiresAt) > new Date()) {
					return true; // Valid session exists - NO refresh needed
				}
				// Token expired, clear it and continue to refresh
				sessionStorage.removeItem('access_token');
				sessionStorage.removeItem('auth_user');
				sessionStorage.removeItem('cipher_token');
				sessionStorage.removeItem('nonce_token');
				sessionStorage.removeItem('hmac_key');
				sessionStorage.removeItem('hmac_key');
			} catch (error) {
				console.warn('Failed to parse user data from sessionStorage:', error);
				// Clear invalid data and continue to refresh
				sessionStorage.removeItem('access_token');
				sessionStorage.removeItem('auth_user');
				sessionStorage.removeItem('cipher_token');
				sessionStorage.removeItem('nonce_token');
				sessionStorage.removeItem('hmac_key');
				sessionStorage.removeItem('hmac_key');
			}
		}

		// No valid access token in sessionStorage, try to refresh using cookie
		console.log('🔄 No valid access token found, attempting automatic refresh...');
		
		// Set refreshing state
		update((state) => ({ ...state, isRefreshing: true }));
		
		try {
			// Import api to avoid circular dependencies
			const { api } = await import('../api');
			const refreshSuccess = await api.refreshToken();
			
			if (refreshSuccess) {
				console.log('✅ Automatic refresh successful');
				return true;
			} else {
				console.log('❌ Automatic refresh failed - login required');
				return false;
			}
		} catch (error) {
			console.warn('Refresh attempt failed:', error);
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
	async requestMagicLink(email: string, next?: string): Promise<MagicLinkResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			// Capture current UI host for magic link generation
			const ui_host = typeof window !== 'undefined' ? window.location.origin : undefined;

			const response = await api.requestMagicLink({
				email,
				ui_host,
				next
			});

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
	 * @param magicToken - Magic link token from URL parameter
	 * @param randomHash - Random hash from localStorage for additional validation
	 * @returns Promise<LoginResponse>
	 */
	async validateMagicLink(magicToken: string, randomHash: string): Promise<LoginResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			const loginResponse = await api.validateMagicLink(magicToken, randomHash);

			// Calculate token expiration (15 minutes from now)
			const expiresAt = new Date();
			expiresAt.setSeconds(expiresAt.getSeconds() + loginResponse.expires_in);

			const user: AuthUser = {
				email: '', // Not needed for Zero Knowledge auth
				user_id: loginResponse.user_id, // Base58 user_id
				isAuthenticated: true,
				expiresAt
			};

			// Update store state
			update((state) => ({
				...state,
				user,
				accessToken: loginResponse.access_token,
				isLoading: false,
				error: null
			}));

			// Save to sessionStorage
			saveAuthToStorage(user, loginResponse.access_token);

			// Generate crypto tokens ONLY after successful login
			generateCryptoTokens();

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

		// Check token expiration
		if (state.user.expiresAt && new Date(state.user.expiresAt) <= new Date()) {
			// Token expired, logout
			this.logout();
			return false;
		}
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
	 * Get cipher token from sessionStorage
	 *
	 * @returns string | null
	 */
	getCipherToken(): string | null {
		if (typeof window === 'undefined') return null;
		return sessionStorage.getItem('cipher_token');
	},

	/**
	 * Get nonce token from sessionStorage
	 *
	 * @returns string | null
	 */
	getNonceToken(): string | null {
		if (typeof window === 'undefined') return null;
		return sessionStorage.getItem('nonce_token');
	},

	/**
	 * Get HMAC key from sessionStorage
	 *
	 * @returns string | null
	 */
	getHmacKey(): string | null {
		if (typeof window === 'undefined') return null;
		return sessionStorage.getItem('hmac_key');
	},

	/**
	 * Logout user and clear all authentication data
	 */
	async logout(): Promise<void> {
		// Call API logout to clear server-side session and refresh token cookie
		await api.logout();

		// Clear local state and storage
		set(initialState);
		clearAuthFromStorage();
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

		saveAuthToStorage(user, accessToken);
	}
};

// Initialize the store when module loads
if (typeof window !== 'undefined') {
	authStore.init().catch((error) => {
		console.warn('Failed to initialize auth store:', error);
	});
}
