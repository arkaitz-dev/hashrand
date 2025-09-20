/**
 * Ed25519 Digital Signature Module for Magic Link Authentication
 *
 * Provides secure Ed25519 keypair generation, storage, and signing functionality
 * using Web Crypto API with non-extractable keys stored in IndexedDB.
 */

import { ed25519 } from '@noble/curves/ed25519';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils';

// Database configuration for IndexedDB
const DB_NAME = 'hashrand-ed25519';
const DB_VERSION = 1;
const STORE_NAME = 'keypairs';

/**
 * Ed25519 key pair interface
 */
export interface Ed25519KeyPair {
	publicKey: CryptoKey | null; // null when using Noble fallback
	privateKey: CryptoKey | null; // null when using Noble fallback
	publicKeyBytes: Uint8Array; // 32 bytes for serialization
	privateKeyBytes?: Uint8Array; // 32 bytes, only for Noble fallback
	isNoble?: boolean; // true when using Noble curves fallback
}

/**
 * Open IndexedDB for storing Ed25519 keys
 */
async function openKeyDatabase(): Promise<IDBDatabase> {
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
 * Generate Ed25519 key pair using Web Crypto API
 * Private key is non-extractable for security
 */
export async function generateEd25519KeyPair(): Promise<Ed25519KeyPair> {
	// Check if Web Crypto API supports Ed25519
	if (!('subtle' in crypto)) {
		throw new Error('Web Crypto API not available');
	}

	try {
		// Generate key pair using Web Crypto API (non-extractable private key)
		const keyPair = await crypto.subtle.generateKey(
			{
				name: 'Ed25519',
				namedCurve: 'Ed25519'
			},
			false, // extractable: false - Private key cannot be extracted as raw bytes
			['sign', 'verify']
		);

		// Export public key for serialization/transmission
		const publicKeyRaw = await crypto.subtle.exportKey('raw', keyPair.publicKey);
		const publicKeyBytes = new Uint8Array(publicKeyRaw);

		if (publicKeyBytes.length !== 32) {
			throw new Error(`Invalid Ed25519 public key length: ${publicKeyBytes.length}, expected 32`);
		}

		return {
			publicKey: keyPair.publicKey,
			privateKey: keyPair.privateKey,
			publicKeyBytes
		};
	} catch {
		// Fallback to Noble curves if WebCrypto Ed25519 not supported
		return generateEd25519KeyPairFallback();
	}
}

/**
 * Fallback Ed25519 key generation using Noble curves
 * Used when WebCrypto doesn't support Ed25519
 */
async function generateEd25519KeyPairFallback(): Promise<Ed25519KeyPair> {
	// Using Noble curves fallback (WebCrypto not supported)

	// Generate random private key (32 bytes)
	const privateKeyBytes = crypto.getRandomValues(new Uint8Array(32));
	const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);

	// Return Noble-based keypair (no WebCrypto dependency)
	return {
		publicKey: null, // Not using WebCrypto
		privateKey: null, // Not using WebCrypto
		publicKeyBytes: new Uint8Array(publicKeyBytes),
		privateKeyBytes: privateKeyBytes,
		isNoble: true
	};
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
 * Sign message using Ed25519 private key
 * @param message - Message to sign (string or bytes)
 * @param privateKey - Ed25519 private key (CryptoKey)
 * @returns Signature as hex string (128 hex chars = 64 bytes)
 */
export async function signMessage(
	message: string | Uint8Array,
	keyPair: Ed25519KeyPair
): Promise<string> {
	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	if (keyPair.isNoble && keyPair.privateKeyBytes) {
		// Use Noble curves for signing
		// Signing with Noble curves
		const signature = ed25519.sign(new Uint8Array(messageBytes), keyPair.privateKeyBytes);
		return bytesToHex(signature);
	} else if (keyPair.privateKey) {
		// Use WebCrypto for signing
		// Signing with WebCrypto
		try {
			const signature = await crypto.subtle.sign(
				'Ed25519',
				keyPair.privateKey,
				new Uint8Array(messageBytes)
			);
			return bytesToHex(new Uint8Array(signature));
		} catch (error) {
			throw new Error(`WebCrypto signing failed: ${error}`);
		}
	} else {
		throw new Error('No valid private key available for signing');
	}
}

/**
 * Verify Ed25519 signature
 * @param message - Original message (string or bytes)
 * @param signature - Signature as hex string
 * @param publicKeyBytes - Ed25519 public key (32 bytes)
 * @returns True if signature is valid
 */
export async function verifySignature(
	message: string | Uint8Array,
	signature: string,
	publicKeyBytes: Uint8Array
): Promise<boolean> {
	if (publicKeyBytes.length !== 32) {
		throw new Error(`Invalid Ed25519 public key length: ${publicKeyBytes.length}, expected 32`);
	}

	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	const signatureBytes = hexToBytes(signature);

	if (signatureBytes.length !== 64) {
		throw new Error(`Invalid Ed25519 signature length: ${signatureBytes.length}, expected 64`);
	}

	try {
		// Try WebCrypto verification first
		const publicKey = await crypto.subtle.importKey(
			'raw',
			new Uint8Array(publicKeyBytes),
			{ name: 'Ed25519', namedCurve: 'Ed25519' },
			false,
			['verify']
		);

		return await crypto.subtle.verify(
			'Ed25519',
			publicKey,
			new Uint8Array(signatureBytes),
			new Uint8Array(messageBytes)
		);
	} catch {
		// Fallback to Noble curves
		return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
	}
}

/**
 * Get or create Ed25519 key pair for current session
 * Returns existing key pair from IndexedDB or creates new one
 */
export async function getOrCreateKeyPair(): Promise<Ed25519KeyPair> {
	// Try to get existing key pair
	const existingKeyPair = await getKeyPair();
	if (existingKeyPair) {
		return existingKeyPair;
	}

	// Generate new key pair if none exists
	const newKeyPair = await generateEd25519KeyPair();
	await storeKeyPair(newKeyPair);

	// Generated new Ed25519 key pair for magic link authentication
	return newKeyPair;
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

/**
 * Export public key as hex string for transmission
 */
export function publicKeyToHex(publicKeyBytes: Uint8Array): string {
	return bytesToHex(publicKeyBytes);
}

/**
 * Import public key from hex string
 */
export function publicKeyFromHex(hex: string): Uint8Array {
	return hexToBytes(hex);
}
