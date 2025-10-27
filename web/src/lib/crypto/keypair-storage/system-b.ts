/**
 * System B - Permanent User Keys Storage Module
 *
 * Manages permanent keypairs used for user-to-user end-to-end encryption (E2EE).
 * These keys are deterministic, derived from privkey_context, and enable Perfect Forward Secrecy.
 *
 * PURPOSE:
 * - User-to-user message encryption (future feature)
 * - User-to-user file sharing (future feature)
 * - Public key publication via `/api/keys/rotate`
 * - Public key retrieval via `/api/user/keys/?target_user=...`
 *
 * LIFECYCLE:
 * - Long-lived (deterministic, permanent)
 * - Derived from `blake3_kdf(email, "Ed25519" + base58(privkey_context))`
 * - IDENTICAL on every login (same email + privkey_context)
 * - Public keys stored in backend database (NOT private keys)
 *
 * KEYS STORED:
 * - 'user-ed25519-private' / 'user-ed25519-public' (CryptoKey, signing)
 * - 'user-x25519-private' / 'user-x25519-public' (CryptoKey, encryption)
 * - 'user-ed25519-public-hex' / 'user-x25519-public-hex' (string, quick access)
 * - 'user-ed25519-public-bytes' / 'user-x25519-public-bytes' (Uint8Array, raw bytes)
 *
 * SECURITY:
 * - Private keys are non-extractable CryptoKey objects
 * - Backend stores PUBLIC keys ONLY in database tables
 * - Backend CANNOT derive these keys (requires plaintext email, has only user_id hash)
 * - Zero Knowledge architecture (server never sees private keys)
 *
 * @see System A (system-a.ts) for temporary session keys
 * @see user-key-derivation.ts for key derivation logic
 */

import { logger } from '$lib/utils/logger';
import { openDB, STORE_NAME } from './indexeddb';

/**
 * Store derived user keypairs in IndexedDB
 *
 * Stores deterministic keypairs derived from privkey_context.
 * Called after magic link validation and key derivation.
 *
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
			logger.error(
				'[storeDerivedUserKeys] ❌ Failed to store derived user keys:',
				transaction.error
			);
			reject(new Error(`Failed to store derived user keys: ${transaction.error?.message}`));
		};
	});
}

/**
 * Retrieve derived user Ed25519 private key (CryptoKey) from IndexedDB
 *
 * Used for signing user-to-user messages (future feature).
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
 * Used for ECDH key agreement with other users (future feature).
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
 * Fast access to public keys without WebCrypto export.
 * Used for publishing keys to backend via `/api/keys/rotate`.
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
				new Error(
					`Failed to retrieve derived public key hex strings: ${transaction.error?.message}`
				)
			);
		};
	});
}
