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
import { handlePostRequest } from './api-helpers';

const API_BASE = '/api';

/**
 * Generic seed-based generation function (DRY implementation)
 */
async function generateWithSeed<T>(
	endpoint: string,
	seedRequest: T,
	// eslint-disable-next-line no-unused-vars
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await handlePostRequest<CustomHashResponse>(
		`${API_BASE}/${endpoint}`,
		seedRequest,
		authenticatedFetch
	);
}

/**
 * Generate custom hash with seed
 */
export async function generateCustomWithSeed(
	seedRequest: SeedGenerateRequest,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateWithSeed('custom', seedRequest, authenticatedFetch);
}

/**
 * Generate password with seed
 */
export async function generatePasswordWithSeed(
	seedRequest: SeedPasswordRequest,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateWithSeed('password', seedRequest, authenticatedFetch);
}

/**
 * Generate API key with seed
 */
export async function generateApiKeyWithSeed(
	seedRequest: SeedApiKeyRequest,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateWithSeed('api-key', seedRequest, authenticatedFetch);
}

/**
 * Generate mnemonic with seed
 */
export async function generateMnemonicWithSeed(
	seedRequest: SeedMnemonicRequest,
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<CustomHashResponse> {
	return await generateWithSeed('mnemonic', seedRequest, authenticatedFetch);
}
