/**
 * OTP operations for confirm-read cache
 * Manages cached OTPs for receiver role (sender bypasses OTP)
 */

import { getDB } from './db';
import { STORE_NAME, type ConfirmReadCache } from './types';

/**
 * Retrieves cached OTP for a hash
 *
 * @param hash - Shared secret hash (Base58 encoded)
 * @returns OTP string if cached and valid, null otherwise
 */
export async function getCachedOtp(hash: string): Promise<string | null> {
	const { logger } = await import('../logger');

	logger.debug('[ConfirmReadCache] getCachedOtp() called', { hash, hashLength: hash.length });

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readonly');
		const store = tx.objectStore(STORE_NAME);
		const request = store.get(hash);

		request.onerror = () => {
			logger.error('[ConfirmReadCache] Request error retrieving cached OTP', {
				hash,
				error: request.error
			});
			reject(request.error);
		};

		request.onsuccess = () => {
			const result = request.result as ConfirmReadCache | undefined;
			const otp = result?.otp || null;

			if (otp) {
				const age = Date.now() - (result?.timestamp || 0);
				logger.debug('[ConfirmReadCache] OTP Cache HIT - Found cached OTP', {
					hash,
					age_ms: age,
					age_seconds: Math.round(age / 1000),
					cached_at: new Date(result?.timestamp || 0).toISOString()
				});
			} else {
				logger.debug('[ConfirmReadCache] OTP Cache MISS - No cached OTP found', { hash });
			}

			resolve(otp);
		};
	});
}

/**
 * Stores OTP for a hash with current timestamp
 *
 * @param hash - Shared secret hash (Base58 encoded)
 * @param otp - 9-digit OTP to cache
 */
export async function setCachedOtp(hash: string, otp: string): Promise<void> {
	const { logger } = await import('../logger');
	const timestamp = Date.now();

	logger.debug('[ConfirmReadCache] setCachedOtp() called', {
		hash,
		otp: otp.substring(0, 3) + '***', // Log only first 3 digits for security
		timestamp,
		datetime: new Date(timestamp).toISOString()
	});

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);

		// Get existing entry or create new one
		const getRequest = store.get(hash);

		getRequest.onerror = () => {
			logger.error('[ConfirmReadCache] Request error getting entry for OTP update', {
				hash,
				error: getRequest.error
			});
			reject(getRequest.error);
		};

		getRequest.onsuccess = () => {
			const existing = getRequest.result as ConfirmReadCache | undefined;

			// Merge with existing data (preserve timestamp if exists)
			// IMPORTANT: Use 0 (not current timestamp) if no existing timestamp
			// This allows confirmRead() to execute on first load with OTP
			const data: ConfirmReadCache = {
				hash,
				timestamp: existing?.timestamp || 0,
				otp
			};

			const putRequest = store.put(data);

			putRequest.onerror = () => {
				logger.error('[ConfirmReadCache] Request error storing cached OTP', {
					hash,
					error: putRequest.error
				});
				reject(putRequest.error);
			};

			putRequest.onsuccess = () => {
				logger.info('[ConfirmReadCache] ✅ OTP Cache SAVED successfully', {
					hash,
					otp: otp.substring(0, 3) + '***',
					datetime: new Date(timestamp).toISOString()
				});
				resolve();
			};
		};
	});
}

/**
 * Removes cached OTP entry (keeps timestamp)
 *
 * @param hash - Shared secret hash (Base58 encoded)
 */
export async function clearCachedOtp(hash: string): Promise<void> {
	const { logger } = await import('../logger');

	logger.debug('[ConfirmReadCache] clearCachedOtp() called', { hash });

	const db = await getDB();

	return new Promise((resolve, reject) => {
		const tx = db.transaction(STORE_NAME, 'readwrite');
		const store = tx.objectStore(STORE_NAME);

		// Get existing entry
		const getRequest = store.get(hash);

		getRequest.onerror = () => {
			logger.error('[ConfirmReadCache] Request error getting entry for OTP clearing', {
				hash,
				error: getRequest.error
			});
			reject(getRequest.error);
		};

		getRequest.onsuccess = () => {
			const existing = getRequest.result as ConfirmReadCache | undefined;

			if (!existing) {
				// No entry exists, nothing to clear
				logger.debug('[ConfirmReadCache] No entry to clear OTP from', { hash });
				resolve();
				return;
			}

			// Remove OTP but keep timestamp
			const data: ConfirmReadCache = {
				hash,
				timestamp: existing.timestamp
				// otp intentionally omitted
			};

			const putRequest = store.put(data);

			putRequest.onerror = () => {
				logger.error('[ConfirmReadCache] Request error clearing cached OTP', {
					hash,
					error: putRequest.error
				});
				reject(putRequest.error);
			};

			putRequest.onsuccess = () => {
				logger.info('[ConfirmReadCache] 🗑️ OTP Cache CLEARED successfully', { hash });
				resolve();
			};
		};
	});
}
