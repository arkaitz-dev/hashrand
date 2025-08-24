import type {
	GenerateParams,
	PasswordParams,
	ApiKeyParams,
	VersionResponse,
	HashResponse,
	CustomHashResponse,
	SeedGenerateRequest,
	SeedPasswordRequest,
	SeedApiKeyRequest
} from './types';

const API_BASE = '/api';

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

// Handle both JSON and text responses automatically
async function handleResponse(
	response: Response
): Promise<string | HashResponse | CustomHashResponse> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}

	// Check content type to determine response format
	const contentType = response.headers.get('content-type');
	if (contentType?.includes('application/json')) {
		// Parse JSON and check if it's a CustomHashResponse (has otp field)
		const jsonResponse = await response.json();
		return jsonResponse as HashResponse | CustomHashResponse;
	} else {
		return response.text();
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
	async generate(params: GenerateParams): Promise<string | HashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.prefix) searchParams.set('prefix', params.prefix);
		if (params.suffix) searchParams.set('suffix', params.suffix);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await fetch(`${API_BASE}/custom?${searchParams}`);
		return handleResponse(response);
	},

	async generatePassword(params: PasswordParams): Promise<string | HashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await fetch(`${API_BASE}/password?${searchParams}`);
		return handleResponse(response);
	},

	async generateApiKey(params: ApiKeyParams): Promise<string | HashResponse> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await fetch(`${API_BASE}/api-key?${searchParams}`);
		return handleResponse(response);
	},

	async getVersion(): Promise<VersionResponse> {
		const response = await fetch(`${API_BASE}/version`);
		return handleJsonResponse<VersionResponse>(response);
	},

	async generateWithSeed(seedRequest: SeedGenerateRequest): Promise<HashResponse> {
		const response = await fetch(`${API_BASE}/custom`, {
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

		return response.json() as Promise<HashResponse>;
	},

	async generatePasswordWithSeed(seedRequest: SeedPasswordRequest): Promise<HashResponse> {
		const response = await fetch(`${API_BASE}/password`, {
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

		return response.json() as Promise<HashResponse>;
	},

	async generateApiKeyWithSeed(seedRequest: SeedApiKeyRequest): Promise<HashResponse> {
		const response = await fetch(`${API_BASE}/api-key`, {
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

		return response.json() as Promise<HashResponse>;
	}
};

export { ApiError };
