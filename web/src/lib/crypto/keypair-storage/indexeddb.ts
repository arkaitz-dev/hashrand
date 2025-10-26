/**
 * IndexedDB Infrastructure Module
 *
 * Provides core IndexedDB connection and configuration for keypair storage.
 * Shared by System A (temporary session keys) and System B (permanent user keys).
 *
 * ARCHITECTURE:
 * - Database: 'hashrand-crypto'
 * - Store: 'keypairs'
 * - Version: 1
 *
 * SECURITY:
 * - Stores non-extractable CryptoKey objects
 * - Only accessible via WebCrypto API operations
 * - Automatic cleanup on logout
 */

export const DB_NAME = 'hashrand-crypto';
export const DB_VERSION = 1;
export const STORE_NAME = 'keypairs';

/**
 * Open IndexedDB connection with proper error handling
 *
 * Creates object store on first run (onupgradeneeded).
 * Shared by both System A and System B operations.
 *
 * @returns {Promise<IDBDatabase>} Database connection
 */
export function openDB(): Promise<IDBDatabase> {
	return new Promise((resolve, reject) => {
		const request = indexedDB.open(DB_NAME, DB_VERSION);

		request.onerror = () => {
			reject(new Error(`Failed to open IndexedDB: ${request.error?.message}`));
		};

		request.onsuccess = () => {
			resolve(request.result);
		};

		request.onupgradeneeded = (event) => {
			const db = (event.target as IDBOpenDBRequest).result;

			// Create object store if it doesn't exist
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				db.createObjectStore(STORE_NAME);
			}
		};
	});
}
