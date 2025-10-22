/**
 * Shared Secret E2E Encryption Module
 *
 * Single Responsibility: ChaCha20-Poly1305 encryption + ECDH key agreement for shared secrets
 * Part of SOLID architecture - Pure crypto operations with ZERO UI dependencies
 *
 * Architecture:
 * 1. Frontend generates random key_material[44] (nonce[12] + cipher_key[32])
 * 2. Frontend encrypts secret_text with ChaCha20-Poly1305 using key_material
 * 3. Frontend encrypts key_material with ECDH (frontend X25519 private + backend X25519 public)
 * 4. Backend decrypts key_material with ECDH, stores both encrypted_secret + key_material
 * 5. Backend encrypts key_material with ECDH (backend X25519 private + requester X25519 public) for response
 * 6. Frontend decrypts key_material with ECDH, then decrypts secret_text with ChaCha20-Poly1305
 *
 * Security:
 * - WebCrypto native X25519 ECDH (non-extractable private keys in IndexedDB)
 * - Blake3 KDF for cipher key + nonce derivation from ECDH shared secret
 * - Context string "SharedSecretKeyMaterial_v1" for domain separation
 * - Zero-Knowledge transport: Secret never travels in cleartext
 *
 * ARCHITECTURE CHANGE:
 * - Ed25519 and X25519 are now INDEPENDENT keypairs (not converted)
 * - Uses WebCrypto API for ECDH operations
 * - ChaCha20-Poly1305 still uses Noble (WebCrypto doesn't support it)
 */

import { chacha20poly1305 } from '@noble/ciphers/chacha.js';
import { blake3 } from '@noble/hashes/blake3';
import { logger } from '$lib/utils/logger';

// Constants matching backend
const NONCE_LENGTH = 12; // ChaCha20-Poly1305 nonce size
const CIPHER_KEY_LENGTH = 32; // ChaCha20 key size
const KEY_MATERIAL_LENGTH = NONCE_LENGTH + CIPHER_KEY_LENGTH; // 44 bytes total
const ECDH_CONTEXT = 'SharedSecretKeyMaterial_v1'; // Domain separation for ECDH

/**
 * Perform ECDH key agreement using WebCrypto and derive cipher key + nonce
 *
 * Uses WebCrypto X25519 deriveBits + Blake3 KDF with context string for domain separation
 *
 * @param myPrivateKey - My X25519 private key CryptoKey
 * @param theirPublicKey - Their X25519 public key CryptoKey
 * @returns Object with cipher_key[32] and nonce[12]
 */
async function deriveEncryptionMaterial(
	myPrivateKey: CryptoKey,
	theirPublicKey: CryptoKey
): Promise<{ cipherKey: Uint8Array; nonce: Uint8Array }> {
	logger.debug('🔐 ECDH: Computing shared secret with WebCrypto');

	// Perform ECDH with WebCrypto
	// FIX: X25519 requires algorithm name string 'X25519', NOT {name:'ECDH', namedCurve}
	// namedCurve is ONLY for traditional ECDH curves (P-256, P-384, P-521)
	const sharedSecretBuffer = await crypto.subtle.deriveBits(
		{
			name: 'X25519',
			public: theirPublicKey
		},
		myPrivateKey,
		256 // 32 bytes
	);

	const sharedSecret = new Uint8Array(sharedSecretBuffer);
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
 * Encrypt data with ECDH + ChaCha20-Poly1305 (WebCrypto)
 *
 * @param plaintext - Data to encrypt
 * @param myPrivateKey - My X25519 private key CryptoKey
 * @param theirPublicKey - Their X25519 public key CryptoKey
 * @returns Encrypted data (plaintext.length + 16 bytes for auth tag)
 */
async function encryptWithECDH(
	plaintext: Uint8Array,
	myPrivateKey: CryptoKey,
	theirPublicKey: CryptoKey
): Promise<Uint8Array> {
	logger.debug(`🔐 ECDH: Starting encryption (plaintext_size=${plaintext.length})`);

	// Derive cipher key + nonce from ECDH
	const { cipherKey, nonce } = await deriveEncryptionMaterial(myPrivateKey, theirPublicKey);

	// Encrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, nonce);
	const ciphertext = cipher.encrypt(plaintext);

	logger.debug(`✅ ECDH: Encrypted data (output_size=${ciphertext.length})`);
	return ciphertext;
}

/**
 * Decrypt data with ECDH + ChaCha20-Poly1305 (WebCrypto)
 *
 * @param ciphertext - Encrypted data
 * @param myPrivateKey - My X25519 private key CryptoKey
 * @param theirPublicKey - Their X25519 public key CryptoKey
 * @returns Decrypted plaintext
 * @throws Error if authentication tag verification fails
 */
async function decryptWithECDH(
	ciphertext: Uint8Array,
	myPrivateKey: CryptoKey,
	theirPublicKey: CryptoKey
): Promise<Uint8Array> {
	logger.debug(`🔓 ECDH: Starting decryption (ciphertext_size=${ciphertext.length})`);

	// Derive cipher key + nonce from ECDH
	const { cipherKey, nonce } = await deriveEncryptionMaterial(myPrivateKey, theirPublicKey);

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
 * 3. Get sender's X25519 private key from IndexedDB
 * 4. Import backend's X25519 public key from hex
 * 5. Encrypt key_material with ECDH (sender X25519 private + backend X25519 public)
 *
 * @param secretText - Secret text to encrypt
 * @param backendPublicKeyHex - Backend's X25519 public key (hex, 64 chars)
 * @returns Promise with encrypted_secret and encrypted_key_material (both Uint8Array)
 */
export async function encryptSecretForCreation(
	secretText: string,
	backendPublicKeyHex: string
): Promise<{ encryptedSecret: Uint8Array; encryptedKeyMaterial: Uint8Array }> {
	logger.debug('🔐 Starting secret encryption for creation', {
		backendPublicKeyHex_received: backendPublicKeyHex,
		length: backendPublicKeyHex?.length,
		type: typeof backendPublicKeyHex
	});

	// 1. Generate random key_material
	const keyMaterial = generateRandomKeyMaterial();

	// 2. Encrypt secret_text with ChaCha20-Poly1305
	const encryptedSecret = encryptSecretText(secretText, keyMaterial);

	// 3. Get sender's X25519 private key from IndexedDB
	const { getX25519PrivateKey } = await import('./keypair-storage');
	const senderX25519PrivateKey = await getX25519PrivateKey();

	if (!senderX25519PrivateKey) {
		throw new Error('X25519 private key not found in IndexedDB');
	}

	// 4. Import backend's X25519 public key from hex
	const { importX25519PublicKey } = await import('./keypair-generation');
	const backendX25519PublicKey = await importX25519PublicKey(backendPublicKeyHex);

	// 5. Encrypt key_material with ECDH (WebCrypto)
	const encryptedKeyMaterial = await encryptWithECDH(
		keyMaterial,
		senderX25519PrivateKey,
		backendX25519PublicKey
	);

	logger.debug('✅ Secret encryption for creation complete');
	return { encryptedSecret, encryptedKeyMaterial };
}

/**
 * Decrypt secret after retrieval (requester decrypts after receiving from backend)
 *
 * Flow:
 * 1. Get requester's X25519 private key from IndexedDB
 * 2. Import backend's X25519 public key from hex
 * 3. Decrypt key_material with ECDH (requester X25519 private + backend X25519 public)
 * 4. Decrypt secret_text with ChaCha20-Poly1305 using key_material
 *
 * @param encryptedSecret - Encrypted secret from backend (Uint8Array)
 * @param encryptedKeyMaterial - Encrypted key_material from backend (Uint8Array)
 * @param backendPublicKeyHex - Backend's X25519 public key (hex, 64 chars)
 * @returns Promise with decrypted secret text
 */
export async function decryptSecretAfterRetrieval(
	encryptedSecret: Uint8Array,
	encryptedKeyMaterial: Uint8Array,
	backendPublicKeyHex: string
): Promise<string> {
	logger.debug('🔓 Starting secret decryption after retrieval');

	// 1. Get requester's X25519 private key from IndexedDB
	const { getX25519PrivateKey } = await import('./keypair-storage');
	const requesterX25519PrivateKey = await getX25519PrivateKey();

	if (!requesterX25519PrivateKey) {
		throw new Error('X25519 private key not found in IndexedDB');
	}

	// 2. Import backend's X25519 public key from hex
	const { importX25519PublicKey } = await import('./keypair-generation');
	const backendX25519PublicKey = await importX25519PublicKey(backendPublicKeyHex);

	// 3. Decrypt key_material with ECDH (WebCrypto)
	const keyMaterial = await decryptWithECDH(
		encryptedKeyMaterial,
		requesterX25519PrivateKey,
		backendX25519PublicKey
	);

	// 4. Decrypt secret_text with ChaCha20-Poly1305
	const secretText = decryptSecretText(encryptedSecret, keyMaterial);

	logger.debug('✅ Secret decryption after retrieval complete');
	return secretText;
}

/**
 * Decrypt user private key context from backend (magic link validation)
 *
 * Flow:
 * 1. Decode base64 → Uint8Array[80] (64 bytes + 16 MAC)
 * 2. Get client's X25519 private key from IndexedDB
 * 3. Import server's X25519 public key from hex
 * 4. Decrypt with ECDH (client X25519 private + server X25519 public)
 *
 * Uses same ECDH encryption as shared secrets (Blake3 KDF + ChaCha20-Poly1305)
 * Context: "SharedSecretKeyMaterial_v1"
 *
 * @param encryptedBase64 - Base64-encoded encrypted privkey_context (80 bytes when decoded)
 * @param serverPublicKeyHex - Server's X25519 public key (hex, 64 chars)
 * @returns Promise with decrypted privkey_context (64 bytes)
 * @throws Error if keys not found, invalid format, or decryption fails (MAC verification)
 */
export async function decryptPrivkeyContext(
	encryptedBase64: string,
	serverPublicKeyHex: string
): Promise<Uint8Array> {
	logger.debug('🔓 Starting privkey_context decryption from magic link response');

	// 1. Decode base64 → Uint8Array[80]
	const { base64ToBytes } = await import('./crypto-encoding');
	const encryptedBytes = base64ToBytes(encryptedBase64);

	logger.debug('🔓 Decoded base64 privkey_context', {
		encryptedSize: encryptedBytes.length,
		expectedSize: 80 // 64 + 16 MAC
	});

	if (encryptedBytes.length !== 80) {
		throw new Error(
			`Invalid encrypted_privkey_context size: ${encryptedBytes.length} (expected 80 bytes)`
		);
	}

	// 2. Get client's X25519 private key from IndexedDB
	const { getX25519PrivateKey } = await import('./keypair-storage');
	const clientX25519PrivateKey = await getX25519PrivateKey();

	if (!clientX25519PrivateKey) {
		throw new Error('X25519 private key not found in IndexedDB');
	}

	// 3. Import server's X25519 public key from hex
	const { importX25519PublicKey } = await import('./keypair-generation');
	const serverX25519PublicKey = await importX25519PublicKey(serverPublicKeyHex);

	// 4. Decrypt with ECDH (reuses existing decryptWithECDH function)
	const privkeyContext = await decryptWithECDH(
		encryptedBytes,
		clientX25519PrivateKey,
		serverX25519PublicKey
	);

	logger.debug('✅ Privkey context decrypted successfully', {
		size: privkeyContext.length,
		expectedSize: 64
	});

	if (privkeyContext.length !== 64) {
		throw new Error(
			`Invalid decrypted privkey_context size: ${privkeyContext.length} (expected 64 bytes)`
		);
	}

	return privkeyContext;
}
