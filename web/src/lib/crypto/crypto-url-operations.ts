/**
 * Crypto URL Operations Module - URL Parameter Encryption/Decryption
 *
 * Single Responsibility: Core URL parameter encryption/decryption using ChaCha20-Poly1305
 * Part of crypto.ts refactorization to apply SOLID principles
 */

import { chacha20poly1305 } from '@noble/ciphers/chacha.js';
import {
	generatePrehash,
	generateCipherKey,
	generateCipherNonce,
	generateCryptoSalt,
	generatePrehashSeed
} from './crypto-core';
import { bytesToBase64Url, base64UrlToBytes, bytesToBase64 } from './crypto-encoding';
import { storePrehashSeed, getPrehashSeed } from './crypto-storage';

/**
 * Serialize parameters object to a consistent string representation
 *
 * @param params - Parameters object to serialize
 * @returns Consistent string representation
 */
export function serializeParams(params: Record<string, unknown>): string {
	// Sort keys for consistent output
	const sortedKeys = Object.keys(params).sort();
	const sortedParams: Record<string, unknown> = {};

	for (const key of sortedKeys) {
		sortedParams[key] = params[key];
	}

	return JSON.stringify(sortedParams);
}

/**
 * Encrypt parameters for secure URL transmission using prehash seed
 * Returns single compact parameter 'p' containing idx_bytes + encrypted_bytes
 *
 * @param params - Parameters object to encrypt
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Promise<Single compact parameter 'p' as base64URL string>
 */
export async function encryptUrlParams(
	params: Record<string, unknown>,
	cipherToken: string,
	nonceToken: string,
	hmacKey: string
): Promise<{ p: string }> {
	// Starting URL parameter encryption

	// 1. Add crypto salt to parameters for noise
	const salt = generateCryptoSalt();
	const saltBase64 = bytesToBase64(salt);
	const paramsWithSalt = { ...params, _salt: saltBase64 };

	// 2. Generate random prehash seed (independent of content)
	const prehashSeed = generatePrehashSeed();

	// 3. Store prehash seed and get key (8 bytes)
	const idx = await storePrehashSeed(prehashSeed, hmacKey);
	const idxBytes = base64UrlToBytes(idx); // Convert idx back to 8 bytes

	// 4. Generate prehash from seed
	const prehash = generatePrehash(prehashSeed, hmacKey);

	// 5. Generate cipher key and nonce for ChaCha20-Poly1305
	const cipherKey = generateCipherKey(cipherToken, prehash);
	const cipherNonce = generateCipherNonce(nonceToken, prehash);

	// 6. Encrypt params (with salt) using ChaCha20-Poly1305
	const paramsString = serializeParams(paramsWithSalt);
	const cipher = chacha20poly1305(cipherKey, cipherNonce);
	const plaintext = new TextEncoder().encode(paramsString);
	const ciphertext = cipher.encrypt(plaintext);

	// 7. Concatenate idx_bytes (8 bytes) + encrypted_bytes and encode as base64URL
	const combined = new Uint8Array(idxBytes.length + ciphertext.length);
	combined.set(idxBytes, 0);
	combined.set(ciphertext, idxBytes.length);

	const result = {
		p: bytesToBase64Url(combined)
	};

	// Parameter "p" generated successfully

	return result;
}

/**
 * Decrypt parameters from secure URL transmission using compact 'p' parameter
 * Extracts idx (first 8 bytes) and encrypted data (remaining bytes)
 *
 * @param compactParam - Compact parameter 'p' containing idx_bytes + encrypted_bytes
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Promise<Decrypted parameters object (without salt)>
 */
export async function decryptUrlParams(
	compactParam: string,
	cipherToken: string,
	nonceToken: string,
	hmacKey: string
): Promise<Record<string, unknown>> {
	// 1. Decode base64URL to get combined bytes
	const combinedBytes = base64UrlToBytes(compactParam);

	// 2. Extract idx_bytes (first 8 bytes) and encrypted_bytes (remaining)
	if (combinedBytes.length < 8) {
		throw new Error('Invalid compact parameter: too short for idx extraction');
	}

	const idxBytes = combinedBytes.slice(0, 8);
	const encryptedBytes = combinedBytes.slice(8);

	// 3. Convert idx_bytes back to base64URL string for IndexedDB lookup
	const idx = bytesToBase64Url(idxBytes);

	// 4. Retrieve prehash seed from IndexedDB
	const prehashSeed = await getPrehashSeed(idx);
	if (!prehashSeed) {
		throw new Error(`Prehash seed not found with key ${idx}`);
	}

	// 5. Regenerate prehash from seed
	const prehash = generatePrehash(prehashSeed, hmacKey);

	// 6. Regenerate cipher key and nonce
	const cipherKey = generateCipherKey(cipherToken, prehash);
	const cipherNonce = generateCipherNonce(nonceToken, prehash);

	// 7. Decrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, cipherNonce);
	const plaintext = cipher.decrypt(encryptedBytes);

	// 8. Convert to string and parse JSON
	const paramsString = new TextDecoder().decode(plaintext);
	const paramsWithSalt = JSON.parse(paramsString);

	// 9. Remove internal salt and return clean params
	const { _salt, ...params } = paramsWithSalt;
	return params;
}
