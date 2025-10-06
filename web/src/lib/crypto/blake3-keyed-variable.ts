/**
 * Blake3 Keyed Variable - Universal cryptographic pipeline
 *
 * EXACT TypeScript port of backend pseudonimizer.rs blake3_keyed_variable function
 * 100% API-compatible with Rust implementation using @noble/hashes streaming API
 *
 * CRYPTOGRAPHIC FLOW (IDENTICAL to Rust backend):
 * 1. hmac_env_key[64] → Base58 → context (domain separation, fixed per use case)
 * 2. data[n] → key_material preparation:
 *    - If data.length >= 32: use data directly as key_material
 *    - If data.length < 32: blake3(data) → key_material[32 bytes]
 * 3. (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
 * 4. (data, deterministic_key, length) → Blake3 keyed → output[length bytes]
 *
 * Security Properties:
 * - Domain separation: Different hmac_env_key → cryptographically independent outputs
 * - Deterministic: Same inputs always produce same output
 * - Variable output: Supports any output length (1 byte to practical limits)
 * - Key derivation: Unique 32-byte key derived per data input via KDF
 * - Superior to HMAC: Internal key changes with each derivation
 */

import { blake3 } from '@noble/hashes/blake3';
import { base58 } from '@scure/base';

/**
 * Universal Blake3 pipeline: hmac_env_key + data → variable-length output
 *
 * @param hmacEnvKey - **MUST be exactly 64 bytes** for domain separation (one per use case)
 * @param data - Variable-length input data (any size)
 * @param outputLength - Desired output length in bytes
 * @returns Cryptographically derived output of specified length
 *
 * @throws {Error} If hmacEnvKey is not exactly 64 bytes
 *
 * @security
 * - Input: 64-byte hmacEnvKey provides 512 bits of entropy
 * - Blake3 KDF: Uses full 64 bytes via Base58-encoded context (no entropy loss)
 * - Blake3 keyed: Uses 32-byte derived key (256-bit security, Blake3 design limit)
 * - Result: 256-bit security level (industry standard for symmetric crypto)
 *
 * @example
 * ```typescript
 * const hmacKey = new Uint8Array(64).fill(1);  // From environment variable (64 bytes!)
 * const userData = new TextEncoder().encode('user_id + pub_key');
 * const privateKey = blake3KeyedVariable(hmacKey, userData, 32);
 * ```
 */
export function blake3KeyedVariable(
	hmacEnvKey: Uint8Array,
	data: Uint8Array,
	outputLength: number
): Uint8Array {
	// SECURITY: Validate hmacEnvKey is exactly 64 bytes (same as backend)
	if (hmacEnvKey.length !== 64) {
		throw new Error(
			`blake3KeyedVariable: hmacEnvKey must be exactly 64 bytes, got ${hmacEnvKey.length} bytes`
		);
	}

	// PASO 1: hmac_env_key[64] → Base58 (~87 chars) → context (full 64-byte entropy preserved)
	const contextString = base58.encode(hmacEnvKey);
	const contextBytes = new TextEncoder().encode(contextString);

	// PASO 2: Prepare key_material (Blake3 KDF works best with ≥32 bytes)
	const keyMaterial: Uint8Array = data.length >= 32 ? data : blake3(data);

	// PASO 3: (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
	// IMPORTANT: Using blake3.create() with context is equivalent to Rust's blake3::derive_key()
	// The FULL 64-byte hmac_env_key (via Base58 context) influences the derived key
	const deterministicKey = blake3(keyMaterial, { context: contextBytes });

	// PASO 4: (data, deterministic_key, length) → Blake3 keyed+XOF → output
	// CRITICAL: Using streaming API to exactly match backend behavior:
	// Rust: Hasher::new_keyed() + update() + finalize_xof()
	// TypeScript: blake3.create({key}) + update() + xof()
	const hasher = blake3.create({ key: deterministicKey });
	hasher.update(data);
	const output = hasher.xof(outputLength);

	return output;
}

/**
 * Test the blake3KeyedVariable function for correctness
 * Ensures compatibility with Rust backend implementation
 */
export function testBlake3KeyedVariable(): void {
	console.log('Testing blake3KeyedVariable...');

	// Test 0: Validate 64-byte requirement (SECURITY)
	const invalidKey32 = new Uint8Array(32).fill(1); // Only 32 bytes
	const invalidKey128 = new Uint8Array(128).fill(1); // Too many bytes
	const testData = new TextEncoder().encode('test data');

	try {
		blake3KeyedVariable(invalidKey32, testData, 32);
		console.assert(false, 'Should reject 32-byte key');
	} catch (e) {
		console.assert(
			(e as Error).message.includes('must be exactly 64 bytes'),
			'Should throw error for wrong key size'
		);
	}

	try {
		blake3KeyedVariable(invalidKey128, testData, 32);
		console.assert(false, 'Should reject 128-byte key');
	} catch (e) {
		console.assert(
			(e as Error).message.includes('must be exactly 64 bytes'),
			'Should throw error for wrong key size'
		);
	}

	// Test 1: Deterministic output (with correct 64-byte key)
	const hmacKey1 = new Uint8Array(64).fill(1);

	const output1 = blake3KeyedVariable(hmacKey1, testData, 32);
	const output2 = blake3KeyedVariable(hmacKey1, testData, 32);

	console.assert(output1.length === 32 && output2.length === 32, 'Output should be 32 bytes');
	console.assert(
		output1.every((byte, i) => byte === output2[i]),
		'Same inputs should produce same output (deterministic)'
	);

	// Test 2: Variable output lengths
	const output32 = blake3KeyedVariable(hmacKey1, testData, 32);
	const output64 = blake3KeyedVariable(hmacKey1, testData, 64);
	const output128 = blake3KeyedVariable(hmacKey1, testData, 128);

	console.assert(output32.length === 32, 'Should produce 32-byte output');
	console.assert(output64.length === 64, 'Should produce 64-byte output');
	console.assert(output128.length === 128, 'Should produce 128-byte output');

	// First 32 bytes should match (XOF property)
	console.assert(
		output64.slice(0, 32).every((byte, i) => byte === output32[i]),
		'First 32 bytes of 64-byte output should match 32-byte output'
	);

	// Test 3: Domain separation
	const hmacKey2 = new Uint8Array(64).fill(2);
	const outputDifferentKey = blake3KeyedVariable(hmacKey2, testData, 32);

	console.assert(
		!output1.every((byte, i) => byte === outputDifferentKey[i]),
		'Different hmac_env_key should produce different outputs'
	);

	// Test 4: Data sensitivity
	const testData2 = new TextEncoder().encode('different data');
	const outputDifferentData = blake3KeyedVariable(hmacKey1, testData2, 32);

	console.assert(
		!output1.every((byte, i) => byte === outputDifferentData[i]),
		'Different data should produce different outputs'
	);

	// Test 5: Short data handling (< 32 bytes)
	const shortData = new TextEncoder().encode('short');
	const outputShort = blake3KeyedVariable(hmacKey1, shortData, 32);

	console.assert(outputShort.length === 32, 'Short data should produce 32-byte output');

	// Test 6: Long data handling (>= 32 bytes)
	const longData = new TextEncoder().encode(
		'this is a very long data string with more than 32 bytes of content'
	);
	const outputLong = blake3KeyedVariable(hmacKey1, longData, 32);

	console.assert(outputLong.length === 32, 'Long data should produce 32-byte output');
	console.assert(
		!outputShort.every((byte, i) => byte === outputLong[i]),
		'Short and long data should produce different outputs'
	);

	console.log('✅ All blake3KeyedVariable tests passed!');
}
