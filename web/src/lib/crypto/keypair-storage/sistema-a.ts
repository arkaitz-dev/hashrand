/**
 * Sistema A - Temporary Session Keys Storage Module
 *
 * Manages temporary keypairs used for frontend ↔ backend API communication.
 * These keys are ephemeral, rotate frequently, and are NOT stored in backend database.
 *
 * PURPOSE:
 * - SignedRequest validation (Ed25519 signature verification)
 * - SignedResponse generation (Ed25519 signature)
 * - JWT token validation
 * - Request/response encryption (X25519 ECDH)
 *
 * LIFECYCLE:
 * - Short-lived (regenerated frequently, every request can use new keys)
 * - Stored in IndexedDB (4 CryptoKey objects + 2 hex strings)
 * - Cleared on logout
 *
 * KEYS STORED:
 * - 'ed25519-private' / 'ed25519-public' (CryptoKey, signing)
 * - 'x25519-private' / 'x25519-public' (CryptoKey, encryption)
 * - 'ed25519-public-hex' / 'x25519-public-hex' (string, quick access)
 *
 * SECURITY:
 * - Private keys are non-extractable CryptoKey objects
 * - Backend derives its own keys on-demand (not persistent)
 * - Keys never sent in plaintext (only used for cryptographic operations)
 *
 * @see Sistema B (sistema-b.ts) for permanent user-to-user E2EE keys
 */

import type { KeypairResult } from '../keypair-generation';
import { openDB, STORE_NAME } from './indexeddb';

/**
 * Store Sistema A keypairs in IndexedDB
 *
 * Stores temporary session keys used for API communication.
 * Called after login/key generation to persist keys across page reloads.
 *
 * @param {KeypairResult} keypairs - Generated keypairs from keypair-generation.ts
 * @returns {Promise<void>}
 */
export async function storeKeypairs(keypairs: KeypairResult): Promise<void> {
	const db = await openDB();

	return new Promise((resolve, reject) => {
		const transaction = db.transaction([STORE_NAME], 'readwrite');
		const store = transaction.objectStore(STORE_NAME);

		// Store all 4 CryptoKey objects
		store.put(keypairs.ed25519.privateKey, 'ed25519-private');
		store.put(keypairs.ed25519.publicKey, 'ed25519-public');
		store.put(keypairs.x25519.privateKey, 'x25519-private');
		store.put(keypairs.x25519.publicKey, 'x25519-public');

		// Also store hex public keys for quick access (no WebCrypto export needed)
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
 * Used for signing API requests (SignedRequest).
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
 * Used for signature verification (if needed client-side).
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
 * Used for ECDH key agreement (decrypt backend responses).
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
 * Used for ECDH key agreement (send to backend for encryption).
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
 * Fast access to public key without WebCrypto export.
 * Used for displaying key to user or sending in API requests.
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
 * Fast access to public key without WebCrypto export.
 * Used for displaying key to user or sending in API requests.
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
 * Optimized for scenarios where both keys are needed simultaneously
 * (e.g., sending both to backend in single API request).
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
 * Check if Sistema A keypairs exist in IndexedDB
 *
 * Used to determine if user needs to generate new keys or can use existing ones.
 * Checks all 4 required CryptoKey objects.
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
