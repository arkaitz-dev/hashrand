/**
 * Types and constants for IndexedDB confirm-read cache
 * Prevents multiple counter decrements on same-user page reloads
 *
 * Timeout: 15 min (prod) / 3 min (dev)
 */

export const DB_NAME = 'hashrand-confirm-read-cache';
export const STORE_NAME = 'confirm-read';
export const DB_VERSION = 1;

export const CONFIRM_READ_CACHE_TIMEOUT =
	import.meta.env.MODE === 'production'
		? 15 * 60 * 1000 // 15 minutes (production)
		: 3 * 60 * 1000; // 3 minutes (development)

export interface ConfirmReadCache {
	hash: string;
	timestamp: number;
	otp?: string; // Optional OTP for receiver role (sender doesn't need OTP)
}
