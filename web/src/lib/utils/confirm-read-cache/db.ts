/**
 * IndexedDB database management for confirm-read cache
 * Handles connection, initialization, and corruption repair
 */

import { DB_NAME, DB_VERSION, STORE_NAME } from './types';

/**
 * Opens IndexedDB connection, creates object store if needed
 */
export async function getDB(): Promise<IDBDatabase> {
	// Import logger dynamically to avoid circular dependencies
	const { logger } = await import('../logger');

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
			const db = request.result;

			// Integrity check: verify object store exists (defensive fallback)
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				logger.error('[ConfirmReadCache] ❌ Object store missing - DB corrupted', {
					dbName: DB_NAME,
					expectedStore: STORE_NAME,
					actualStores: Array.from(db.objectStoreNames),
					hint: 'initConfirmReadCache() should have fixed this - possible race condition'
				});
				db.close();
				reject(new Error('Database corrupted - missing object store'));
				return;
			}

			logger.debug('[ConfirmReadCache] IndexedDB opened successfully');
			resolve(db);
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
 * Detects and repairs corrupted DBs (missing object stores)
 */
export async function initConfirmReadCache(): Promise<void> {
	const { logger } = await import('../logger');
	logger.debug('[ConfirmReadCache] Initializing cache database');

	let db: IDBDatabase;

	try {
		db = await getDB();
	} catch (error) {
		// getDB() failed (DB corrupted or missing object store)
		logger.warn('[ConfirmReadCache] ⚠️ Failed to open DB, will recreate', {
			error: error instanceof Error ? error.message : String(error)
		});

		// Delete corrupted/conflicting DB and wait for completion
		await new Promise<void>((resolve, reject) => {
			const deleteRequest = indexedDB.deleteDatabase(DB_NAME);

			deleteRequest.onsuccess = () => {
				logger.info('[ConfirmReadCache] 🗑️ DB deleted successfully');
				resolve();
			};

			deleteRequest.onerror = () => {
				logger.error('[ConfirmReadCache] ❌ Failed to delete DB', {
					error: deleteRequest.error
				});
				reject(deleteRequest.error);
			};

			deleteRequest.onblocked = () => {
				logger.warn('[ConfirmReadCache] ⏳ DB deletion blocked (connections still open)');
			};
		});

		// Reopen - this will trigger onupgradeneeded (DB doesn't exist now)
		logger.debug('[ConfirmReadCache] Reopening DB after deletion');
		const freshDb = await getDB();
		freshDb.close();

		logger.info('[ConfirmReadCache] ✅ Cache database recreated successfully');
		return;
	}

	// DB opened successfully - verify object store exists
	if (!db.objectStoreNames.contains(STORE_NAME)) {
		logger.warn('[ConfirmReadCache] ⚠️ Object store missing - DB corrupted, recreating...', {
			dbName: DB_NAME,
			expectedStore: STORE_NAME,
			actualStores: Array.from(db.objectStoreNames)
		});
		db.close();

		// Delete corrupted DB and wait for completion
		await new Promise<void>((resolve, reject) => {
			const deleteRequest = indexedDB.deleteDatabase(DB_NAME);

			deleteRequest.onsuccess = () => {
				logger.info('[ConfirmReadCache] 🗑️ Corrupted DB deleted successfully');
				resolve();
			};

			deleteRequest.onerror = () => {
				logger.error('[ConfirmReadCache] ❌ Failed to delete corrupted DB', {
					error: deleteRequest.error
				});
				reject(deleteRequest.error);
			};

			deleteRequest.onblocked = () => {
				logger.warn('[ConfirmReadCache] ⏳ DB deletion blocked (connections still open)');
			};
		});

		// Reopen - this will trigger onupgradeneeded (DB doesn't exist now)
		logger.debug('[ConfirmReadCache] Reopening DB after deletion');
		const freshDb = await getDB();
		freshDb.close();

		logger.info('[ConfirmReadCache] ✅ Cache database recreated successfully');
	} else {
		db.close();
		logger.info('[ConfirmReadCache] ✅ Cache database already initialized correctly');
	}
}
