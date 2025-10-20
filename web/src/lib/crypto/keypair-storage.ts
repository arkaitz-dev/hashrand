/**
 * Keypair Storage Module - IndexedDB
 *
 * Stores non-extractable CryptoKey objects in IndexedDB with versioned schema.
 * Replaces session storage (which can't store CryptoKey objects).
 *
 * ARCHITECTURE:
 * - Database: 'hashrand-crypto'
 * - Store: 'keypairs'
 * - Version: 1
 * - Keys: 'ed25519-private', 'ed25519-public', 'x25519-private', 'x25519-public'
 *
 * SECURITY:
 * - Private keys are non-extractable CryptoKey objects
 * - Only accessible via WebCrypto API operations
 * - Automatic cleanup on logout
 */

import type { KeypairResult } from './keypair-generation';

const DB_NAME = 'hashrand-crypto';
const DB_VERSION = 1;
const STORE_NAME = 'keypairs';

/**
 * Open IndexedDB connection with proper error handling
 *
 * @returns {Promise<IDBDatabase>} Database connection
 */
function openDB(): Promise<IDBDatabase> {
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

/**
 * Store keypairs in IndexedDB
 *
 * @param {KeypairResult} keypairs - Generated keypairs
 * @returns {Promise<void>}
 */
export async function storeKeypairs(keypairs: KeypairResult): Promise<void> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		// Store all 4 keys
		store.put(keypairs.ed25519.privateKey, 'ed25519-private');
		store.put(keypairs.ed25519.publicKey, 'ed25519-public');
		store.put(keypairs.x25519.privateKey, 'x25519-private');
		store.put(keypairs.x25519.publicKey, 'x25519-public');

		// Also store hex public keys for quick access
		store.put(keypairs.ed25519.publicKeyHex, 'ed25519-public-hex');
		store.put(keypairs.x25519.publicKeyHex, 'x25519-public-hex');

		transaction.oncomplete = () => {
			db.close();
			resolve();
		};

		transaction.onerror = () => {
			db.close();
			reject(new Error(`Failed to store keypairs: ${transaction.error?.message}`));
		};
	});
}

/**
 * Retrieve Ed25519 private key from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} Ed25519 private key or null if not found
 */
export async function getEd25519PrivateKey(): Promise<CryptoKey | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('ed25519-private');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve Ed25519 private key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve Ed25519 public key from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} Ed25519 public key or null if not found
 */
export async function getEd25519PublicKey(): Promise<CryptoKey | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('ed25519-public');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve Ed25519 public key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve X25519 private key from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} X25519 private key or null if not found
 */
export async function getX25519PrivateKey(): Promise<CryptoKey | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('x25519-private');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve X25519 private key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve X25519 public key from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} X25519 public key or null if not found
 */
export async function getX25519PublicKey(): Promise<CryptoKey | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('x25519-public');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve X25519 public key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve Ed25519 public key hex from IndexedDB
 *
 * @returns {Promise<string | null>} Ed25519 public key hex or null if not found
 */
export async function getEd25519PublicKeyHex(): Promise<string | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('ed25519-public-hex');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve Ed25519 public key hex: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve X25519 public key hex from IndexedDB
 *
 * @returns {Promise<string | null>} X25519 public key hex or null if not found
 */
export async function getX25519PublicKeyHex(): Promise<string | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('x25519-public-hex');

		request.onsuccess = () => {
			db.close();
			resolve(request.result || null);
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve X25519 public key hex: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve both public key hex strings (for quick access)
 *
 * @returns {Promise<{ed25519: string, x25519: string} | null>} Public key hex strings or null
 */
export async function getPublicKeyHexStrings(): Promise<{
	ed25519: string;
	x25519: string;
} | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);

		const ed25519Request = store.get('ed25519-public-hex');
		const x25519Request = store.get('x25519-public-hex');

		let ed25519Hex: string | null = null;
		let x25519Hex: string | null = null;

		ed25519Request.onsuccess = () => {
			ed25519Hex = ed25519Request.result || null;
		};

		x25519Request.onsuccess = () => {
			x25519Hex = x25519Request.result || null;
		};

		transaction.oncomplete = () => {
			db.close();
			if (ed25519Hex && x25519Hex) {
				resolve({ ed25519: ed25519Hex, x25519: x25519Hex });
			} else {
				resolve(null);
			}
		};

		transaction.onerror = () => {
			db.close();
			reject(new Error(`Failed to retrieve public key hex strings: ${transaction.error?.message}`));
		};
	});
}

/**
 * Check if keypairs exist in IndexedDB
 *
 * @returns {Promise<boolean>} True if all 4 keys exist
 */
export async function keypairsExist(): Promise<boolean> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);

		const keys = [
			'ed25519-private',
			'ed25519-public',
			'x25519-private',
			'x25519-public'
		] as const;

		let allExist = true;
		let completedChecks = 0;

		keys.forEach((key) => {
			const request = store.get(key);

			request.onsuccess = () => {
				if (!request.result) {
					allExist = false;
				}
				completedChecks++;

				if (completedChecks === keys.length) {
					db.close();
					resolve(allExist);
				}
			};

			request.onerror = () => {
				db.close();
				reject(new Error(`Failed to check keypair existence: ${request.error?.message}`));
			};
		});
	});
}

/**
 * Clear all keypairs from IndexedDB (logout cleanup)
 *
 * @returns {Promise<void>}
 */
export async function clearKeypairs(): Promise<void> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		const request = store.clear();

		request.onsuccess = () => {
			db.close();
			resolve();
		};

		request.onerror = () => {
			db.close();
			reject(new Error(`Failed to clear keypairs: ${request.error?.message}`));
		};
	});
}

/**
 * Delete entire IndexedDB database (for complete cleanup)
 *
 * @returns {Promise<void>}
 */
export async function deleteDatabase(): Promise<void> {
	return new Promise((resolve, reject) => {
		const request = indexedDB.deleteDatabase(DB_NAME);

		request.onsuccess = () => {
			resolve();
		};

		request.onerror = () => {
			reject(new Error(`Failed to delete database: ${request.error?.message}`));
		};

		request.onblocked = () => {
			reject(new Error('Database deletion blocked (close all tabs)'));
		};
	});
}
