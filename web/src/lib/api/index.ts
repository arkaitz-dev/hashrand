/**
 * API Module Index - Centralized Exports
 *
 * Provides clean imports for all API modules
 * Part of api.ts refactorization to apply DRY principles
 */

// Helper utilities
export { ApiError, handleJsonResponse } from './api-helpers';

// Generation endpoints
export { generate, generatePassword, generateApiKey, generateMnemonic } from './api-generators';

// Seed-based operations
export {
	generateCustomWithSeed,
	generatePasswordWithSeed,
	generateApiKeyWithSeed,
	generateMnemonicWithSeed
} from './api-seed-operations';

// Authentication operations
export {
	requestMagicLink,
	validateMagicLink,
	checkAuthStatus,
	logout,
	refreshToken
} from './api-auth-operations';
