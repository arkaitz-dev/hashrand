/**
 * Crypto Module - Refactored with SOLID Principles
 *
 * Simplified main crypto module using specialized modules for different responsibilities.
 * Provides unified cryptographic interface with clean imports.
 */

// Re-export all functions from specialized modules for backward compatibility
export {
	cryptoHashGen,
	generatePrehash,
	generateCipherKey,
	generateCipherNonce,
	generateCryptoSalt,
	generatePrehashSeed,
	bytesToBase64,
	bytesToBase64Url,
	base64ToBytes,
	base64UrlToBytes,
	storePrehashSeed,
	getPrehashSeed,
	serializeParams,
	encryptUrlParams,
	decryptUrlParams,
	prepareSecureUrlParams,
	parseNextUrl,
	encryptNextUrl,
	decryptPageParams,
	createEncryptedUrl
} from './crypto/index';
