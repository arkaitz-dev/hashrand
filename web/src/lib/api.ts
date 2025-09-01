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
	const authStore = localStorage.getItem('auth_user');
	const accessToken = localStorage.getItem('access_token');

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
				localStorage.removeItem('auth_user');
				localStorage.removeItem('access_token');
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

		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/custom?${searchParams}`, {
			headers: {
				...authHeaders
			}
		});

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

		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/password?${searchParams}`, {
			headers: {
				...authHeaders
			}
		});

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

		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/api-key?${searchParams}`, {
			headers: {
				...authHeaders
			}
		});

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
		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/custom`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...authHeaders
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
		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/password`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...authHeaders
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
		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/api-key`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...authHeaders
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

		const authHeaders = await getAuthHeaders();
		const url = `${API_BASE}/mnemonic${urlParams.toString() ? `?${urlParams}` : ''}`;
		const response = await fetch(url, {
			headers: {
				...authHeaders
			}
		});

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		return response.json() as Promise<CustomHashResponse>;
	},

	async generateMnemonicWithSeed(seedRequest: SeedMnemonicRequest): Promise<CustomHashResponse> {
		const authHeaders = await getAuthHeaders();
		const response = await fetch(`${API_BASE}/mnemonic`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...authHeaders
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

	async validateMagicLink(magicToken: string): Promise<LoginResponse> {
		const response = await fetch(`${API_BASE}/login/?magiclink=${encodeURIComponent(magicToken)}`);

		if (!response.ok) {
			const errorData = (await response.json()) as AuthError;
			throw new ApiError(errorData.error || `HTTP ${response.status}`, response.status);
		}

		// The refresh token will be set as HttpOnly cookie by the server
		return response.json() as Promise<LoginResponse>;
	},

	async checkAuthStatus(): Promise<boolean> {
		// Check if we have both user info and access token in localStorage
		const authStore = localStorage.getItem('auth_user');
		const accessToken = localStorage.getItem('access_token');

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
		// No need to clear HttpOnly refresh token cookie from client-side
		// It will expire automatically (15 minutes Max-Age) and cannot be accessed from JavaScript
		// The main cleanup is done in authStore.logout() (localStorage clearing)
	}
};

export { ApiError };
