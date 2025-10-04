/**
 * Session Module Index - Centralized Exports
 *
 * Provides clean imports for all session management modules
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

// Core database operations
export { sessionDB, type AppSessionData } from './session-db';

// Crypto token management
export {
	getCryptoTokens,
	setCryptoTokens,
	hasCryptoTokens,
	addPrehashSeed,
	getPrehashSeed
} from './session-crypto';

// Authentication data management
export {
	getAuthData,
	setAuthData,
	isAuthenticated,
	getUserEmail,
	clearAuthData,
	setServerPubKey,
	getServerPubKey,
	clearServerPubKey,
	getPrivKey,
	setPrivKey,
	clearPrivKey
} from './session-auth';

// User preferences management
export {
	getUserPreferences,
	setLanguagePreference,
	setThemePreference,
	setUserPreferences
} from './session-preferences';

// Auth flow temporary data
export {
	setPendingAuthEmail,
	getPendingAuthEmail,
	clearPendingAuthEmail
} from './session-auth-flow';
