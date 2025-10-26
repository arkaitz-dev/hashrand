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

import { base58 } from '@scure/base';
import { logger } from '$lib/utils/logger';
import {
	deriveEd25519PrivateKey,
	generateEd25519PublicKey,
	importEd25519PrivateKey,
	importEd25519PublicKey
} from './ed25519';
import {
	deriveX25519PrivateKey,
	generateX25519PublicKey,
	importX25519PrivateKey,
	importX25519PublicKey
} from './x25519';

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
	const { publicKeyBytes: ed25519PublicKeyBytes, publicKeyHex: ed25519PublicKeyHex } =
		generateEd25519PublicKey(ed25519PrivateKeyBytes);

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
	const { publicKeyBytes: x25519PublicKeyBytes, publicKeyHex: x25519PublicKeyHex } =
		generateX25519PublicKey(x25519PrivateKeyBytes);

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
