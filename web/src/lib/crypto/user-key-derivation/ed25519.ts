/**
 * Ed25519 key derivation and import utilities
 */

import { blake3 } from '@noble/hashes/blake3.js';
import { ed25519 } from '@noble/curves/ed25519.js';
import { base58, base64urlnopad } from '@scure/base';
import { logger } from '$lib/utils/logger';
import { bytesToHex } from './helpers';

/**
 * Derive Ed25519 private key from email + privkey_context
 *
 * Process:
 * 1. Context = "Ed25519" + base58(privkey_context)
 * 2. IKM = email bytes
 * 3. Blake3(IKM, { context, dkLen: 32 }) → private_key[32]
 *
 * @param email - User email address
 * @param privkeyContext - 64-byte private key context from backend
 * @returns 32-byte Ed25519 private key
 */
export function deriveEd25519PrivateKey(email: string, privkeyContext: Uint8Array): Uint8Array {
	if (privkeyContext.length !== 64) {
		throw new Error(`privkeyContext must be exactly 64 bytes, got ${privkeyContext.length} bytes`);
	}

	const emailBytes = new TextEncoder().encode(email);
	const privkeyContextB58 = base58.encode(privkeyContext);
	const contextString = 'Ed25519' + privkeyContextB58;
	const contextBytes = new TextEncoder().encode(contextString);

	logger.debug('[deriveEd25519PrivateKey] Deriving key:', {
		email,
		contextLength: contextString.length,
		contextPrefix: contextString.substring(0, 20) + '...'
	});

	// Blake3 with context (KDF mode)
	const privateKey = blake3(emailBytes, {
		context: contextBytes,
		dkLen: 32
	});

	logger.debug('[deriveEd25519PrivateKey] Derived Ed25519 private key:', {
		length: privateKey.length,
		first_8_bytes: Array.from(privateKey.slice(0, 8))
	});

	return privateKey;
}

/**
 * Generate Ed25519 public key from private key
 */
export function generateEd25519PublicKey(privateKeyBytes: Uint8Array): {
	publicKeyBytes: Uint8Array;
	publicKeyHex: string;
} {
	const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);
	const publicKeyHex = bytesToHex(publicKeyBytes);
	return { publicKeyBytes, publicKeyHex };
}

/**
 * Convert Ed25519 raw keys to JWK format for WebCrypto import
 *
 * Format complies with RFC 8037 (CFRG Elliptic Curve Diffie-Hellman)
 *
 * @param privateKeyBytes - 32-byte Ed25519 private key
 * @param publicKeyBytes - 32-byte Ed25519 public key
 * @returns JWK object ready for crypto.subtle.importKey()
 */
/* eslint-disable no-undef */
export function ed25519ToJWK(privateKeyBytes: Uint8Array, publicKeyBytes: Uint8Array): JsonWebKey {
	logger.debug('[ed25519ToJWK] Converting Ed25519 keys to JWK format:', {
		privateKeyLength: privateKeyBytes.length,
		publicKeyLength: publicKeyBytes.length
	});

	// Base64url encoding WITHOUT padding (RFC 4648 Section 5 / RFC 7515)
	const d_base64url = base64urlnopad.encode(privateKeyBytes);
	const x_base64url = base64urlnopad.encode(publicKeyBytes);

	const jwk: JsonWebKey = {
		kty: 'OKP',
		crv: 'Ed25519',
		d: d_base64url,
		x: x_base64url
		// Note: key_ops and ext omitted - let importKey parameters control these
	};
	/* eslint-enable no-undef */

	logger.debug('[ed25519ToJWK] ✅ JWK created:', {
		kty: jwk.kty,
		crv: jwk.crv,
		d_length: d_base64url.length,
		x_length: x_base64url.length,
		x_value: x_base64url, // Public key is safe to show
		d_first_10: d_base64url.substring(0, 10),
		d_last_10: d_base64url.substring(d_base64url.length - 10),
		x_first_10: x_base64url.substring(0, 10),
		x_last_10: x_base64url.substring(x_base64url.length - 10),
		d_has_padding: d_base64url.includes('='),
		x_has_padding: x_base64url.includes('='),
		d_has_plus: d_base64url.includes('+'),
		x_has_plus: x_base64url.includes('+'),
		d_has_slash: d_base64url.includes('/'),
		x_has_slash: x_base64url.includes('/')
	});

	return jwk;
}

/**
 * Import Ed25519 private key from raw bytes to WebCrypto CryptoKey (non-extractable)
 * Uses JWK format as intermediate step (RFC 8037 compliant)
 *
 * @param privateKeyBytes - 32-byte Ed25519 private key
 * @param publicKeyBytes - 32-byte Ed25519 public key
 * @returns Non-extractable CryptoKey for signing operations
 */
export async function importEd25519PrivateKey(
	privateKeyBytes: Uint8Array,
	publicKeyBytes: Uint8Array
): Promise<CryptoKey> {
	logger.debug('[importEd25519PrivateKey] Starting Ed25519 private key import');

	try {
		const jwk = ed25519ToJWK(privateKeyBytes, publicKeyBytes);

		logger.debug('[importEd25519PrivateKey] Calling crypto.subtle.importKey...', {
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
			'Ed25519', // Algorithm as string (not object)
			false, // ✅ NON-EXTRACTABLE
			['sign']
		);

		logger.info('[importEd25519PrivateKey] ✅ Ed25519 private key imported successfully:', {
			type: key.type,
			extractable: key.extractable,
			algorithm: key.algorithm,
			usages: key.usages
		});

		// Limpiar JWK de memoria
		jwk.d = '';
		logger.debug('[importEd25519PrivateKey] 🧹 JWK cleaned from memory');

		return key;
	} catch (error) {
		logger.error('[importEd25519PrivateKey] ❌ Failed to import Ed25519 private key:', {
			error,
			error_name: error instanceof Error ? error.name : 'unknown',
			error_message: error instanceof Error ? error.message : String(error),
			error_stack: error instanceof Error ? error.stack : undefined
		});
		throw new Error(
			`Ed25519 private key import failed: ${error instanceof Error ? error.message : String(error)}`
		);
	}
}

/**
 * Import Ed25519 public key bytes to WebCrypto CryptoKey
 */
export async function importEd25519PublicKey(publicKeyBytes: Uint8Array): Promise<CryptoKey> {
	// Create clean ArrayBuffer (fix for TypeScript strict checking)
	const cleanBuffer = publicKeyBytes.buffer.slice(
		publicKeyBytes.byteOffset,
		publicKeyBytes.byteOffset + publicKeyBytes.byteLength
	) as ArrayBuffer;

	return await crypto.subtle.importKey(
		'raw',
		cleanBuffer,
		{
			name: 'Ed25519',
			namedCurve: 'Ed25519'
		},
		true, // extractable
		['verify']
	);
}
