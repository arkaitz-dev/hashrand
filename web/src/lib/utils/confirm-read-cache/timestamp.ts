/**
 * Timestamp operations for confirm-read cache
 * Manages confirmation timestamps to prevent duplicate counter decrements
 */

import { getDB } from './db';
import { STORE_NAME, type ConfirmReadCache } from './types';

/**
 * Retrieves cached confirmation timestamp for a hash
 *
 * @param hash - Shared secret hash (Base58 encoded)
 * @returns Timestamp (ms since epoch) if cached, null otherwise
 */
export async function getCachedConfirmation(hash: string): Promise<number | null> {
	const { logger } = await import('../logger');

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
	const { logger } = await import('../logger');
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
	const { logger } = await import('../logger');

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
