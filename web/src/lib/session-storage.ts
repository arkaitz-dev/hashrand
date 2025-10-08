/**
 * Session Storage Utility - IndexedDB session expiration management
 *
 * Single Responsibility: Manage session_expires_at timestamp in persistent storage
 * Part of auth system for tracking refresh cookie expiration synchronization
 */

import { logger } from './utils/logger';

const DB_NAME = 'hashrand-session';
const DB_VERSION = 1;
const STORE_NAME = 'session-metadata';
const SESSION_EXPIRES_KEY = 'session_expires_at';

/**
 * Session metadata stored in IndexedDB
 */
interface SessionMetadata {
	key: string;
	expires_at: number;
	updated_at: number;
}

/**
 * Initialize IndexedDB for session storage
 *
 * @returns Promise<IDBDatabase> - Opened database connection
 */
async function openSessionDB(): Promise<IDBDatabase> {
	return new Promise<IDBDatabase>((resolve, reject) => {
		const request = indexedDB.open(DB_NAME, DB_VERSION);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve(request.result);

		request.onupgradeneeded = (event) => {
			const db = (event.target as IDBOpenDBRequest).result;
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				db.createObjectStore(STORE_NAME, { keyPath: 'key' });
			}
		};
	});
}

/**
 * Store session_expires_at timestamp in IndexedDB
 *
 * @param expires_at - Unix timestamp when refresh cookie expires
 * @returns Promise<void>
 */
export async function storeSessionExpiration(expires_at: number): Promise<void> {
	try {
		const db = await openSessionDB();

		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		const metadata: SessionMetadata = {
			key: SESSION_EXPIRES_KEY,
			expires_at,
			updated_at: Date.now()
		};

		store.put(metadata);

		await new Promise<void>((resolve, reject) => {
			transaction.oncomplete = () => resolve();
			transaction.onerror = () => reject(transaction.error);
		});
	} catch (error) {
		logger.warn('Failed to store session expiration:', error);
		// Non-blocking - session continues without persistent expiration tracking
	}
}

/**
 * Retrieve session_expires_at timestamp from IndexedDB
 *
 * @returns Promise<number | null> - Unix timestamp or null if not found/expired
 */
export async function getSessionExpiration(): Promise<number | null> {
	try {
		const db = await openSessionDB();

		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get(SESSION_EXPIRES_KEY);

		const result = await new Promise<SessionMetadata | undefined>((resolve, reject) => {
			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve(request.result);
		});

		if (!result) return null;

		// Check if stored expiration is still valid (not in the past)
		const now = Math.floor(Date.now() / 1000);
		if (result.expires_at <= now) {
			await clearSessionExpiration();
			return null;
		}

		return result.expires_at;
	} catch (error) {
		logger.warn('Failed to retrieve session expiration:', error);
		return null;
	}
}

/**
 * Clear session_expires_at from IndexedDB
 *
 * @returns Promise<void>
 */
export async function clearSessionExpiration(): Promise<void> {
	try {
		const db = await openSessionDB();

		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);
		store.delete(SESSION_EXPIRES_KEY);

		await new Promise<void>((resolve, reject) => {
			transaction.oncomplete = () => resolve();
			transaction.onerror = () => reject(transaction.error);
		});
	} catch (error) {
		logger.warn('Failed to clear session expiration:', error);
		// Non-blocking
	}
}

/**
 * Check if current session has expired based on stored timestamp
 *
 * @returns Promise<boolean> - true if session is expired or no expiration stored
 */
export async function isSessionExpired(): Promise<boolean> {
	const expires_at = await getSessionExpiration();
	if (!expires_at) return true; // No stored expiration = consider expired

	const now = Math.floor(Date.now() / 1000);
	return expires_at <= now;
}
