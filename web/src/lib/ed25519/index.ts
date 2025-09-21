/**
 * Ed25519 Module Index - Centralized Exports
 *
 * Provides clean imports for all Ed25519 modules
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

// Type definitions
export type { Ed25519KeyPair } from './ed25519-types';

// Key generation operations
export { generateEd25519KeyPair, generateEd25519KeyPairFallback } from './ed25519-keygen';

// Database operations
export { storeKeyPair, getKeyPair, clearAllKeyPairs } from './ed25519-database';

// Signing operations
export { signMessage, verifySignature } from './ed25519-signing';

// Utility functions
export { publicKeyToHex, publicKeyFromHex } from './ed25519-utils';

// High-level API
export { getOrCreateKeyPair } from './ed25519-api';
