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
	// Import logger dynamically to avoid circular dependencies
	const { logger } = await import('./logger');

	// Check if IndexedDB is available
	if (typeof indexedDB === 'undefined') {
		logger.error('[ConfirmReadCache] IndexedDB not available in this browser');
		throw new Error('IndexedDB not available in this browser');
	}

	logger.debug('[ConfirmReadCache] Opening IndexedDB', { DB_NAME, DB_VERSION });

	return new Promise((resolve, reject) => {
		const request = indexedDB.open(DB_NAME, DB_VERSION);

		request.onerror = () => {
			const error = request.error || new Error('IndexedDB open failed');
			logger.error('[ConfirmReadCache] IndexedDB open failed', { error });
			reject(error);
		};

		request.onsuccess = () => {
			logger.debug('[ConfirmReadCache] IndexedDB opened successfully');
			resolve(request.result);
		};

		request.onupgradeneeded = (event) => {
			const db = (event.target as IDBOpenDBRequest).result;
			logger.info('[ConfirmReadCache] Upgrading IndexedDB schema', {
				oldVersion: event.oldVersion,
				newVersion: event.newVersion
			});
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				db.createObjectStore(STORE_NAME, { keyPath: 'hash' });
				logger.info('[ConfirmReadCache] ✅ Created object store', { STORE_NAME });
			}
		};
	});
}

/**
 * Initialize confirm-read cache database
 * Called during login to ensure DB is ready before first use
 * Triggers onupgradeneeded if DB doesn't exist
 */
export async function initConfirmReadCache(): Promise<void> {
	const { logger } = await import('./logger');
	logger.debug('[ConfirmReadCache] Initializing cache database');

	const db = await getDB();
	db.close(); // Close immediately - just needed to trigger onupgradeneeded

	logger.info('[ConfirmReadCache] ✅ Cache database initialized');
}

/**
 * Retrieves cached confirmation timestamp for a hash
 *
 * @param hash - Shared secret hash (Base58 encoded)
 * @returns Timestamp (ms since epoch) if cached, null otherwise
 */
export async function getCachedConfirmation(hash: string): Promise<number | null> {
	const { logger } = await import('./logger');

	logger.debug('[ConfirmReadCache] getCachedConfirmation() called', {
		hash,
		hashLength: hash.length
	});

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readonly');
		const store = tx.objectStore(STORE_NAME);
		const request = store.get(hash);

		request.onerror = () => {
			logger.error('[ConfirmReadCache] Request error retrieving cached confirmation', {
				hash,
				error: request.error
			});
			reject(request.error);
		};

		request.onsuccess = () => {
			const result = request.result as ConfirmReadCache | undefined;
			const timestamp = result?.timestamp || null;

			if (timestamp) {
				const age = Date.now() - timestamp;
				logger.debug('[ConfirmReadCache] Cache HIT - Found cached confirmation', {
					hash,
					timestamp,
					age_ms: age,
					age_seconds: Math.round(age / 1000),
					cached_at: new Date(timestamp).toISOString()
				});
			} else {
				logger.debug('[ConfirmReadCache] Cache MISS - No cached confirmation found', {
					hash
				});
			}

			resolve(timestamp);
		};
	});
}

/**
 * Stores confirmation with current timestamp
 *
 * @param hash - Shared secret hash (Base58 encoded)
 */
export async function setCachedConfirmation(hash: string): Promise<void> {
	const { logger } = await import('./logger');
	const timestamp = Date.now();

	logger.debug('[ConfirmReadCache] setCachedConfirmation() called', {
		hash,
		timestamp,
		datetime: new Date(timestamp).toISOString()
	});

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);
		const request = store.put({ hash, timestamp });

		request.onerror = () => {
			logger.error('[ConfirmReadCache] Request error storing cached confirmation', {
				hash,
				timestamp,
				error: request.error
			});
			reject(request.error);
		};

		request.onsuccess = () => {
			logger.info('[ConfirmReadCache] ✅ Cache SAVED successfully', {
				hash,
				timestamp,
				datetime: new Date(timestamp).toISOString()
			});
			resolve();
		};
	});
}

/**
 * Removes cached confirmation entry
 *
 * @param hash - Shared secret hash (Base58 encoded)
 */
export async function clearCachedConfirmation(hash: string): Promise<void> {
	const { logger } = await import('./logger');

	logger.debug('[ConfirmReadCache] clearCachedConfirmation() called', { hash });

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);
		const request = store.delete(hash);

		request.onerror = () => {
			logger.error('[ConfirmReadCache] Request error clearing cached confirmation', {
				hash,
				error: request.error
			});
			reject(request.error);
		};

		request.onsuccess = () => {
			logger.info('[ConfirmReadCache] 🗑️ Cache CLEARED successfully', { hash });
			resolve();
		};
	});
}
