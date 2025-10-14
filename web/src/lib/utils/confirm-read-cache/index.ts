/**
 * Confirm-read cache module
 * Public API for IndexedDB-based caching of shared secret confirmations and OTPs
 */

// Re-export types and constants
export { CONFIRM_READ_CACHE_TIMEOUT, type ConfirmReadCache } from './types';

// Re-export database initialization
export { initConfirmReadCache } from './db';

// Re-export timestamp operations
export { getCachedConfirmation, setCachedConfirmation, clearCachedConfirmation } from './timestamp';

// Re-export OTP operations
export { getCachedOtp, setCachedOtp, clearCachedOtp } from './otp';
