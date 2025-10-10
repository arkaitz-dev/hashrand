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
 * Includes integrity verification to detect and recover from corrupted databases
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
		try {
			const request = indexedDB.open(DB_NAME, DB_VERSION);

			request.onerror = () => {
				const error = request.error || new Error('IndexedDB open failed');
				logger.error('[ConfirmReadCache] IndexedDB open failed', { error });
				reject(error);
			};

			request.onsuccess = () => {
				const db = request.result;
				logger.debug('[ConfirmReadCache] IndexedDB opened successfully');

				// CRITICAL: Verify DB integrity - check object store exists
				if (!db.objectStoreNames.contains(STORE_NAME)) {
					logger.error('[ConfirmReadCache] 🚨 DB INTEGRITY CHECK FAILED - Object store missing', {
						STORE_NAME,
						DB_VERSION,
						existingStores: Array.from(db.objectStoreNames),
						diagnosis: 'Database corrupted or schema mismatch'
					});

					// Close corrupted DB before deletion
					db.close();

					// Delete corrupted DB to force recreation on next access
					logger.warn('[ConfirmReadCache] Deleting corrupted IndexedDB', { DB_NAME });
					const deleteRequest = indexedDB.deleteDatabase(DB_NAME);

					deleteRequest.onsuccess = () => {
						logger.info(
							'[ConfirmReadCache] ✅ Corrupted DB deleted - will be recreated on next access',
							{
								DB_NAME
							}
						);
					};

					deleteRequest.onerror = () => {
						logger.error('[ConfirmReadCache] Failed to delete corrupted DB', {
							DB_NAME,
							error: deleteRequest.error
						});
					};

					// Reject to signal caller that DB needs retry
					reject(new Error('IndexedDB object store missing - DB deleted and will be recreated'));
					return;
				}

				// DB integrity verified
				logger.debug('[ConfirmReadCache] ✅ DB Integrity Check PASSED', {
					STORE_NAME,
					existingStores: Array.from(db.objectStoreNames)
				});

				resolve(db);
			};

			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;
				logger.debug('[ConfirmReadCache] Upgrading IndexedDB schema', {
					oldVersion: event.oldVersion,
					newVersion: event.newVersion
				});
				if (!db.objectStoreNames.contains(STORE_NAME)) {
					db.createObjectStore(STORE_NAME, { keyPath: 'hash' });
					logger.info('[ConfirmReadCache] ✅ Created object store', { STORE_NAME });
				}
			};
		} catch (error) {
			logger.error('[ConfirmReadCache] Exception opening IndexedDB', { error });
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
	const { logger } = await import('./logger');

	logger.debug('[ConfirmReadCache] getCachedConfirmation() called', {
		hash,
		hashLength: hash.length
	});

	const db = await getDB();

	return new Promise((resolve, reject) => {
		try {
			// Protected: These operations can throw synchronously if object store doesn't exist
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
		} catch (syncError: unknown) {
			// Catch synchronous errors (e.g., object store not found)
			const error = syncError as Error;
			logger.error('[ConfirmReadCache] Synchronous error in getCachedConfirmation', {
				hash,
				errorType: error?.constructor?.name,
				errorMessage: error?.message,
				errorStack: error?.stack
			});
			reject(syncError);
		}
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
		try {
			// Protected: These operations can throw synchronously if object store doesn't exist
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
		} catch (syncError: unknown) {
			// Catch synchronous errors (e.g., object store not found)
			const error = syncError as Error;
			logger.error('[ConfirmReadCache] Synchronous error in setCachedConfirmation', {
				hash,
				timestamp,
				errorType: error?.constructor?.name,
				errorMessage: error?.message,
				errorStack: error?.stack
			});
			reject(syncError);
		}
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
		try {
			// Protected: These operations can throw synchronously if object store doesn't exist
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
		} catch (syncError: unknown) {
			// Catch synchronous errors (e.g., object store not found)
			const error = syncError as Error;
			logger.error('[ConfirmReadCache] Synchronous error in clearCachedConfirmation', {
				hash,
				errorType: error?.constructor?.name,
				errorMessage: error?.message,
				errorStack: error?.stack
			});
			reject(syncError);
		}
	});
}
