/**
 * Ed25519 Database Module - IndexedDB Operations
 *
 * Single Responsibility: Handle all IndexedDB storage operations for Ed25519 keys
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

import type { Ed25519KeyPair } from './ed25519-types';

// Database configuration for IndexedDB
const DB_NAME = 'hashrand-ed25519';
const DB_VERSION = 1;
const STORE_NAME = 'keypairs';

/**
 * Open IndexedDB for storing Ed25519 keys
 */
export async function openKeyDatabase(): Promise<IDBDatabase> {
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
 * Store Ed25519 key pair in IndexedDB
 */
export async function storeKeyPair(
	keyPair: Ed25519KeyPair,
	keyId: string = 'default'
): Promise<void> {
	const db = await openKeyDatabase();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		const keyData = {
			id: keyId,
			publicKey: keyPair.publicKey,
			privateKey: keyPair.privateKey,
			publicKeyBytes: Array.from(keyPair.publicKeyBytes), // Convert to plain array for storage
			privateKeyBytes: keyPair.privateKeyBytes ? Array.from(keyPair.privateKeyBytes) : undefined, // Store Noble private key if exists
			isNoble: keyPair.isNoble || false,
			created: Date.now()
		};

		const request = store.put(keyData);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve();

		transaction.onerror = () => reject(transaction.error);
	});
}

/**
 * Retrieve Ed25519 key pair from IndexedDB
 */
export async function getKeyPair(keyId: string = 'default'): Promise<Ed25519KeyPair | null> {
	const db = await openKeyDatabase();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get(keyId);

		request.onerror = () => reject(request.error);
		request.onsuccess = () => {
			const result = request.result;
			if (!result) {
				resolve(null);
				return;
			}

			resolve({
				publicKey: result.publicKey,
				privateKey: result.privateKey,
				publicKeyBytes: new Uint8Array(result.publicKeyBytes),
				privateKeyBytes: result.privateKeyBytes
					? new Uint8Array(result.privateKeyBytes)
					: undefined,
				isNoble: result.isNoble || false
			});
		};

		transaction.onerror = () => reject(transaction.error);
	});
}

/**
 * Clear all stored key pairs (for logout/reset)
 */
export async function clearAllKeyPairs(): Promise<void> {
	const db = await openKeyDatabase();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.clear();

		request.onerror = () => reject(request.error);
		request.onsuccess = () => resolve();

		transaction.onerror = () => reject(transaction.error);
	});
}
