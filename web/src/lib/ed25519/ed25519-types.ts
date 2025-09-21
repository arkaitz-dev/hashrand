/**
 * Ed25519 Types Module - Type Definitions
 *
 * Single Responsibility: Define all types and interfaces for Ed25519 operations
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

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
