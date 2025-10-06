/**
 * Version Update Store - Reactive version difference detection
 *
 * Detects when API returns different frontend version than cached,
 * triggering UI update notification for seamless frontend upgrades.
 */

import { writable, derived, type Readable } from 'svelte/store';
import { getVersionWithCache, writeCache } from '$lib/version-cache';
import type { VersionResponse } from '$lib/types';

// Session backup interface
interface SessionBackup {
	timestamp: number;
	currentRoute: string;
	value?: unknown;
}

// Current versions from API
const currentVersions = writable<VersionResponse | null>(null);

// Cached versions (what user currently has)
const cachedVersions = writable<VersionResponse | null>(null);

// Update available flag
export const updateAvailable: Readable<boolean> = derived(
	[currentVersions, cachedVersions],
	([current, cached]) => {
		// Only show update if we have both versions and they differ
		if (!current || !cached) return false;

		// Check if frontend version changed
		return current.ui_version !== cached.ui_version;
	}
);

// Current version info (for display)
export const versionInfo: Readable<VersionResponse | null> = derived(
	currentVersions,
	(current) => current
);

/**
 * Initialize version checking - call this on app start
 */
export async function initializeVersionCheck(): Promise<void> {
	try {
		// Get current versions from API/cache
		const versions = await getVersionWithCache();
		currentVersions.set(versions);

		// Set as cached versions initially
		cachedVersions.set(versions);
	} catch (error) {
		console.warn('Failed to initialize version check:', error);
	}
}

/**
 * Check for version updates - call this periodically or on focus
 */
export async function checkForUpdates(): Promise<void> {
	try {
		// Force fresh API call by bypassing cache
		const response = await fetch('/api/version');
		if (!response.ok) return;

		const latestVersions: VersionResponse = await response.json();

		// Update current versions
		currentVersions.set(latestVersions);

		// Note: cachedVersions remains unchanged until user accepts update
	} catch (error) {
		console.warn('Failed to check for updates:', error);
	}
}

/**
 * Mark update as accepted - updates cached version and triggers reload
 */
export async function acceptUpdate(): Promise<void> {
	const current = getVersionInfo();
	if (!current) return;

	try {
		// Backup current state to IndexedDB before reload
		await backupCurrentState();

		// Update cached version to match current
		cachedVersions.set(current);

		// Update the persistent cache with new version
		await writeCache(current);

		// Reload the entire frontend
		window.location.reload();
	} catch (error) {
		console.error('Failed to accept update:', error);
	}
}

/**
 * Backup current application state before reload
 */
async function backupCurrentState(): Promise<void> {
	try {
		// Open IndexedDB for session backup
		const request = indexedDB.open('hashrand-session-backup', 1);

		const db = await new Promise<IDBDatabase>((resolve, reject) => {
			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve(request.result);

			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;
				if (!db.objectStoreNames.contains('session-state')) {
					db.createObjectStore('session-state', { keyPath: 'id' });
				}
			};
		});

		// Backup critical state
		const backupData = {
			id: 'pre-update-backup',
			currentRoute: window.location.pathname + window.location.search,
			timestamp: Date.now()
			// Add any other state that needs to survive reload
		};

		const transaction = db.transaction(['session-state'], 'readwrite');
		const store = transaction.objectStore('session-state');
		store.put(backupData);

		await new Promise<void>((resolve, reject) => {
			transaction.oncomplete = () => resolve();
			transaction.onerror = () => reject(transaction.error);
		});
	} catch (error) {
		console.warn('Failed to backup session state:', error);
		// Non-blocking - continue with reload even if backup fails
	}
}

/**
 * Restore session state after reload (call this on app initialization)
 */
export async function restoreSessionState(): Promise<void> {
	try {
		const request = indexedDB.open('hashrand-session-backup', 1);

		const db = await new Promise<IDBDatabase>((resolve, reject) => {
			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve(request.result);
			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;
				if (!db.objectStoreNames.contains('session-state')) {
					db.createObjectStore('session-state');
				}
			};
		});

		const transaction = db.transaction(['session-state'], 'readonly');
		const store = transaction.objectStore('session-state');
		const getRequest = store.get('pre-update-backup');

		const backup = await new Promise<SessionBackup | undefined>((resolve, reject) => {
			getRequest.onerror = () => reject(getRequest.error);
			getRequest.onsuccess = () => resolve(getRequest.result?.value || undefined);
		});

		if (backup && Date.now() - backup.timestamp < 300000) {
			// 5 minutes max
			// Restore route if different from current
			if (backup.currentRoute !== window.location.pathname + window.location.search) {
				// Navigate to previous route
				window.history.replaceState(null, '', backup.currentRoute);
			}

			// Clean up backup
			const deleteTransaction = db.transaction(['session-state'], 'readwrite');
			const deleteStore = deleteTransaction.objectStore('session-state');
			deleteStore.delete('pre-update-backup');
		}
	} catch (error) {
		console.warn('Failed to restore session state:', error);
		// Non-blocking
	}
}

/**
 * Get current version info (helper)
 */
function getVersionInfo(): VersionResponse | null {
	let current: VersionResponse | null = null;
	currentVersions.subscribe((v) => (current = v))();
	return current;
}
