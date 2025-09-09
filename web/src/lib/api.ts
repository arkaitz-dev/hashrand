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

const API_BASE = '/api';

// Helper function to get authentication headers
async function getAuthHeaders(): Promise<Record<string, string>> {
	const authStore = sessionStorage.getItem('auth_user');
	const accessToken = sessionStorage.getItem('access_token');

	if (!authStore || !accessToken) {
		return {};
	}

	try {
		const user = JSON.parse(authStore);

		// Check if token is expired
		if (user.expiresAt) {
			const expiresAt = new Date(user.expiresAt);
			if (expiresAt <= new Date()) {
				// Token expired, clear storage and return empty headers
				sessionStorage.removeItem('auth_user');
				sessionStorage.removeItem('access_token');
				sessionStorage.removeItem('cipher_token');
				sessionStorage.removeItem('nonce_token');
				return {};
			}
		}

		return {
			Authorization: `Bearer ${accessToken}`
		};
	} catch {
		return {};
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

	// If 401 Unauthorized, try to refresh token
	if (response.status === 401) {
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
			dialogStore.show('auth');
		}
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
	async requestMagicLink(loginRequest: LoginRequest): Promise<MagicLinkResponse> {
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

	async validateMagicLink(magicToken: string, randomHash: string): Promise<LoginResponse> {
		const response = await fetch(
			`${API_BASE}/login/?magiclink=${encodeURIComponent(magicToken)}&hash=${encodeURIComponent(randomHash)}`
		);

		if (!response.ok) {
			const errorData = (await response.json()) as AuthError;
			throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
		}

		// The refresh token will be set as HttpOnly cookie by the server
		return response.json() as Promise<LoginResponse>;
	},

	async checkAuthStatus(): Promise<boolean> {
		// Check if we have both user info and access token in sessionStorage
		const authStore = sessionStorage.getItem('auth_user');
		const accessToken = sessionStorage.getItem('access_token');

		if (!authStore || !accessToken) return false;

		try {
			const user = JSON.parse(authStore);
			if (!user.expiresAt) return false;

			const expiresAt = new Date(user.expiresAt);
			return expiresAt > new Date();
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
			const response = await fetch(`${API_BASE}/refresh`, {
				method: 'POST',
				credentials: 'include' // Include HttpOnly cookies
			});

			if (response.ok) {
				const data = await response.json();

				// Update auth store with new token
				const { authStore } = await import('./stores/auth');
				const expiresAt = new Date();
				expiresAt.setSeconds(expiresAt.getSeconds() + data.expires_in);

				const user = {
					email: '', // Not needed for Zero Knowledge auth
					user_id: data.user_id,
					isAuthenticated: true,
					expiresAt
				};

				// Update store and sessionStorage
				authStore.updateTokens(user, data.access_token);
				
				// Generate crypto tokens if they don't exist (new tab scenario)
				if (!sessionStorage.getItem('cipher_token') || !sessionStorage.getItem('nonce_token')) {
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
