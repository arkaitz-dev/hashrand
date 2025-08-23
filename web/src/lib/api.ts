import type { GenerateParams, PasswordParams, ApiKeyParams, VersionResponse } from './types';

const API_BASE = '/api';

class ApiError extends Error {
	constructor(
		message: string,
		public readonly status: number
	) {
		super(message);
		this.name = 'ApiError';
	}
}

async function handleResponse(response: Response): Promise<string> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}
	return response.text();
}

async function handleJsonResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}
	return response.json();
}

export const api = {
	async generate(params: GenerateParams): Promise<string> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.prefix) searchParams.set('prefix', params.prefix);
		if (params.suffix) searchParams.set('suffix', params.suffix);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await fetch(`${API_BASE}/generate?${searchParams}`);
		return handleResponse(response);
	},

	async generatePassword(params: PasswordParams): Promise<string> {
		const searchParams = new URLSearchParams();

		if (params.length !== undefined) searchParams.set('length', params.length.toString());
		if (params.alphabet) searchParams.set('alphabet', params.alphabet);
		if (params.raw) searchParams.set('raw', 'true');

		const response = await fetch(`${API_BASE}/password?${searchParams}`);
		return handleResponse(response);
	},

	async generateApiKey(params: ApiKeyParams): Promise<string> {
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
	}
};

export { ApiError };
