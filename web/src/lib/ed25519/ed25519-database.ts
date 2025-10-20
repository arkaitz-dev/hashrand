/**
 * Ed25519 Database Module - IndexedDB Operations
 *
 * Single Responsibility: Handle all IndexedDB storage operations for Ed25519 keys
 * Part of ed25519.ts refactorization to apply SOLID principles
 *
 * ARCHITECTURE CHANGE (v1.9.0):
 * - Now bridges to new WebCrypto keypair storage (hashrand-crypto DB)
 * - Ed25519 and X25519 are stored together in new storage system
 * - Maintains backward compatibility with Ed25519KeyPair interface
 */

import type { Ed25519KeyPair } from './ed25519-types';

// Legacy database configuration (kept for migration/cleanup purposes)
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
 *
 * DEPRECATED (v1.9.0): New keypairs are stored via keypair-storage.ts
 * This function is kept for backward compatibility but is a no-op
 * since keypairs are now generated and stored together in login flow
 */
export async function storeKeyPair(
	keyPair: Ed25519KeyPair,
	keyId: string = 'default'
): Promise<void> {
	// No-op: Keypairs are now stored via keypair-storage.ts during login
	// This function is kept for backward compatibility with existing code
	console.warn(
		'[ed25519-database] storeKeyPair is deprecated. Use keypair-storage.ts for new implementations.'
	);
}

/**
 * Retrieve Ed25519 key pair from IndexedDB
 *
 * UPDATED (v1.9.0): Now retrieves from new WebCrypto storage system
 */
export async function getKeyPair(keyId: string = 'default'): Promise<Ed25519KeyPair | null> {
	// Import new storage functions
	const {
		getEd25519PrivateKey,
		getEd25519PublicKey,
		getEd25519PublicKeyHex
	} = await import('../crypto/keypair-storage');

	try {
		// Get keys from new WebCrypto storage
		const privateKey = await getEd25519PrivateKey();
		const publicKey = await getEd25519PublicKey();
		const publicKeyHex = await getEd25519PublicKeyHex();

		if (!privateKey || !publicKey || !publicKeyHex) {
			return null;
		}

		// Export public key bytes for compatibility
		const publicKeyBuffer = await crypto.subtle.exportKey('raw', publicKey);
		const publicKeyBytes = new Uint8Array(publicKeyBuffer);

		// Return Ed25519KeyPair structure (compatible with signing module)
		return {
			publicKey, // WebCrypto CryptoKey
			privateKey, // WebCrypto CryptoKey (non-extractable)
			publicKeyBytes, // Raw bytes for verification
			privateKeyBytes: undefined, // Not available (non-extractable)
			isNoble: false // WebCrypto, not Noble
		};
	} catch (error) {
		console.error('Failed to retrieve Ed25519 keypair from new storage:', error);
		return null;
	}
}

/**
 * Clear all stored key pairs (for logout/reset)
 *
 * UPDATED (v1.9.0): Clears both legacy and new storage systems
 */
export async function clearAllKeyPairs(): Promise<void> {
	// Clear new WebCrypto storage
	const { clearKeypairs } = await import('../crypto/keypair-storage');
	await clearKeypairs();

	// Also clear legacy storage if it exists (for migration)
	try {
		const db = await openKeyDatabase();
		await new Promise<void>((resolve, reject) => {
			const transaction = db.transaction([STORE_NAME], 'readwrite');
			const store = transaction.objectStore(STORE_NAME);
			const request = store.clear();

			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve();

			transaction.onerror = () => reject(transaction.error);
		});
	} catch (error) {
		// Ignore errors clearing legacy storage (may not exist)
		console.debug('[ed25519-database] Legacy storage clear skipped:', error);
	}
}
