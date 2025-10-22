/**
 * User Key Derivation Module - Deterministic Ed25519/X25519 from privkey_context
 *
 * Derives user's permanent Ed25519 and X25519 keypairs from:
 * - privkey_context (64 random bytes from backend, unique per user)
 * - user email
 *
 * CRYPTOGRAPHIC DESIGN:
 * - Algorithm: Blake3 KDF (NOT standard blake3)
 * - IKM (Input Key Material): user email bytes
 * - Context: "Ed25519" + base58(privkey_context) for Ed25519
 *            "X25519" + base58(privkey_context) for X25519
 * - Output: 32 bytes for each private key
 *
 * SECURITY PROPERTIES:
 * - Deterministic: Same email + privkey_context → same keypairs
 * - Domain separation: Different context strings → independent keys
 * - User-bound: Email as IKM ensures keys tied to user identity
 * - Unique per user: privkey_context (64 random bytes) ensures uniqueness
 *
 * NOBLE CURVES INTEGRATION:
 * - Uses @noble/curves for Ed25519 and X25519
 * - Generates public keys from derived private keys
 * - Imports to WebCrypto CryptoKey objects for compatibility
 */

import { blake3 } from '@noble/hashes/blake3';
import { ed25519 } from '@noble/curves/ed25519';
import { x25519 } from '@noble/curves/ed25519';
import { base58, base64urlnopad } from '@scure/base';
import { logger } from '$lib/utils/logger';

/**
 * User keypair structure (derived from privkey_context)
 *
 * SECURITY: Private keys are stored as non-extractable CryptoKey objects
 *           Raw privateKeyBytes are NOT stored to maximize security
 */
export interface DerivedUserKeys {
	ed25519: {
		// ❌ privateKeyBytes removed for security
		publicKeyBytes: Uint8Array; // 32 bytes
		publicKeyHex: string; // 64 hex chars
		privateKey: CryptoKey; // WebCrypto CryptoKey (non-extractable)
		publicKey: CryptoKey; // WebCrypto CryptoKey (extractable)
	};
	x25519: {
		// ❌ privateKeyBytes removed for security
		publicKeyBytes: Uint8Array; // 32 bytes
		publicKeyHex: string; // 64 hex chars
		privateKey: CryptoKey; // WebCrypto CryptoKey (non-extractable)
		publicKey: CryptoKey; // WebCrypto CryptoKey (extractable)
	};
}

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
function deriveEd25519PrivateKey(email: string, privkeyContext: Uint8Array): Uint8Array {
	if (privkeyContext.length !== 64) {
		throw new Error(
			`privkeyContext must be exactly 64 bytes, got ${privkeyContext.length} bytes`
		);
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
function deriveX25519PrivateKey(email: string, privkeyContext: Uint8Array): Uint8Array {
	if (privkeyContext.length !== 64) {
		throw new Error(
			`privkeyContext must be exactly 64 bytes, got ${privkeyContext.length} bytes`
		);
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
 * Convert bytes to hex string
 */
function bytesToHex(bytes: Uint8Array): string {
	return Array.from(bytes)
		.map((b) => b.toString(16).padStart(2, '0'))
		.join('');
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
function ed25519ToJWK(privateKeyBytes: Uint8Array, publicKeyBytes: Uint8Array): JsonWebKey {
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
 * Convert X25519 raw keys to JWK format for WebCrypto import
 *
 * Format complies with RFC 8037 (CFRG Elliptic Curve Diffie-Hellman)
 *
 * @param privateKeyBytes - 32-byte X25519 private key
 * @param publicKeyBytes - 32-byte X25519 public key
 * @returns JWK object ready for crypto.subtle.importKey()
 */
function x25519ToJWK(privateKeyBytes: Uint8Array, publicKeyBytes: Uint8Array): JsonWebKey {
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
 * Import Ed25519 private key from raw bytes to WebCrypto CryptoKey (non-extractable)
 * Uses JWK format as intermediate step (RFC 8037 compliant)
 *
 * @param privateKeyBytes - 32-byte Ed25519 private key
 * @param publicKeyBytes - 32-byte Ed25519 public key
 * @returns Non-extractable CryptoKey for signing operations
 */
async function importEd25519PrivateKey(
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
		throw new Error(`Ed25519 private key import failed: ${error instanceof Error ? error.message : String(error)}`);
	}
}

/**
 * Import Ed25519 public key bytes to WebCrypto CryptoKey
 */
async function importEd25519PublicKey(publicKeyBytes: Uint8Array): Promise<CryptoKey> {
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

/**
 * Import X25519 private key from raw bytes to WebCrypto CryptoKey (non-extractable)
 * Uses JWK format as intermediate step (RFC 8037 compliant)
 *
 * @param privateKeyBytes - 32-byte X25519 private key
 * @param publicKeyBytes - 32-byte X25519 public key
 * @returns Non-extractable CryptoKey for ECDH operations
 */
async function importX25519PrivateKey(
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
		throw new Error(`X25519 private key import failed: ${error instanceof Error ? error.message : String(error)}`);
	}
}

/**
 * Import X25519 public key bytes to WebCrypto CryptoKey
 */
async function importX25519PublicKey(publicKeyBytes: Uint8Array): Promise<CryptoKey> {
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

/**
 * Derive complete user keypairs from email + privkey_context
 *
 * Generates deterministic Ed25519 and X25519 keypairs using Blake3 KDF.
 * Returns non-extractable WebCrypto CryptoKey objects for maximum security.
 *
 * SECURITY:
 * - Private key bytes are cleaned from memory after import
 * - Only non-extractable CryptoKeys are returned
 * - No raw private key bytes are stored anywhere
 *
 * @param email - User email address
 * @param privkeyContext - 64-byte private key context from backend
 * @returns Complete user keypairs with non-extractable CryptoKeys
 */
export async function deriveUserKeys(
	email: string,
	privkeyContext: Uint8Array
): Promise<DerivedUserKeys> {
	logger.info('[deriveUserKeys] Starting key derivation:', {
		email,
		privkeyContextLength: privkeyContext.length,
		privkeyContextB58: base58.encode(privkeyContext)
	});

	// 1. Derive Ed25519 private key
	const ed25519PrivateKeyBytes = deriveEd25519PrivateKey(email, privkeyContext);

	logger.debug('[deriveUserKeys] 🔍 Ed25519 private key bytes (before cleanup):', {
		length: ed25519PrivateKeyBytes.length,
		first_8_bytes: Array.from(ed25519PrivateKeyBytes.slice(0, 8))
	});

	// 2. Generate Ed25519 public key from private key (Noble)
	const ed25519PublicKeyBytes = ed25519.getPublicKey(ed25519PrivateKeyBytes);
	const ed25519PublicKeyHex = bytesToHex(ed25519PublicKeyBytes);

	logger.debug('[deriveUserKeys] ✅ Ed25519 keypair derived:', {
		publicKeyHex: ed25519PublicKeyHex
	});

	// 3. Import Ed25519 to WebCrypto CryptoKey (non-extractable)
	const ed25519PrivateKey = await importEd25519PrivateKey(
		ed25519PrivateKeyBytes,
		ed25519PublicKeyBytes
	);
	const ed25519PublicKey = await importEd25519PublicKey(ed25519PublicKeyBytes);

	// 4. ✅ CLEANUP: Overwrite Ed25519 private key bytes with zeros
	logger.debug('[deriveUserKeys] 🧹 Cleaning Ed25519 private key bytes from memory...');
	const ed25519BeforeCleanup = Array.from(ed25519PrivateKeyBytes.slice(0, 8));
	ed25519PrivateKeyBytes.fill(0);
	logger.info('[deriveUserKeys] ✅ Ed25519 private key bytes cleaned:', {
		before_first_8: ed25519BeforeCleanup,
		after_first_8: Array.from(ed25519PrivateKeyBytes.slice(0, 8)),
		all_zeros: ed25519PrivateKeyBytes.every((b) => b === 0)
	});

	// 5. Derive X25519 private key
	const x25519PrivateKeyBytes = deriveX25519PrivateKey(email, privkeyContext);

	logger.debug('[deriveUserKeys] 🔍 X25519 private key bytes (before cleanup):', {
		length: x25519PrivateKeyBytes.length,
		first_8_bytes: Array.from(x25519PrivateKeyBytes.slice(0, 8))
	});

	// 6. Generate X25519 public key from private key (Noble)
	const x25519PublicKeyBytes = x25519.getPublicKey(x25519PrivateKeyBytes);
	const x25519PublicKeyHex = bytesToHex(x25519PublicKeyBytes);

	logger.debug('[deriveUserKeys] ✅ X25519 keypair derived:', {
		publicKeyHex: x25519PublicKeyHex
	});

	// 7. Import X25519 to WebCrypto CryptoKey (non-extractable)
	const x25519PrivateKey = await importX25519PrivateKey(
		x25519PrivateKeyBytes,
		x25519PublicKeyBytes
	);
	const x25519PublicKey = await importX25519PublicKey(x25519PublicKeyBytes);

	// 8. ✅ CLEANUP: Overwrite X25519 private key bytes with zeros
	logger.debug('[deriveUserKeys] 🧹 Cleaning X25519 private key bytes from memory...');
	const x25519BeforeCleanup = Array.from(x25519PrivateKeyBytes.slice(0, 8));
	x25519PrivateKeyBytes.fill(0);
	logger.info('[deriveUserKeys] ✅ X25519 private key bytes cleaned:', {
		before_first_8: x25519BeforeCleanup,
		after_first_8: Array.from(x25519PrivateKeyBytes.slice(0, 8)),
		all_zeros: x25519PrivateKeyBytes.every((b) => b === 0)
	});

	logger.info('[deriveUserKeys] 🔐 User keys successfully derived (non-extractable):', {
		ed25519PublicKeyHex,
		x25519PublicKeyHex,
		ed25519_extractable: ed25519PrivateKey.extractable,
		x25519_extractable: x25519PrivateKey.extractable
	});

	return {
		ed25519: {
			// ❌ NO privateKeyBytes (security)
			publicKeyBytes: ed25519PublicKeyBytes,
			publicKeyHex: ed25519PublicKeyHex,
			privateKey: ed25519PrivateKey, // ✅ Real CryptoKey non-extractable
			publicKey: ed25519PublicKey
		},
		x25519: {
			// ❌ NO privateKeyBytes (security)
			publicKeyBytes: x25519PublicKeyBytes,
			publicKeyHex: x25519PublicKeyHex,
			privateKey: x25519PrivateKey, // ✅ Real CryptoKey non-extractable
			publicKey: x25519PublicKey
		}
	};
}

/**
 * Verify derived public keys against JWT public keys
 *
 * @param derivedKeys - Derived user keypairs
 * @param jwtEd25519Hex - Ed25519 public key from JWT (64 hex chars)
 * @param jwtX25519Hex - X25519 public key from JWT (64 hex chars)
 * @returns true if keys match, false otherwise
 */
export function verifyDerivedPublicKeys(
	derivedKeys: DerivedUserKeys,
	jwtEd25519Hex: string,
	jwtX25519Hex: string
): boolean {
	const ed25519Match = derivedKeys.ed25519.publicKeyHex === jwtEd25519Hex;
	const x25519Match = derivedKeys.x25519.publicKeyHex === jwtX25519Hex;

	logger.debug('[verifyDerivedPublicKeys] Public key verification:', {
		ed25519Match,
		x25519Match,
		derived_ed25519: derivedKeys.ed25519.publicKeyHex,
		jwt_ed25519: jwtEd25519Hex,
		derived_x25519: derivedKeys.x25519.publicKeyHex,
		jwt_x25519: jwtX25519Hex
	});

	return ed25519Match && x25519Match;
}
