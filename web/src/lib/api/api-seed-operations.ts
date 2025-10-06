/**
 * API Seed Operations Module - Seed-based Generation Endpoints
 *
 * Single Responsibility: Handle all POST endpoints with seed parameters using DRY principles
 * Part of api.ts refactorization to eliminate code duplication
 */

import type {
	SeedGenerateRequest,
	SeedPasswordRequest,
	SeedApiKeyRequest,
	SeedMnemonicRequest,
	CustomHashResponse
} from '../types';
const API_BASE = '/api';

/**
 * Generic seed-based generation function using universal authenticated signed POST request
 */
async function generateWithSeed<T extends object>(
	endpoint: string,
	seedRequest: T
): Promise<CustomHashResponse> {
	const { httpAuthenticatedSignedPOSTRequest } = await import('../httpSignedRequests');
	const { alphabetToInt, mnemonicLangToInt } = await import('../types');

	// Convert enums to integers (CRITICAL: must match backend)
	const convertedRequest = { ...seedRequest } as Record<string, unknown>;

	// Convert alphabet to integer if present
	if ('alphabet' in seedRequest && typeof seedRequest.alphabet === 'string') {
		const alphabetInt = alphabetToInt(
			seedRequest.alphabet as import('../types').AlphabetTypeString
		);
		convertedRequest.alphabet = alphabetInt;
	}

	// Convert language to integer if present
	if ('language' in seedRequest && typeof seedRequest.language === 'string') {
		const langInt = mnemonicLangToInt(
			seedRequest.language as import('../types').MnemonicLanguageString
		);
		convertedRequest.language = langInt;
	}

	return await httpAuthenticatedSignedPOSTRequest<T, CustomHashResponse>(
		`${API_BASE}/${endpoint}`,
		convertedRequest as T
	);
}

/**
 * Generate custom hash with seed
 */
export async function generateCustomWithSeed(
	seedRequest: SeedGenerateRequest
): Promise<CustomHashResponse> {
	return await generateWithSeed('custom', seedRequest);
}

/**
 * Generate password with seed
 */
export async function generatePasswordWithSeed(
	seedRequest: SeedPasswordRequest
): Promise<CustomHashResponse> {
	return await generateWithSeed('password', seedRequest);
}

/**
 * Generate API key with seed
 */
export async function generateApiKeyWithSeed(
	seedRequest: SeedApiKeyRequest
): Promise<CustomHashResponse> {
	return await generateWithSeed('api-key', seedRequest);
}

/**
 * Generate mnemonic with seed
 */
export async function generateMnemonicWithSeed(
	seedRequest: SeedMnemonicRequest
): Promise<CustomHashResponse> {
	return await generateWithSeed('mnemonic', seedRequest);
}
