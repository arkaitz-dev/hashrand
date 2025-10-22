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
import { logger } from '$lib/utils/logger';

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

/**
 * Store derived user keypairs in IndexedDB
 *
 * Stores deterministic keypairs derived from privkey_context.
 * SECURITY: Only stores non-extractable CryptoKey objects, NOT raw bytes.
 *
 * @param derivedKeys - Derived user keypairs from user-key-derivation.ts
 * @returns {Promise<void>}
 */
export async function storeDerivedUserKeys(derivedKeys: {
	ed25519: {
		// ❌ NO privateKeyBytes
		publicKeyBytes: Uint8Array;
		publicKeyHex: string;
		privateKey: CryptoKey; // Non-extractable
		publicKey: CryptoKey;
	};
	x25519: {
		// ❌ NO privateKeyBytes
		publicKeyBytes: Uint8Array;
		publicKeyHex: string;
		privateKey: CryptoKey; // Non-extractable
		publicKey: CryptoKey;
	};
}): Promise<void> {
	logger.debug('[storeDerivedUserKeys] Storing derived user keys in IndexedDB');

	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		logger.debug('[storeDerivedUserKeys] Storing Ed25519 keys:', {
			privateKey_extractable: derivedKeys.ed25519.privateKey.extractable,
			publicKey_extractable: derivedKeys.ed25519.publicKey.extractable
		});

		// Store Ed25519 CryptoKeys (non-extractable)
		store.put(derivedKeys.ed25519.privateKey, 'user-ed25519-private');
		store.put(derivedKeys.ed25519.publicKey, 'user-ed25519-public');
		store.put(derivedKeys.ed25519.publicKeyHex, 'user-ed25519-public-hex');
		store.put(derivedKeys.ed25519.publicKeyBytes, 'user-ed25519-public-bytes');

		logger.debug('[storeDerivedUserKeys] Storing X25519 keys:', {
			privateKey_extractable: derivedKeys.x25519.privateKey.extractable,
			publicKey_extractable: derivedKeys.x25519.publicKey.extractable
		});

		// Store X25519 CryptoKeys (non-extractable)
		store.put(derivedKeys.x25519.privateKey, 'user-x25519-private');
		store.put(derivedKeys.x25519.publicKey, 'user-x25519-public');
		store.put(derivedKeys.x25519.publicKeyHex, 'user-x25519-public-hex');
		store.put(derivedKeys.x25519.publicKeyBytes, 'user-x25519-public-bytes');

		// ❌ Private key bytes NOT stored (security)
		logger.debug('[storeDerivedUserKeys] ⚠️  Private key BYTES not stored (security)');

		transaction.oncomplete = () => {
			db.close();
			logger.info('[storeDerivedUserKeys] ✅ Derived user keys stored successfully');
			resolve();
		};

		transaction.onerror = () => {
			db.close();
			logger.error('[storeDerivedUserKeys] ❌ Failed to store derived user keys:', transaction.error);
			reject(new Error(`Failed to store derived user keys: ${transaction.error?.message}`));
		};
	});
}

/**
 * Retrieve derived user Ed25519 private key (CryptoKey) from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} Ed25519 private key or null if not found
 */
export async function getDerivedEd25519PrivateKey(): Promise<CryptoKey | null> {
	logger.debug('[getDerivedEd25519PrivateKey] Retrieving user Ed25519 private key from IndexedDB');

	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('user-ed25519-private');

		request.onsuccess = () => {
			db.close();
			const key = request.result || null;
			if (key) {
				logger.debug('[getDerivedEd25519PrivateKey] ✅ Retrieved Ed25519 private key:', {
					type: key.type,
					extractable: key.extractable,
					usages: key.usages
				});
			} else {
				logger.warn('[getDerivedEd25519PrivateKey] ⚠️  Ed25519 private key not found');
			}
			resolve(key);
		};

		request.onerror = () => {
			db.close();
			logger.error(
				'[getDerivedEd25519PrivateKey] ❌ Failed to retrieve Ed25519 private key:',
				request.error
			);
			reject(new Error(`Failed to retrieve Ed25519 private key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve derived user X25519 private key (CryptoKey) from IndexedDB
 *
 * @returns {Promise<CryptoKey | null>} X25519 private key or null if not found
 */
export async function getDerivedX25519PrivateKey(): Promise<CryptoKey | null> {
	logger.debug('[getDerivedX25519PrivateKey] Retrieving user X25519 private key from IndexedDB');

	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);
		const request = store.get('user-x25519-private');

		request.onsuccess = () => {
			db.close();
			const key = request.result || null;
			if (key) {
				logger.debug('[getDerivedX25519PrivateKey] ✅ Retrieved X25519 private key:', {
					type: key.type,
					extractable: key.extractable,
					usages: key.usages
				});
			} else {
				logger.warn('[getDerivedX25519PrivateKey] ⚠️  X25519 private key not found');
			}
			resolve(key);
		};

		request.onerror = () => {
			db.close();
			logger.error(
				'[getDerivedX25519PrivateKey] ❌ Failed to retrieve X25519 private key:',
				request.error
			);
			reject(new Error(`Failed to retrieve X25519 private key: ${request.error?.message}`));
		};
	});
}

/**
 * Retrieve derived user public key hex strings from IndexedDB
 *
 * @returns {Promise<{ed25519: string, x25519: string} | null>} Public key hex strings or null
 */
export async function getDerivedPublicKeyHexStrings(): Promise<{
	ed25519: string;
	x25519: string;
} | null> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readonly');
		const store = transaction.objectStore(STORE_NAME);

		const ed25519Request = store.get('user-ed25519-public-hex');
		const x25519Request = store.get('user-x25519-public-hex');

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
			reject(
				new Error(`Failed to retrieve derived public key hex strings: ${transaction.error?.message}`)
			);
		};
	});
}
