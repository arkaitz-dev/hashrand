/**
 * Shared Secret E2E Encryption Module
 *
 * Single Responsibility: ChaCha20-Poly1305 encryption + ECDH key agreement for shared secrets
 * Part of SOLID architecture - Pure crypto operations with ZERO UI dependencies
 *
 * Architecture:
 * 1. Frontend generates random key_material[44] (nonce[12] + cipher_key[32])
 * 2. Frontend encrypts secret_text with ChaCha20-Poly1305 using key_material
 * 3. Frontend encrypts key_material with ECDH (frontend private + backend public)
 * 4. Backend decrypts key_material with ECDH, stores both encrypted_secret + key_material
 * 5. Backend encrypts key_material with ECDH (backend private + requester public) for response
 * 6. Frontend decrypts key_material with ECDH, then decrypts secret_text with ChaCha20-Poly1305
 *
 * Security:
 * - Ed25519 → X25519 conversion for ECDH (reusing signature keys for encryption)
 * - Blake3 KDF for cipher key + nonce derivation from ECDH shared secret
 * - Context string "SharedSecretKeyMaterial_v1" for domain separation
 * - Zero-Knowledge transport: Secret never travels in cleartext
 */

import { ed25519 } from '@noble/curves/ed25519.js';
import { x25519 } from '@noble/curves/ed25519.js';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils.js';
import { sha512 } from '@noble/hashes/sha2';
import { chacha20poly1305 } from '@noble/ciphers/chacha.js';
import { blake3 } from '@noble/hashes/blake3';
import { logger } from '$lib/utils/logger';

// Constants matching backend
const NONCE_LENGTH = 12; // ChaCha20-Poly1305 nonce size
const CIPHER_KEY_LENGTH = 32; // ChaCha20 key size
const KEY_MATERIAL_LENGTH = NONCE_LENGTH + CIPHER_KEY_LENGTH; // 44 bytes total
const ECDH_CONTEXT = 'SharedSecretKeyMaterial_v1'; // Domain separation for ECDH

/**
 * Convert Ed25519 private key to X25519 private key
 *
 * Uses SHA-512 hash + scalar clamping (RFC 7748)
 * Matches backend ed25519_to_x25519.rs implementation exactly
 *
 * @param ed25519PrivateBytes - Ed25519 private key (32 bytes)
 * @returns X25519 private key (32 bytes)
 */
function ed25519PrivateToX25519(ed25519PrivateBytes: Uint8Array): Uint8Array {
	logger.debug('🔄 Converting Ed25519 private key to X25519');

	if (ed25519PrivateBytes.length !== 32) {
		throw new Error(
			`Invalid Ed25519 private key length: ${ed25519PrivateBytes.length}, expected 32`
		);
	}

	// SHA-512 hash of private key (matches backend exactly)
	const hash = sha512(ed25519PrivateBytes);

	// Take first 32 bytes
	const x25519Bytes = new Uint8Array(hash.slice(0, 32));

	// Clamp scalar (RFC 7748)
	x25519Bytes[0] &= 248; // Clear bits 0, 1, 2
	x25519Bytes[31] &= 127; // Clear bit 255
	x25519Bytes[31] |= 64; // Set bit 254

	logger.debug('✅ Ed25519 → X25519 private key conversion successful (clamped)');
	return x25519Bytes;
}

/**
 * Convert Ed25519 public key to X25519 public key
 *
 * Uses Montgomery curve point conversion via ed25519.getPublicKey
 * Matches backend ed25519_to_x25519.rs implementation
 *
 * NOTE: For Ed25519 public key conversion, we use the edwardsToMontgomeryPub
 * utility from @noble/curves/ed25519 which implements the same conversion
 * formula as the backend: u = (1 + y) / (1 - y)
 *
 * @param ed25519PublicBytes - Ed25519 public key (32 bytes)
 * @returns X25519 public key (32 bytes)
 */
function ed25519PublicToX25519(ed25519PublicBytes: Uint8Array): Uint8Array {
	logger.debug('🔄 Converting Ed25519 public key to X25519');

	if (ed25519PublicBytes.length !== 32) {
		throw new Error(`Invalid Ed25519 public key length: ${ed25519PublicBytes.length}, expected 32`);
	}

	// Convert Ed25519 point to X25519 using toMontgomery
	// This implements the formula u = (1 + y) / (1 - y) in the field
	try {
		const x25519Public = ed25519.utils.toMontgomery(ed25519PublicBytes);

		logger.debug('✅ Ed25519 → X25519 public key conversion successful');
		return x25519Public;
	} catch (error) {
		logger.error('❌ Failed to convert Ed25519 public key to X25519:', error);
		throw new Error(`Ed25519 to X25519 conversion failed: ${error}`);
	}
}

/**
 * Perform ECDH key agreement and derive cipher key + nonce
 *
 * Uses Blake3 KDF with context string for domain separation
 *
 * @param myPrivate - My X25519 private key (32 bytes)
 * @param theirPublic - Their X25519 public key (32 bytes)
 * @returns Object with cipher_key[32] and nonce[12]
 */
function deriveEncryptionMaterial(
	myPrivate: Uint8Array,
	theirPublic: Uint8Array
): { cipherKey: Uint8Array; nonce: Uint8Array } {
	logger.debug('🔐 ECDH: Computing shared secret');

	// Perform ECDH
	const sharedSecret = x25519.getSharedSecret(myPrivate, theirPublic);
	logger.debug('🔐 ECDH: Computed shared secret');

	// Derive cipher_key + nonce using Blake3 KDF with context
	const contextBytes = new TextEncoder().encode(ECDH_CONTEXT);
	const hasher = blake3.create({ key: sharedSecret });
	hasher.update(contextBytes);
	const derived = hasher.xof(CIPHER_KEY_LENGTH + NONCE_LENGTH); // 44 bytes

	const cipherKey = derived.slice(0, CIPHER_KEY_LENGTH);
	const nonce = derived.slice(CIPHER_KEY_LENGTH);

	logger.debug('🔐 ECDH: Derived cipher_key[32] + nonce[12] with Blake3');

	return { cipherKey, nonce };
}

/**
 * Encrypt data with ECDH + ChaCha20-Poly1305
 *
 * @param plaintext - Data to encrypt
 * @param myPrivate - My X25519 private key (32 bytes)
 * @param theirPublic - Their X25519 public key (32 bytes)
 * @returns Encrypted data (plaintext.length + 16 bytes for auth tag)
 */
function encryptWithECDH(
	plaintext: Uint8Array,
	myPrivate: Uint8Array,
	theirPublic: Uint8Array
): Uint8Array {
	logger.debug(`🔐 ECDH: Starting encryption (plaintext_size=${plaintext.length})`);

	// Derive cipher key + nonce from ECDH
	const { cipherKey, nonce } = deriveEncryptionMaterial(myPrivate, theirPublic);

	// Encrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, nonce);
	const ciphertext = cipher.encrypt(plaintext);

	logger.debug(`✅ ECDH: Encrypted data (output_size=${ciphertext.length})`);
	return ciphertext;
}

/**
 * Decrypt data with ECDH + ChaCha20-Poly1305
 *
 * @param ciphertext - Encrypted data
 * @param myPrivate - My X25519 private key (32 bytes)
 * @param theirPublic - Their X25519 public key (32 bytes)
 * @returns Decrypted plaintext
 * @throws Error if authentication tag verification fails
 */
function decryptWithECDH(
	ciphertext: Uint8Array,
	myPrivate: Uint8Array,
	theirPublic: Uint8Array
): Uint8Array {
	logger.debug(`🔓 ECDH: Starting decryption (ciphertext_size=${ciphertext.length})`);

	// Derive cipher key + nonce from ECDH
	const { cipherKey, nonce } = deriveEncryptionMaterial(myPrivate, theirPublic);

	// Decrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, nonce);
	const plaintext = cipher.decrypt(ciphertext);

	logger.debug(`✅ ECDH: Decrypted data (output_size=${plaintext.length})`);
	return plaintext;
}

/**
 * Generate random key_material for ChaCha20-Poly1305
 *
 * @returns 44 bytes: nonce[12] + cipher_key[32]
 */
function generateRandomKeyMaterial(): Uint8Array {
	logger.debug('🎲 Generating random key_material[44] for ChaCha20-Poly1305');
	const keyMaterial = crypto.getRandomValues(new Uint8Array(KEY_MATERIAL_LENGTH));
	logger.debug('✅ Random key_material generated');
	return keyMaterial;
}

/**
 * Encrypt secret text with ChaCha20-Poly1305 using provided key_material
 *
 * @param secretText - Secret text to encrypt
 * @param keyMaterial - 44 bytes: nonce[12] + cipher_key[32]
 * @returns Encrypted secret (includes 16-byte auth tag)
 */
function encryptSecretText(secretText: string, keyMaterial: Uint8Array): Uint8Array {
	logger.debug(`🔐 Encrypting secret text (length=${secretText.length})`);

	if (keyMaterial.length !== KEY_MATERIAL_LENGTH) {
		throw new Error(`Invalid key_material length: ${keyMaterial.length}, expected ${KEY_MATERIAL_LENGTH}`);
	}

	// Extract nonce and cipher_key from key_material
	const nonce = keyMaterial.slice(0, NONCE_LENGTH);
	const cipherKey = keyMaterial.slice(NONCE_LENGTH);

	// Encrypt with ChaCha20-Poly1305
	const plaintext = new TextEncoder().encode(secretText);
	const cipher = chacha20poly1305(cipherKey, nonce);
	const ciphertext = cipher.encrypt(plaintext);

	logger.debug(`✅ Secret text encrypted (output_size=${ciphertext.length})`);
	return ciphertext;
}

/**
 * Decrypt secret text with ChaCha20-Poly1305 using provided key_material
 *
 * @param encryptedSecret - Encrypted secret (includes auth tag)
 * @param keyMaterial - 44 bytes: nonce[12] + cipher_key[32]
 * @returns Decrypted secret text
 * @throws Error if authentication tag verification fails
 */
function decryptSecretText(encryptedSecret: Uint8Array, keyMaterial: Uint8Array): string {
	logger.debug(`🔓 Decrypting secret text (ciphertext_size=${encryptedSecret.length})`);

	if (keyMaterial.length !== KEY_MATERIAL_LENGTH) {
		throw new Error(`Invalid key_material length: ${keyMaterial.length}, expected ${KEY_MATERIAL_LENGTH}`);
	}

	// Extract nonce and cipher_key from key_material
	const nonce = keyMaterial.slice(0, NONCE_LENGTH);
	const cipherKey = keyMaterial.slice(NONCE_LENGTH);

	// Decrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, nonce);
	const plaintext = cipher.decrypt(encryptedSecret);

	const secretText = new TextDecoder().decode(plaintext);
	logger.debug(`✅ Secret text decrypted (length=${secretText.length})`);
	return secretText;
}

/**
 * Encrypt secret for creation (sender encrypts before sending to backend)
 *
 * Flow:
 * 1. Generate random key_material[44]
 * 2. Encrypt secret_text with ChaCha20-Poly1305 using key_material
 * 3. Convert sender's Ed25519 private → X25519
 * 4. Convert backend's Ed25519 public → X25519
 * 5. Encrypt key_material with ECDH (sender private + backend public)
 *
 * @param secretText - Secret text to encrypt
 * @param senderPrivateKeyHex - Sender's Ed25519 private key (hex, 64 chars)
 * @param backendPublicKeyHex - Backend's X25519 public key (hex, 64 chars)
 * @returns Object with encrypted_secret and encrypted_key_material (both Uint8Array)
 */
export function encryptSecretForCreation(
	secretText: string,
	senderPrivateKeyHex: string,
	backendPublicKeyHex: string
): { encryptedSecret: Uint8Array; encryptedKeyMaterial: Uint8Array } {
	logger.debug('🔐 Starting secret encryption for creation');

	// 1. Generate random key_material
	const keyMaterial = generateRandomKeyMaterial();

	// 2. Encrypt secret_text with ChaCha20-Poly1305
	const encryptedSecret = encryptSecretText(secretText, keyMaterial);

	// 3. Convert sender's Ed25519 private → X25519
	const senderEd25519Private = hexToBytes(senderPrivateKeyHex);
	const senderX25519Private = ed25519PrivateToX25519(senderEd25519Private);

	// 4. Backend public key is already X25519
	const backendX25519Public = hexToBytes(backendPublicKeyHex);

	// 5. Encrypt key_material with ECDH
	const encryptedKeyMaterial = encryptWithECDH(keyMaterial, senderX25519Private, backendX25519Public);

	logger.debug('✅ Secret encryption for creation complete');
	return { encryptedSecret, encryptedKeyMaterial };
}

/**
 * Decrypt secret after retrieval (requester decrypts after receiving from backend)
 *
 * Flow:
 * 1. Convert requester's Ed25519 private → X25519
 * 2. Backend public key is already X25519
 * 3. Decrypt key_material with ECDH (requester private + backend public)
 * 4. Decrypt secret_text with ChaCha20-Poly1305 using key_material
 *
 * @param encryptedSecret - Encrypted secret from backend (Uint8Array)
 * @param encryptedKeyMaterial - Encrypted key_material from backend (Uint8Array)
 * @param requesterPrivateKeyHex - Requester's Ed25519 private key (hex, 64 chars)
 * @param backendPublicKeyHex - Backend's X25519 public key (hex, 64 chars)
 * @returns Decrypted secret text
 */
export function decryptSecretAfterRetrieval(
	encryptedSecret: Uint8Array,
	encryptedKeyMaterial: Uint8Array,
	requesterPrivateKeyHex: string,
	backendPublicKeyHex: string
): string {
	logger.debug('🔓 Starting secret decryption after retrieval');

	// 1. Convert requester's Ed25519 private → X25519
	const requesterEd25519Private = hexToBytes(requesterPrivateKeyHex);
	const requesterX25519Private = ed25519PrivateToX25519(requesterEd25519Private);

	// 2. Backend public key is already X25519
	const backendX25519Public = hexToBytes(backendPublicKeyHex);

	// 3. Decrypt key_material with ECDH
	const keyMaterial = decryptWithECDH(encryptedKeyMaterial, requesterX25519Private, backendX25519Public);

	// 4. Decrypt secret_text with ChaCha20-Poly1305
	const secretText = decryptSecretText(encryptedSecret, keyMaterial);

	logger.debug('✅ Secret decryption after retrieval complete');
	return secretText;
}
