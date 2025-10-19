/**
 * Ed25519 Module - Refactored with SOLID Principles
 *
 * Simplified main Ed25519 module using specialized modules for different responsibilities.
 * Provides unified digital signature interface with clean imports.
 */

// Re-export all functions from specialized modules for backward compatibility
export type { Ed25519KeyPair } from './ed25519/index';
export {
	generateEd25519KeyPair,
	storeKeyPair,
	getKeyPair,
	clearAllKeyPairs,
	signMessage,
	signMessageWithKeyPair,
	verifySignature,
	publicKeyToHex,
	publicKeyFromHex,
	getOrCreateKeyPair
} from './ed25519/index';
