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
import { handleGetRequest } from './api-helpers';

const API_BASE = '/api';

/**
 * Generic generation function (DRY implementation)
 */
async function generateHash(
	endpoint: string,
	params: GenerateParams | PasswordParams | ApiKeyParams | MnemonicParams,
	// eslint-disable-next-line no-unused-vars
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await handleGetRequest<CustomHashResponse>(
		`${API_BASE}/${endpoint}`,
		params as Record<string, unknown>,
		authenticatedFetch
	);
}

/**
 * Generate custom hash
 */
export async function generate(
	params: GenerateParams,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateHash('custom', params, authenticatedFetch);
}

/**
 * Generate password
 */
export async function generatePassword(
	params: PasswordParams,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateHash('password', params, authenticatedFetch);
}

/**
 * Generate API key
 */
export async function generateApiKey(
	params: ApiKeyParams,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateHash('api-key', params, authenticatedFetch);
}

/**
 * Generate mnemonic
 */
export async function generateMnemonic(
	params: MnemonicParams = {},
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateHash('mnemonic', params, authenticatedFetch);
}
