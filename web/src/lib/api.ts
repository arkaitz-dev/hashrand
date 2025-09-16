import type {
	GenerateParams,
	PasswordParams,
	ApiKeyParams,
	MnemonicParams,
	VersionResponse,
	CustomHashResponse,
	SeedGenerateRequest,
	SeedPasswordRequest,
	SeedApiKeyRequest,
	SeedMnemonicRequest,
	LoginRequest,
	LoginResponse,
	MagicLinkResponse,
	AuthError
} from './types';
import { getOrCreateKeyPair, signMessage, publicKeyToHex } from './ed25519';

const API_BASE = '/api';

// Helper function to check if crypto tokens exist in IndexedDB (DRY)
async function hasCryptoTokens(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('./session-manager');
		return await sessionManager.hasCryptoTokens();
	} catch {
		return false;
	}
}

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
 * Check if a 401 response indicates dual token expiry (both access and refresh tokens expired)
 * @param response - HTTP 401 response to check
 * @returns Promise<boolean> - true if both tokens have expired
 */
async function isDualTokenExpiry(response: Response): Promise<boolean> {
	if (response.status !== 401) return false;

	try {
		// Clone response to read body without consuming it
		const responseClone = response.clone();
		const errorData = await responseClone.json();

		// Check for dual expiry message from backend
		return !!(
			errorData.error && errorData.error.includes('Both access and refresh tokens have expired')
		);
	} catch {
		// If parsing fails, it's not a dual expiry response
		return false;
	}
}

/**
 * Handle dual token expiry scenario
 * Clears all authentication data and shows login dialog
 */
async function handleDualTokenExpiry(): Promise<void> {
	console.log('🔄 DUAL EXPIRY detected - clearing all auth data and requesting new login');

	const { authStore } = await import('./stores/auth');
	const { dialogStore } = await import('./stores/dialog');

	// Complete logout with ALL IndexedDB cleanup
	await authStore.logout();

	// Clear all crypto tokens and auth data (defensive security)
	await authStore.clearPreventiveAuthData();

	// Show auth dialog to request new email authentication
	dialogStore.show('auth');
}

/**
 * Handle proactive token renewal from response headers
 * @param response - HTTP response that may contain renewal tokens
 */
async function handleProactiveTokenRenewal(response: Response): Promise<void> {
	const newAccessToken = response.headers.get('x-new-access-token');
	const newExpiresIn = response.headers.get('x-token-expires-in');

	if (newAccessToken && newExpiresIn) {
		console.log('🔄 Proactive token renewal detected, updating tokens...');

		// Update access token in IndexedDB
		try {
			const { sessionManager } = await import('./session-manager');
			const authData = await sessionManager.getAuthData();
			if (authData.user) {
				await sessionManager.setAuthData(authData.user, newAccessToken);
			}
		} catch (error) {
			console.warn('Failed to update access token during proactive renewal:', error);
		}

		// NOTE: Crypto tokens are NOT regenerated during proactive renewal
		// They remain stable throughout the session for URL parameter encryption consistency

		console.log('✅ Proactive token renewal completed transparently');
	}
}

/**
 * Authenticated fetch wrapper with automatic token refresh
 * @param url - Request URL
 * @param options - Fetch options
 * @returns Promise<Response>
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

	// If 401 Unauthorized, check for dual token expiry first
	if (response.status === 401) {
		// Check if this is a dual token expiry case
		const isDualExpiry = await isDualTokenExpiry(response);

		if (isDualExpiry) {
			console.log('🔄 DUAL EXPIRY detected - both tokens expired, skipping refresh attempt');
			await handleDualTokenExpiry();
			return response; // Return original 401 response for caller handling
		}

		console.log('🔄 Access token expired, attempting refresh...');

		const refreshSuccess = await api.refreshToken();

		if (refreshSuccess) {
			console.log('✅ Token refresh successful, retrying request...');

			// Get updated auth headers and retry request
			const newAuthHeaders = await getAuthHeaders();
			response = await fetch(url, {
				...options,
				headers: {
					...options.headers,
					...newAuthHeaders
				}
			});
		} else {
			console.log('❌ Token refresh failed, forcing logout...');

			// Refresh failed, force logout and show login dialog
			const { authStore } = await import('./stores/auth');
			const { dialogStore } = await import('./stores/dialog');

			await authStore.logout();

			// Clear any residual auth data before asking for email (defensive security)
			authStore.clearPreventiveAuthData();

			dialogStore.show('auth');
		}
	}

	// Check for proactive token renewal headers in successful responses
	if (response.ok) {
		await handleProactiveTokenRenewal(response);
	}

	return response;
}

class ApiError extends Error {
	constructor(
		message: string,
		public readonly status: number // eslint-disable-line no-unused-vars
	) {
		super(message);
		this.name = 'ApiError';
	}

	get statusCode(): number {
		return this.status;
	}
}

async function handleJsonResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}
	return response.json();
}

export const api = {
	async generate(params: GenerateParams): Promise<CustomHashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.prefix) searchParams.set('prefix', params.prefix);
		if (params.suffix) searchParams.set('suffix', params.suffix);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await authenticatedFetch(`${API_BASE}/custom?${searchParams}`);

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generatePassword(params: PasswordParams): Promise<CustomHashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await authenticatedFetch(`${API_BASE}/password?${searchParams}`);

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generateApiKey(params: ApiKeyParams): Promise<CustomHashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await authenticatedFetch(`${API_BASE}/api-key?${searchParams}`);

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async getVersion(): Promise<VersionResponse> {
		const response = await fetch(`${API_BASE}/version`);
		return handleJsonResponse<VersionResponse>(response);
	},

	async generateWithSeed(seedRequest: SeedGenerateRequest): Promise<CustomHashResponse> {
		const response = await authenticatedFetch(`${API_BASE}/custom`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(seedRequest)
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generatePasswordWithSeed(seedRequest: SeedPasswordRequest): Promise<CustomHashResponse> {
		const response = await authenticatedFetch(`${API_BASE}/password`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(seedRequest)
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generateApiKeyWithSeed(seedRequest: SeedApiKeyRequest): Promise<CustomHashResponse> {
		const response = await authenticatedFetch(`${API_BASE}/api-key`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(seedRequest)
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generateMnemonic(params: MnemonicParams = {}): Promise<CustomHashResponse> {
		const urlParams = new URLSearchParams();
		if (params.language) urlParams.set('language', params.language);
		if (params.words) urlParams.set('words', params.words.toString());
		if (params.raw !== undefined) urlParams.set('raw', params.raw.toString());

		const url = `${API_BASE}/mnemonic${urlParams.toString() ? `?${urlParams}` : ''}`;
		const response = await authenticatedFetch(url);

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generateMnemonicWithSeed(seedRequest: SeedMnemonicRequest): Promise<CustomHashResponse> {
		const response = await authenticatedFetch(`${API_BASE}/mnemonic`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(seedRequest)
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	// Authentication endpoints
	async requestMagicLink(
		email: string,
		ui_host: string,
		next?: string
	): Promise<MagicLinkResponse> {
		// Generate or retrieve Ed25519 keypair
		const keyPair = await getOrCreateKeyPair();
		const pubKeyHex = publicKeyToHex(keyPair.publicKeyBytes);

		// Create message to sign: email + pub_key
		const message = email + pubKeyHex;
		const signature = await signMessage(message, keyPair.privateKey);

		const loginRequest: LoginRequest = {
			email,
			ui_host,
			next,
			pub_key: pubKeyHex,
			signature
		};

		const response = await fetch(`${API_BASE}/login/`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(loginRequest)
		});

		if (!response.ok) {
			const errorData = (await response.json()) as AuthError;
			throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<MagicLinkResponse>;
	},

	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		// Generate or retrieve Ed25519 keypair
		const keyPair = await getOrCreateKeyPair();

		// Sign the magic link token itself for verification
		const signature = await signMessage(magicToken, keyPair.privateKey);

		// Create request body with magic link and signature
		const validationRequest = {
			magiclink: magicToken,
			signature
		};

		const response = await fetch(`${API_BASE}/login/magiclink/`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(validationRequest)
		});

		if (!response.ok) {
			const errorData = (await response.json()) as AuthError;
			throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
		}

		// The refresh token will be set as HttpOnly cookie by the server
		return response.json() as Promise<LoginResponse>;
	},

	async checkAuthStatus(): Promise<boolean> {
		// Check if we have both user info and access token in IndexedDB
		try {
			const { sessionManager } = await import('./session-manager');
			const authData = await sessionManager.getAuthData();

			if (!authData.user || !authData.access_token) return false;

			return authData.user.isAuthenticated && !!authData.user.user_id;
		} catch {
			return false;
		}
	},

	async logout(): Promise<void> {
		// Call backend to clear HttpOnly refresh token cookie
		try {
			await fetch(`${API_BASE}/login`, {
				method: 'DELETE'
			});
		} catch (error) {
			console.warn('Failed to clear refresh token cookie:', error);
			// Continue with logout even if cookie clearing fails
		}
	},

	/**
	 * Try to refresh the access token using the HttpOnly refresh token cookie
	 * @returns Promise<boolean> - true if refresh was successful
	 */
	async refreshToken(): Promise<boolean> {
		try {
			console.log('🔄 Frontend: Attempting token refresh...');
			const response = await fetch(`${API_BASE}/refresh`, {
				method: 'POST',
				credentials: 'include' // Include HttpOnly cookies
			});

			console.log('📡 Frontend: Refresh response status:', response.status);

			// Check for dual token expiry in refresh response
			if (response.status === 401) {
				const isDualExpiry = await isDualTokenExpiry(response);
				if (isDualExpiry) {
					console.log('🔄 DUAL EXPIRY detected during refresh - both tokens expired');
					await handleDualTokenExpiry();
					return false;
				}
			}

			if (response.ok) {
				const data = await response.json();
				console.log('✅ Frontend: Refresh successful, new expires_in:', data.expires_in);

				// Update auth store with new token
				const { authStore } = await import('./stores/auth');

				const user = {
					user_id: data.user_id,
					isAuthenticated: true
				};

				// Update store and IndexedDB
				authStore.updateTokens(user, data.access_token);

				// Generate crypto tokens if they don't exist (new tab scenario)
				if (!(await hasCryptoTokens())) {
					authStore.generateCryptoTokens();
				}

				return true;
			}
			return false;
		} catch (error) {
			console.warn('Token refresh failed:', error);
			return false;
		}
	}
};

export { ApiError };
