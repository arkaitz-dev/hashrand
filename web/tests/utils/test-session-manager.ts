/**
 * Test Session Manager - In-Memory Implementation for E2E Tests
 *
 * Mimics sessionManager API without IndexedDB dependency
 * Allows E2E tests to manage session state without browser storage
 */

import type { Ed25519KeyPair } from '../../src/lib/ed25519/ed25519-types';
import {
	generateKeyPairNoble,
	keyPairToHex,
	keyPairFromHex
} from '../../src/lib/ed25519/ed25519-core';

/**
 * In-memory session manager for Playwright tests
 * Provides same interface as browser sessionManager but without IndexedDB
 */
export class TestSessionManager {
	private keyPair: Ed25519KeyPair | null = null;
	private serverPubKey: string | null = null;
	private accessToken: string | null = null;
	private userId: string | null = null;
	private cryptoTokens: {
		cipher: string | null;
		nonce: string | null;
		hmac: string | null;
	} = { cipher: null, nonce: null, hmac: null };

	/**
	 * Generate new Ed25519 keypair using Noble curves (universal)
	 */
	async generateKeyPair(): Promise<Ed25519KeyPair> {
		this.keyPair = generateKeyPairNoble();
		return this.keyPair;
	}

	/**
	 * Get current keypair
	 */
	async getKeyPair(): Promise<Ed25519KeyPair | null> {
		return this.keyPair;
	}

	/**
	 * Set keypair from hex strings (for key rotation)
	 */
	async setKeyPairFromHex(privateKeyHex: string, publicKeyHex: string): Promise<void> {
		this.keyPair = keyPairFromHex(privateKeyHex, publicKeyHex);
	}

	/**
	 * Get keypair as hex strings
	 */
	async getKeyPairHex(): Promise<{ privateKeyHex: string; publicKeyHex: string } | null> {
		if (!this.keyPair || !this.keyPair.privateKeyBytes) {
			return null;
		}
		return keyPairToHex(this.keyPair);
	}

	/**
	 * Set server public key
	 */
	async setServerPubKey(pubKey: string): Promise<void> {
		this.serverPubKey = pubKey;
	}

	/**
	 * Get server public key
	 */
	async getServerPubKey(): Promise<string | null> {
		return this.serverPubKey;
	}

	/**
	 * Set auth data (user + access token)
	 */
	async setAuthData(userId: string, accessToken: string): Promise<void> {
		this.userId = userId;
		this.accessToken = accessToken;
	}

	/**
	 * Get auth data
	 */
	async getAuthData(): Promise<{
		user: { user_id: string; isAuthenticated: boolean } | null;
		access_token: string | null;
	}> {
		if (!this.userId || !this.accessToken) {
			return { user: null, access_token: null };
		}

		return {
			user: { user_id: this.userId, isAuthenticated: true },
			access_token: this.accessToken
		};
	}

	/**
	 * Set crypto tokens for URL encryption
	 */
	async setCryptoTokens(cipher: string, nonce: string, hmac: string): Promise<void> {
		this.cryptoTokens = { cipher, nonce, hmac };
	}

	/**
	 * Get crypto tokens
	 */
	async getCryptoTokens(): Promise<{
		cipher: string | null;
		nonce: string | null;
		hmac: string | null;
	}> {
		return this.cryptoTokens;
	}

	/**
	 * Check if crypto tokens exist
	 */
	async hasCryptoTokens(): Promise<boolean> {
		return !!(
			this.cryptoTokens.cipher &&
			this.cryptoTokens.nonce &&
			this.cryptoTokens.hmac
		);
	}

	/**
	 * Clear all session data (logout)
	 */
	async clear(): Promise<void> {
		this.keyPair = null;
		this.serverPubKey = null;
		this.accessToken = null;
		this.userId = null;
		this.cryptoTokens = { cipher: null, nonce: null, hmac: null };
	}
}
