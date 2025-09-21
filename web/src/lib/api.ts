/**
 * API Client - Refactored with DRY Principles
 *
 * Simplified main API using specialized modules to eliminate code duplication.
 * Provides unified API interface with authentication middleware.
 */

import type {
	GenerateParams,
	PasswordParams,
	ApiKeyParams,
	MnemonicParams,
	VersionResponse,
	SeedGenerateRequest,
	SeedPasswordRequest,
	SeedApiKeyRequest,
	SeedMnemonicRequest,
	CustomHashResponse,
	LoginResponse,
	MagicLinkResponse
} from './types';

import {
	ApiError,
	handleJsonResponse,
	generate as generateHash,
	generatePassword as generatePasswordHash,
	generateApiKey as generateApiKeyHash,
	generateMnemonic as generateMnemonicHash,
	generateCustomWithSeed,
	generatePasswordWithSeed,
	generateApiKeyWithSeed,
	generateMnemonicWithSeed,
	requestMagicLink as requestMagicLinkAuth,
	validateMagicLink as validateMagicLinkAuth,
	checkAuthStatus as checkAuthStatusAuth,
	logout as logoutAuth,
	refreshToken as refreshTokenAuth
} from './api/index';

const API_BASE = '/api';

// Helper function to get authentication headers
async function getAuthHeaders(): Promise<Record<string, string>> {
	try {
		const { sessionManager } = await import('./session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.user || !authData.access_token) {
			return {};
		}

		// NOTE: No time-based validation here - let backend decide and handle 401 with refresh
		return {
			Authorization: `Bearer ${authData.access_token}`
		};
	} catch {
		return {};
	}
}

/**
 * Handle proactive token renewal from response headers
 */
async function handleProactiveTokenRenewal(response: Response): Promise<void> {
	const newAccessToken = response.headers.get('x-new-access-token');
	const newExpiresIn = response.headers.get('x-token-expires-in');

	if (newAccessToken && newExpiresIn) {
		// Proactive token renewal detected, updating tokens

		// Update access token in IndexedDB
		try {
			const { sessionManager } = await import('./session-manager');
			const authData = await sessionManager.getAuthData();
			if (authData.user) {
				await sessionManager.setAuthData(authData.user, newAccessToken);
			}
		} catch {
			// Failed to update access token during proactive renewal
		}

		// NOTE: Crypto tokens are NOT regenerated during proactive renewal
		// They remain stable throughout the session for URL parameter encryption consistency

		// Proactive token renewal completed transparently
	}
}

/**
 * Authenticated fetch wrapper with automatic token refresh
 */
async function authenticatedFetch(url: string, options: RequestInit = {}): Promise<Response> {
	// Get initial auth headers
	const authHeaders = await getAuthHeaders();

	// Make initial request
	let response = await fetch(url, {
		...options,
		headers: {
			...options.headers,
			...authHeaders
		}
	});

	// If 401 Unauthorized, try refresh
	if (response.status === 401) {
		// Access token expired, attempting refresh
		const refreshSuccess = await refreshTokenAuth();

		if (refreshSuccess) {
			// Token refresh successful, retrying request
			const newAuthHeaders = await getAuthHeaders();
			response = await fetch(url, {
				...options,
				headers: {
					...options.headers,
					...newAuthHeaders
				}
			});
		} else {
			// Token refresh failed, force logout
			const { authStore } = await import('./stores/auth');
			const { dialogStore } = await import('./stores/dialog');

			await authStore.logout();
			authStore.clearPreventiveAuthData();

			const authConfig = {
				destination: { route: '/' }
			};
			dialogStore.show('auth', authConfig);
		}
	}

	// Check for proactive token renewal headers in successful responses
	if (response.ok) {
		await handleProactiveTokenRenewal(response);
	}

	return response;
}

/**
 * Main API object with simplified interface
 */
export const api = {
	// Generation endpoints (GET)
	async generate(params: GenerateParams): Promise<CustomHashResponse> {
		return await generateHash(params, authenticatedFetch);
	},

	async generatePassword(params: PasswordParams): Promise<CustomHashResponse> {
		return await generatePasswordHash(params, authenticatedFetch);
	},

	async generateApiKey(params: ApiKeyParams): Promise<CustomHashResponse> {
		return await generateApiKeyHash(params, authenticatedFetch);
	},

	async generateMnemonic(params: MnemonicParams = {}): Promise<CustomHashResponse> {
		return await generateMnemonicHash(params, authenticatedFetch);
	},

	// Version endpoint (public)
	async getVersion(): Promise<VersionResponse> {
		const response = await fetch(`${API_BASE}/version`);
		return handleJsonResponse<VersionResponse>(response);
	},

	// Seed-based endpoints (POST)
	async generateWithSeed(seedRequest: SeedGenerateRequest): Promise<CustomHashResponse> {
		return await generateCustomWithSeed(seedRequest, authenticatedFetch);
	},

	async generatePasswordWithSeed(seedRequest: SeedPasswordRequest): Promise<CustomHashResponse> {
		return await generatePasswordWithSeed(seedRequest, authenticatedFetch);
	},

	async generateApiKeyWithSeed(seedRequest: SeedApiKeyRequest): Promise<CustomHashResponse> {
		return await generateApiKeyWithSeed(seedRequest, authenticatedFetch);
	},

	async generateMnemonicWithSeed(seedRequest: SeedMnemonicRequest): Promise<CustomHashResponse> {
		return await generateMnemonicWithSeed(seedRequest, authenticatedFetch);
	},

	// Authentication endpoints
	async requestMagicLink(
		email: string,
		ui_host: string,
		next: string = '/'
	): Promise<MagicLinkResponse> {
		return await requestMagicLinkAuth(email, ui_host, next);
	},

	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		return await validateMagicLinkAuth(magicToken);
	},

	async checkAuthStatus(): Promise<boolean> {
		return await checkAuthStatusAuth();
	},

	async logout(): Promise<void> {
		return await logoutAuth();
	},

	async refreshToken(): Promise<boolean> {
		return await refreshTokenAuth();
	}
};

export { ApiError };
