/**
 * API Generators Module - Generation Endpoints
 *
 * Single Responsibility: Handle all generation GET endpoints with DRY principles
 * Part of api.ts refactorization to eliminate code duplication
 */

import type {
	GenerateParams,
	PasswordParams,
	ApiKeyParams,
	MnemonicParams,
	CustomHashResponse
} from '../types';
const API_BASE = '/api';

/**
 * Generic generation function using universal authenticated signed GET request
 */
async function generateHash(
	endpoint: string,
	params: GenerateParams | PasswordParams | ApiKeyParams | MnemonicParams
): Promise<CustomHashResponse> {
	const { httpAuthenticatedSignedGETRequest } = await import('../httpSignedRequests');

	// Convert parameters to string format for signing
	const stringParams: Record<string, string> = {};
	Object.entries(params).forEach(([key, value]) => {
		if (value !== undefined && value !== null) {
			if (typeof value === 'boolean') {
				stringParams[key] = value.toString();
			} else if (typeof value === 'number') {
				stringParams[key] = value.toString();
			} else if (typeof value === 'string') {
				stringParams[key] = value;
			}
		}
	});

	return await httpAuthenticatedSignedGETRequest<CustomHashResponse>(
		`${API_BASE}/${endpoint}`,
		stringParams
	);
}

/**
 * Generate custom hash
 */
export async function generate(params: GenerateParams): Promise<CustomHashResponse> {
	return await generateHash('custom', params);
}

/**
 * Generate password
 */
export async function generatePassword(params: PasswordParams): Promise<CustomHashResponse> {
	return await generateHash('password', params);
}

/**
 * Generate API key
 */
export async function generateApiKey(params: ApiKeyParams): Promise<CustomHashResponse> {
	return await generateHash('api-key', params);
}

/**
 * Generate mnemonic
 */
export async function generateMnemonic(params: MnemonicParams = {}): Promise<CustomHashResponse> {
	return await generateHash('mnemonic', params);
}
