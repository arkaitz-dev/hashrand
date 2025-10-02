/**
 * API Auth Operations - Authentication Endpoints
 *
 * Refactored with SRP: Separated login, refresh, and utility operations
 */

// Re-export all public API
export { requestMagicLink, validateMagicLink, logout, checkAuthStatus } from './login';
export { refreshToken } from './refresh';
