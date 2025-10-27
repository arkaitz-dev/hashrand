/**
 * Keypair Generation Module - WebCrypto API
 *
 * Generates independent Ed25519 and X25519 keypairs using native WebCrypto
 * with non-extractable private keys for maximum security.
 *
 * ARCHITECTURE:
 * - Ed25519: For signing (signatures validation)
 * - X25519: For ECDH E2E encryption (shared secrets)
 * - Private keys: Non-extractable CryptoKey objects (stored in IndexedDB)
 * - Public keys: Extractable as raw bytes for transmission to backend
 *
 * BROWSER SUPPORT:
 * - Chrome 111+ (Ed25519 support)
 * - Firefox 119+ (Ed25519 support)
 * - Safari 16.4+ (Ed25519 support)
 *
 * NO FALLBACK: Old browsers will receive clear error message
 */

/**
 * Keypair structure with CryptoKey objects
 */
export interface KeypairResult {
	ed25519: {
		privateKey: CryptoKey; // Non-extractable
		publicKey: CryptoKey; // Extractable (for raw bytes)
		publicKeyHex: string; // 64 hex chars (32 bytes)
	};
	x25519: {
		privateKey: CryptoKey; // Non-extractable
		publicKey: CryptoKey; // Extractable (for raw bytes)
		publicKeyHex: string; // 64 hex chars (32 bytes)
	};
}

/**
 * Generate independent Ed25519 and X25519 keypairs
 *
 * @returns {Promise<KeypairResult>} Keypair with CryptoKey objects and hex public keys
 * @throws {Error} If browser doesn't support WebCrypto Ed25519/X25519
 */
export async function generateKeypairs(): Promise<KeypairResult> {
	// Check browser support
	if (!crypto.subtle) {
		throw new Error(
			'WebCrypto API not available. Please use a modern browser (Chrome 111+, Firefox 119+, Safari 16.4+)'
		);
	}

	try {
		// Generate Ed25519 keypair (for signing)
		const ed25519Keypair = await crypto.subtle.generateKey(
			{
				name: 'Ed25519',
				namedCurve: 'Ed25519'
			},
			false, // privateKey non-extractable
			['sign', 'verify']
		);

		// Generate X25519 keypair (for ECDH)
		// FIX: X25519 requires algorithm name string 'X25519', NOT {name:'ECDH', namedCurve}
		// Must match format used in importKey() and deriveBits() for compatibility
		/* eslint-disable no-undef */
		const x25519Keypair = (await crypto.subtle.generateKey(
			'X25519', // ✅ String format (matches importKey and deriveBits)
			false, // privateKey non-extractable
			['deriveKey', 'deriveBits']
		)) as CryptoKeyPair;
		/* eslint-enable no-undef */

		// Extract public keys as raw bytes
		const ed25519PublicBytes = await crypto.subtle.exportKey('raw', ed25519Keypair.publicKey);
		const x25519PublicBytes = await crypto.subtle.exportKey('raw', x25519Keypair.publicKey);

		// Convert to hex strings
		const ed25519PublicKeyHex = bytesToHex(new Uint8Array(ed25519PublicBytes));
		const x25519PublicKeyHex = bytesToHex(new Uint8Array(x25519PublicBytes));

		// Validate key lengths
		if (ed25519PublicKeyHex.length !== 64) {
			throw new Error(
				`Invalid Ed25519 public key length: ${ed25519PublicKeyHex.length} (expected 64)`
			);
		}
		if (x25519PublicKeyHex.length !== 64) {
			throw new Error(
				`Invalid X25519 public key length: ${x25519PublicKeyHex.length} (expected 64)`
			);
		}

		return {
			ed25519: {
				privateKey: ed25519Keypair.privateKey,
				publicKey: ed25519Keypair.publicKey,
				publicKeyHex: ed25519PublicKeyHex
			},
			x25519: {
				privateKey: x25519Keypair.privateKey,
				publicKey: x25519Keypair.publicKey,
				publicKeyHex: x25519PublicKeyHex
			}
		};
	} catch (error) {
		// Check for specific browser support errors
		// eslint-disable-next-line no-undef
		if (error instanceof DOMException && error.name === 'NotSupportedError') {
			throw new Error(
				'Ed25519/X25519 not supported in this browser. Please update to Chrome 111+, Firefox 119+, or Safari 16.4+'
			);
		}
		throw error;
	}
}

/**
 * Convert Uint8Array to hex string
 *
 * @param {Uint8Array} bytes - Raw bytes
 * @returns {string} Hex string (lowercase)
 */
function bytesToHex(bytes: Uint8Array): string {
	return Array.from(bytes)
		.map((b) => b.toString(16).padStart(2, '0'))
		.join('');
}

/**
 * Convert hex string to Uint8Array
 *
 * @param {string} hex - Hex string
 * @returns {Uint8Array} Raw bytes
 */
export function hexToBytes(hex: string): Uint8Array {
	if (hex.length % 2 !== 0) {
		throw new Error('Invalid hex string length');
	}
	const bytes = new Uint8Array(hex.length / 2);
	for (let i = 0; i < hex.length; i += 2) {
		bytes[i / 2] = parseInt(hex.slice(i, i + 2), 16);
	}
	return bytes;
}

/**
 * Import Ed25519 public key from hex string
 *
 * @param {string} publicKeyHex - 64 hex chars (32 bytes)
 * @returns {Promise<CryptoKey>} Ed25519 public key CryptoKey
 */
export async function importEd25519PublicKey(publicKeyHex: string): Promise<CryptoKey> {
	if (publicKeyHex.length !== 64) {
		throw new Error(`Invalid Ed25519 public key hex length: ${publicKeyHex.length} (expected 64)`);
	}

	const publicKeyBytes = hexToBytes(publicKeyHex);

	return await crypto.subtle.importKey(
		'raw',
		publicKeyBytes.buffer as ArrayBuffer,
		{
			name: 'Ed25519',
			namedCurve: 'Ed25519'
		},
		true, // extractable
		['verify']
	);
}

/**
 * Import X25519 public key from hex string
 *
 * @param {string} publicKeyHex - 64 hex chars (32 bytes)
 * @returns {Promise<CryptoKey>} X25519 public key CryptoKey
 */
export async function importX25519PublicKey(publicKeyHex: string): Promise<CryptoKey> {
	// Import logger for diagnostic logging
	const { logger } = await import('$lib/utils/logger');

	logger.debug('[importX25519PublicKey] Input validation:', {
		hex: publicKeyHex,
		length: publicKeyHex?.length,
		type: typeof publicKeyHex,
		firstBytes: publicKeyHex?.substring(0, 16)
	});

	if (publicKeyHex.length !== 64) {
		throw new Error(`Invalid X25519 public key hex length: ${publicKeyHex.length} (expected 64)`);
	}

	const publicKeyBytes = hexToBytes(publicKeyHex);

	logger.debug('[importX25519PublicKey] Converted to bytes:', {
		byteLength: publicKeyBytes.length,
		bufferLength: publicKeyBytes.buffer.byteLength,
		byteOffset: publicKeyBytes.byteOffset,
		firstBytes: Array.from(publicKeyBytes.slice(0, 8))
	});

	try {
		// FIX: Create clean ArrayBuffer with exact size (no offset issues)
		// slice() creates a new ArrayBuffer from byteOffset to byteOffset+byteLength
		const cleanBuffer = publicKeyBytes.buffer.slice(
			publicKeyBytes.byteOffset,
			publicKeyBytes.byteOffset + publicKeyBytes.byteLength
		) as ArrayBuffer; // Type assertion: always ArrayBuffer in browser context

		// FIX: X25519 requires just the algorithm name string, NOT {name:'ECDH', namedCurve}
		// namedCurve is only for traditional ECDH curves (P-256, P-384, P-521)
		// X25519 is a specific algorithm identifier in WebCrypto
		const importedKey = await crypto.subtle.importKey(
			'raw',
			cleanBuffer, // ✅ Clean ArrayBuffer (exact 32 bytes, no offset)
			'X25519', // ✅ Correct: just the algorithm name (NOT {name:'ECDH', namedCurve:'X25519'})
			true, // extractable
			[]
		);

		logger.info('[importX25519PublicKey] ✅ X25519 public key imported successfully');
		return importedKey;
	} catch (error) {
		logger.error('[importX25519PublicKey] ❌ WebCrypto import failed:', {
			error: error instanceof Error ? error.message : String(error),
			errorName: error instanceof Error ? error.name : typeof error,
			publicKeyHex,
			byteLength: publicKeyBytes.length,
			byteOffset: publicKeyBytes.byteOffset
		});
		throw error;
	}
}

/**
 * Test RFC 7748 vector import (debugging tool)
 *
 * Tests if WebCrypto accepts a known-valid X25519 public key from RFC 7748 Section 6.1
 * This helps diagnose if the issue is with specific keys or WebCrypto API usage
 *
 * @returns {Promise<boolean>} True if RFC 7748 vector imports successfully, false otherwise
 */
export async function testImportRFC7748Vector(): Promise<boolean> {
	const { logger } = await import('$lib/utils/logger');

	// RFC 7748 Section 6.1 - Alice's public key (known valid test vector)
	const alice_public_hex = '8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a';

	logger.info('[RFC7748Test] Testing import of RFC 7748 Alice public key');

	try {
		const testKey = await importX25519PublicKey(alice_public_hex);
		logger.info('[RFC7748Test] ✅ SUCCESS - RFC 7748 vector imported correctly', {
			keyType: testKey.type,
			algorithm: testKey.algorithm
		});
		return true;
	} catch (error) {
		logger.error('[RFC7748Test] ❌ FAILED - RFC 7748 vector import failed', {
			error: error instanceof Error ? error.message : String(error),
			errorName: error instanceof Error ? error.name : typeof error
		});
		return false;
	}
}
