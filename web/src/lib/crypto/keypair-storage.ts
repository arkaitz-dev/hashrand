/**
 * Keypair Storage Module - IndexedDB (Main Orchestrator)
 *
 * Central module for managing both Sistema A and Sistema B cryptographic keys.
 * Provides unified interface for cleanup operations and re-exports specific modules.
 *
 * ARCHITECTURE:
 * - Database: 'hashrand-crypto' (shared by both systems)
 * - Store: 'keypairs' (single store for all keys)
 * - Version: 1
 *
 * DUAL-KEY SYSTEM:
 * - Sistema A: Temporary session keys (API communication)
 * - Sistema B: Permanent user keys (user-to-user E2EE)
 *
 * SECURITY:
 * - Private keys are non-extractable CryptoKey objects
 * - Only accessible via WebCrypto API operations
 * - Automatic cleanup on logout
 *
 * @see keypair-storage/sistema-a.ts for temporary session keys
 * @see keypair-storage/sistema-b.ts for permanent user keys
 * @see keypair-storage/indexeddb.ts for core infrastructure
 */

// Re-export Sistema A functions (temporary session keys)
export {
	storeKeypairs,
	getEd25519PrivateKey,
	getEd25519PublicKey,
	getX25519PrivateKey,
	getX25519PublicKey,
	getEd25519PublicKeyHex,
	getX25519PublicKeyHex,
	getPublicKeyHexStrings,
	keypairsExist
} from './keypair-storage/sistema-a';

// Re-export Sistema B functions (permanent user keys)
export {
	storeDerivedUserKeys,
	getDerivedEd25519PrivateKey,
	getDerivedX25519PrivateKey,
	getDerivedPublicKeyHexStrings
} from './keypair-storage/sistema-b';

// Re-export infrastructure
export { DB_NAME, DB_VERSION, STORE_NAME, openDB } from './keypair-storage/indexeddb';

// Import infrastructure for cleanup operations
import { openDB, STORE_NAME, DB_NAME } from './keypair-storage/indexeddb';

/**
 * Clear all keypairs from IndexedDB (logout cleanup)
 *
 * Removes ALL keys from both Sistema A and Sistema B.
 * Called during logout to ensure no cryptographic material persists.
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
 * Completely removes the database including all Sistema A and Sistema B keys.
 * Use this for complete application reset or uninstallation cleanup.
 *
 * WARNING: This action cannot be undone. All cryptographic keys will be lost.
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
