/**
 * Version Cache Service - IndexedDB persistent cache for API version information
 *
 * Implements 24-hour cache with intelligent update logic to minimize /api/version calls.
 * Falls back to direct fetch if IndexedDB is unavailable or fails.
 */

import type { VersionResponse } from './types';
import { logger } from './utils/logger';

const DB_NAME = 'hashrand-cache';
const DB_VERSION = 1;
const STORE_NAME = 'versions';
const CACHE_KEY = 'current-versions';
const CACHE_DURATION = 24 * 60 * 60 * 1000; // 24 hours in milliseconds

interface VersionCache {
	id: string;
	api_version: string;
	ui_version: string;
	expires_at: number;
}

/**
 * Opens IndexedDB connection and ensures object store exists
 */
function openDB(): Promise<IDBDatabase> {
	return new Promise((resolve, reject) => {
		const request = indexedDB.open(DB_NAME, DB_VERSION);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve(request.result);

		request.onupgradeneeded = (event) => {
			const db = (event.target as IDBOpenDBRequest).result;
			if (!db.objectStoreNames.contains(STORE_NAME)) {
				db.createObjectStore(STORE_NAME, { keyPath: 'id' });
			}
		};
	});
}

/**
 * Reads version cache from IndexedDB
 */
async function readCache(): Promise<VersionCache | null> {
	try {
		const db = await openDB();

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([STORE_NAME], 'readonly');
			const store = transaction.objectStore(STORE_NAME);
			const request = store.get(CACHE_KEY);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve(request.result || null);
		});
	} catch (error) {
		logger.warn('Failed to read version cache from IndexedDB:', error);
		return null;
	}
}

/**
 * Writes version cache to IndexedDB
 */
export async function writeCache(versions: VersionResponse): Promise<void> {
	try {
		const db = await openDB();
		const cache: VersionCache = {
			id: CACHE_KEY,
			api_version: versions.api_version,
			ui_version: versions.ui_version,
			expires_at: Date.now() + CACHE_DURATION
		};

		return new Promise((resolve, reject) => {
			const transaction = db.transaction([STORE_NAME], 'readwrite');
			const store = transaction.objectStore(STORE_NAME);
			const request = store.put(cache);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve();
		});
	} catch (error) {
		logger.warn('Failed to write version cache to IndexedDB:', error);
		// Non-blocking - cache write failure shouldn't break the app
	}
}

/**
 * Fetches version from API
 */
async function fetchVersion(): Promise<VersionResponse> {
	const response = await fetch('/api/version');

	if (!response.ok) {
		const errorText = await response.text();
		throw new Error(errorText || `HTTP ${response.status}`);
	}

	return response.json();
}

/**
 * Gets version information with intelligent caching
 *
 * Logic:
 * 1. Try to read from IndexedDB cache
 * 2. If cache exists and not expired, return cached version
 * 3. If cache expired or doesn't exist, fetch from API
 * 4. Cache new version in IndexedDB
 * 5. Return version information
 *
 * Falls back to direct API call if IndexedDB operations fail.
 */
export async function getVersionWithCache(): Promise<VersionResponse> {
	// Try to get cached version
	const cached = await readCache();

	// Check if cache is valid (exists and not expired)
	if (cached && Date.now() <= cached.expires_at) {
		return {
			api_version: cached.api_version,
			ui_version: cached.ui_version
		};
	}

	// Cache expired or doesn't exist - fetch from API
	const versions = await fetchVersion();

	// Cache the new version (non-blocking)
	await writeCache(versions);

	return versions;
}
