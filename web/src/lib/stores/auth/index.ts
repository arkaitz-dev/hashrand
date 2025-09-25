/**
 * Auth Module Index - Centralized Exports
 *
 * Provides clean imports for all authentication modules
 * Part of auth.ts refactorization to apply SOLID principles
 */

// Storage operations
export { loadAuthFromStorage, saveAuthToStorage, clearAuthFromStorage } from './auth-storage';

// Crypto token management
export { generateCryptoTokens, hasCryptoTokens, hasValidRefreshCookie } from './auth-crypto-tokens';

// Data cleanup operations
export {
	clearPreventiveAuthData,
	clearSensitiveAuthData,
	clearSensitiveAuthDataWithMessage
} from './auth-cleanup';

// Session validation
export { checkSessionValidity, hasLocalAuthTokens } from './auth-session';

// Authentication actions
export { requestMagicLink, validateMagicLink, logout } from './auth-actions';
