/**
 * Authentication store for managing user login state
 *
 * Provides reactive authentication state management with JWT token handling,
 * magic link authentication flow, and automatic token refresh capabilities.
 */

import { writable } from 'svelte/store';
import type { AuthUser, LoginResponse, MagicLinkResponse } from '../types';
import { api } from '../api';

interface AuthState {
	user: AuthUser | null;
	isLoading: boolean;
	error: string | null;
	accessToken: string | null;
}

const initialState: AuthState = {
	user: null,
	isLoading: false,
	error: null,
	accessToken: null
};

// Create the main auth store
const { subscribe, set, update } = writable<AuthState>(initialState);

/**
 * Load authentication state from localStorage on initialization
 */
function loadAuthFromStorage(): void {
	if (typeof window === 'undefined') return;

	try {
		const storedAuth = localStorage.getItem('auth_user');
		const storedToken = localStorage.getItem('access_token');

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
 * Save authentication state to localStorage
 */
function saveAuthToStorage(user: AuthUser, accessToken: string): void {
	if (typeof window === 'undefined') return;

	try {
		localStorage.setItem('auth_user', JSON.stringify(user));
		localStorage.setItem('access_token', accessToken);
	} catch (error) {
		console.warn('Failed to save auth to storage:', error);
	}
}

/**
 * Clear authentication state from localStorage
 */
function clearAuthFromStorage(): void {
	if (typeof window === 'undefined') return;

	localStorage.removeItem('auth_user');
	localStorage.removeItem('access_token');
}

/**
 * Authentication store with actions
 */
export const authStore = {
	subscribe,

	/**
	 * Initialize the auth store by loading from localStorage
	 */
	init(): void {
		loadAuthFromStorage();
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
	 * @returns Promise<LoginResponse>
	 */
	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		update((state) => ({ ...state, isLoading: true, error: null }));

		try {
			const loginResponse = await api.validateMagicLink(magicToken);

			// Calculate token expiration (15 minutes from now)
			const expiresAt = new Date();
			expiresAt.setSeconds(expiresAt.getSeconds() + loginResponse.expires_in);

			const user: AuthUser = {
				email: '', // Not needed for Zero Knowledge auth
				username: loginResponse.username, // Base58 user_id
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

			// Save to localStorage
			saveAuthToStorage(user, loginResponse.access_token);

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
	isAuthenticated(): Promise<boolean> {
		return new Promise((resolve) => {
			// Subscribe once to get current state
			const unsubscribe = subscribe((state) => {
				unsubscribe();

				console.log('[DEBUG] AuthStore isAuthenticated check:', {
					hasUser: !!state.user,
					hasToken: !!state.accessToken,
					userEmail: state.user?.email,
					tokenExists: !!state.accessToken,
					expiresAt: state.user?.expiresAt
				});

				if (!state.user || !state.accessToken) {
					console.log('[DEBUG] Not authenticated - missing user or token');
					resolve(false);
					return;
				}

				// Check token expiration
				if (state.user.expiresAt && new Date(state.user.expiresAt) <= new Date()) {
					console.log('[DEBUG] Token expired, logging out');
					// Token expired, logout
					authStore.logout();
					resolve(false);
					return;
				}

				console.log('[DEBUG] User is authenticated');
				resolve(true);
			});
		});
	},

	/**
	 * Get current access token
	 *
	 * @returns string | null
	 */
	getAccessToken(): Promise<string | null> {
		return new Promise((resolve) => {
			const unsubscribe = subscribe((state) => {
				unsubscribe();
				resolve(state.accessToken);
			});
		});
	},

	/**
	 * Logout user and clear all authentication data
	 */
	logout(): void {
		set(initialState);
		clearAuthFromStorage();
	},

	/**
	 * Clear any error state
	 */
	clearError(): void {
		update((state) => ({ ...state, error: null }));
	}
};

// Initialize the store when module loads
if (typeof window !== 'undefined') {
	authStore.init();
}
