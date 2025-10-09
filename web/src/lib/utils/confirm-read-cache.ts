/**
 * IndexedDB cache for shared secret read confirmations
 * Prevents multiple counter decrements on same-user page reloads
 *
 * Timeout: 15 min (prod) / 3 min (dev)
 */

const DB_NAME = 'hashrand-cache';
const STORE_NAME = 'confirm-read';
const DB_VERSION = 1;

export const CONFIRM_READ_CACHE_TIMEOUT =
	import.meta.env.MODE === 'production'
		? 15 * 60 * 1000 // 15 minutes (production)
		: 3 * 60 * 1000; // 3 minutes (development)

interface ConfirmReadCache {
	hash: string;
	timestamp: number;
}

/**
 * Opens IndexedDB connection, creates object store if needed
 */
async function getDB(): Promise<IDBDatabase> {
	// Check if IndexedDB is available
	if (typeof indexedDB === 'undefined') {
		throw new Error('IndexedDB not available in this browser');
	}

	return new Promise((resolve, reject) => {
		try {
			const request = indexedDB.open(DB_NAME, DB_VERSION);

			request.onerror = () => {
				const error = request.error || new Error('IndexedDB open failed');
				reject(error);
			};

			request.onsuccess = () => resolve(request.result);

			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;
				if (!db.objectStoreNames.contains(STORE_NAME)) {
					db.createObjectStore(STORE_NAME, { keyPath: 'hash' });
				}
			};
		} catch (error) {
			reject(error);
		}
	});
}

/**
 * Retrieves cached confirmation timestamp for a hash
 *
 * @param hash - Shared secret hash (Base58 encoded)
 * @returns Timestamp (ms since epoch) if cached, null otherwise
 */
export async function getCachedConfirmation(hash: string): Promise<number | null> {
	const db = await getDB();
	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readonly');
		const store = tx.objectStore(STORE_NAME);
		const request = store.get(hash);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => {
			const result = request.result as ConfirmReadCache | undefined;
			resolve(result?.timestamp || null);
		};
	});
}

/**
 * Stores confirmation with current timestamp
 *
 * @param hash - Shared secret hash (Base58 encoded)
 */
export async function setCachedConfirmation(hash: string): Promise<void> {
	const db = await getDB();
	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);
		const request = store.put({ hash, timestamp: Date.now() });

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve();
	});
}

/**
 * Removes cached confirmation entry
 *
 * @param hash - Shared secret hash (Base58 encoded)
 */
export async function clearCachedConfirmation(hash: string): Promise<void> {
	const db = await getDB();
	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);
		const request = store.delete(hash);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve();
	});
}
