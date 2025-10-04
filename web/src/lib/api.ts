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
	MagicLinkResponse,
	CreateSharedSecretRequest,
	CreateSharedSecretResponse,
	ViewSharedSecretRequest,
	ViewSharedSecretResponse
} from './types';

import {
	ApiError,
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
	refreshToken as refreshTokenAuth,
	createSharedSecret as createSharedSecretAPI,
	viewSharedSecret as viewSharedSecretAPI,
	deleteSharedSecret as deleteSharedSecretAPI
} from './api/index';

const API_BASE = '/api';

// NOTE: Removed authenticatedFetch function as all authentication is now handled by universal signed request functions

/**
 * Main API object with simplified interface
 */
export const api = {
	// Generation endpoints (GET)
	async generate(params: GenerateParams): Promise<CustomHashResponse> {
		return await generateHash(params);
	},

	async generatePassword(params: PasswordParams): Promise<CustomHashResponse> {
		return await generatePasswordHash(params);
	},

	async generateApiKey(params: ApiKeyParams): Promise<CustomHashResponse> {
		return await generateApiKeyHash(params);
	},

	async generateMnemonic(params: MnemonicParams = {}): Promise<CustomHashResponse> {
		return await generateMnemonicHash(params);
	},

	// Version endpoint (public) - NO SignedResponse expected
	async getVersion(): Promise<VersionResponse> {
		const response = await fetch(`${API_BASE}/version`);

		if (!response.ok) {
			const errorText = await response.text();
			throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
		}

		// Direct JSON parsing - this endpoint does NOT use SignedResponse
		return response.json();
	},

	// Seed-based endpoints (POST)
	async generateWithSeed(seedRequest: SeedGenerateRequest): Promise<CustomHashResponse> {
		return await generateCustomWithSeed(seedRequest);
	},

	async generatePasswordWithSeed(seedRequest: SeedPasswordRequest): Promise<CustomHashResponse> {
		return await generatePasswordWithSeed(seedRequest);
	},

	async generateApiKeyWithSeed(seedRequest: SeedApiKeyRequest): Promise<CustomHashResponse> {
		return await generateApiKeyWithSeed(seedRequest);
	},

	async generateMnemonicWithSeed(seedRequest: SeedMnemonicRequest): Promise<CustomHashResponse> {
		return await generateMnemonicWithSeed(seedRequest);
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
	},

	// Shared Secret endpoints
	async createSharedSecret(
		request: CreateSharedSecretRequest
	): Promise<CreateSharedSecretResponse> {
		return await createSharedSecretAPI(request);
	},

	async viewSharedSecret(
		hash: string,
		otpRequest?: ViewSharedSecretRequest
	): Promise<ViewSharedSecretResponse> {
		return await viewSharedSecretAPI(hash, otpRequest);
	},

	async deleteSharedSecret(hash: string): Promise<void> {
		return await deleteSharedSecretAPI(hash);
	}
};

export { ApiError };
