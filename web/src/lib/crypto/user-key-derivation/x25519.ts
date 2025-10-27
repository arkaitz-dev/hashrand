/**
 * X25519 key derivation and import utilities
 */

import { blake3 } from '@noble/hashes/blake3.js';
import { x25519 } from '@noble/curves/ed25519.js';
import { base58, base64urlnopad } from '@scure/base';
import { logger } from '$lib/utils/logger';
import { bytesToHex } from './helpers';

/**
 * Derive X25519 private key from email + privkey_context
 *
 * Process:
 * 1. Context = "X25519" + base58(privkey_context)
 * 2. IKM = email bytes
 * 3. Blake3(IKM, { context, dkLen: 32 }) → private_key[32]
 *
 * @param email - User email address
 * @param privkeyContext - 64-byte private key context from backend
 * @returns 32-byte X25519 private key
 */
export function deriveX25519PrivateKey(email: string, privkeyContext: Uint8Array): Uint8Array {
	if (privkeyContext.length !== 64) {
		throw new Error(`privkeyContext must be exactly 64 bytes, got ${privkeyContext.length} bytes`);
	}

	const emailBytes = new TextEncoder().encode(email);
	const privkeyContextB58 = base58.encode(privkeyContext);
	const contextString = 'X25519' + privkeyContextB58;
	const contextBytes = new TextEncoder().encode(contextString);

	logger.debug('[deriveX25519PrivateKey] Deriving key:', {
		email,
		contextLength: contextString.length,
		contextPrefix: contextString.substring(0, 20) + '...'
	});

	// Blake3 with context (KDF mode)
	const privateKey = blake3(emailBytes, {
		context: contextBytes,
		dkLen: 32
	});

	logger.debug('[deriveX25519PrivateKey] Derived X25519 private key:', {
		length: privateKey.length,
		first_8_bytes: Array.from(privateKey.slice(0, 8))
	});

	return privateKey;
}

/**
 * Generate X25519 public key from private key
 */
export function generateX25519PublicKey(privateKeyBytes: Uint8Array): {
	publicKeyBytes: Uint8Array;
	publicKeyHex: string;
} {
	const publicKeyBytes = x25519.getPublicKey(privateKeyBytes);
	const publicKeyHex = bytesToHex(publicKeyBytes);
	return { publicKeyBytes, publicKeyHex };
}

/**
 * Convert X25519 raw keys to JWK format for WebCrypto import
 *
 * Format complies with RFC 8037 (CFRG Elliptic Curve Diffie-Hellman)
 *
 * @param privateKeyBytes - 32-byte X25519 private key
 * @param publicKeyBytes - 32-byte X25519 public key
 * @returns JWK object ready for crypto.subtle.importKey()
 */
/* eslint-disable no-undef */
export function x25519ToJWK(privateKeyBytes: Uint8Array, publicKeyBytes: Uint8Array): JsonWebKey {
	logger.debug('[x25519ToJWK] Converting X25519 keys to JWK format:', {
		privateKeyLength: privateKeyBytes.length,
		publicKeyLength: publicKeyBytes.length
	});

	// Base64url encoding WITHOUT padding (RFC 4648 Section 5 / RFC 7515)
	const d_base64url = base64urlnopad.encode(privateKeyBytes);
	const x_base64url = base64urlnopad.encode(publicKeyBytes);

	const jwk: JsonWebKey = {
		kty: 'OKP',
		crv: 'X25519',
		d: d_base64url,
		x: x_base64url
		// Note: key_ops and ext omitted - let importKey parameters control these
	};
	/* eslint-enable no-undef */

	logger.debug('[x25519ToJWK] ✅ JWK created:', {
		kty: jwk.kty,
		crv: jwk.crv,
		d_length: d_base64url.length,
		x_length: x_base64url.length,
		x_value: x_base64url,
		d_has_padding: d_base64url.includes('='),
		x_has_padding: x_base64url.includes('=')
	});

	return jwk;
}

/**
 * Import X25519 private key from raw bytes to WebCrypto CryptoKey (non-extractable)
 * Uses JWK format as intermediate step (RFC 8037 compliant)
 *
 * @param privateKeyBytes - 32-byte X25519 private key
 * @param publicKeyBytes - 32-byte X25519 public key
 * @returns Non-extractable CryptoKey for ECDH operations
 */
export async function importX25519PrivateKey(
	privateKeyBytes: Uint8Array,
	publicKeyBytes: Uint8Array
): Promise<CryptoKey> {
	logger.debug('[importX25519PrivateKey] Starting X25519 private key import');

	try {
		const jwk = x25519ToJWK(privateKeyBytes, publicKeyBytes);

		logger.debug('[importX25519PrivateKey] Calling crypto.subtle.importKey...', {
			jwk_structure: {
				kty: jwk.kty,
				crv: jwk.crv,
				has_d: !!jwk.d,
				has_x: !!jwk.x
			}
		});

		const key = await crypto.subtle.importKey(
			'jwk',
			jwk,
			'X25519',
			false, // ✅ NON-EXTRACTABLE
			['deriveKey', 'deriveBits']
		);

		logger.info('[importX25519PrivateKey] ✅ X25519 private key imported successfully:', {
			type: key.type,
			extractable: key.extractable,
			algorithm: key.algorithm,
			usages: key.usages
		});

		// Limpiar JWK de memoria
		jwk.d = '';
		logger.debug('[importX25519PrivateKey] 🧹 JWK cleaned from memory');

		return key;
	} catch (error) {
		logger.error('[importX25519PrivateKey] ❌ Failed to import X25519 private key:', {
			error,
			error_name: error instanceof Error ? error.name : 'unknown',
			error_message: error instanceof Error ? error.message : String(error),
			error_stack: error instanceof Error ? error.stack : undefined
		});
		throw new Error(
			`X25519 private key import failed: ${error instanceof Error ? error.message : String(error)}`
		);
	}
}

/**
 * Import X25519 public key bytes to WebCrypto CryptoKey
 */
export async function importX25519PublicKey(publicKeyBytes: Uint8Array): Promise<CryptoKey> {
	const cleanBuffer = publicKeyBytes.buffer.slice(
		publicKeyBytes.byteOffset,
		publicKeyBytes.byteOffset + publicKeyBytes.byteLength
	) as ArrayBuffer;

	return await crypto.subtle.importKey(
		'raw',
		cleanBuffer,
		'X25519',
		true, // extractable
		[]
	);
}
