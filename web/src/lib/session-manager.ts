//! Unified Session Manager with IndexedDB
//!
//! Migrates ALL session data from sessionStorage to IndexedDB for:
//! - Cross-tab consistency
//! - Better UX (no re-auth per tab)
//! - Unified crypto + auth session management

export interface AppSessionData {
	// Crypto tokens para URL parameter encryption
	cipher_token: string | null;
	nonce_token: string | null;
	hmac_key: string | null;

	// FIFO KV store para prehashseeds (límite 20)
	prehashSeeds: Array<{
		key: string; // idx (8 bytes Base64URL)
		prehashSeed: string; // 32 bytes para descifrado
		timestamp: number; // Para FIFO rotation
	}>;

	// Auth tokens
	auth_user: {
		user_id: string;
		isAuthenticated: boolean;
	} | null;
	access_token: string | null;
	token_expires_at: number | null;

	// Metadata
	created: number;
	lastAccessed: number;
}

class SessionManager {
	private dbName = 'hashrand-app-session';
	private storeName = 'session-data';
	private version = 1;
	private sessionKey = 'app-session';

	/**
	 * Get IndexedDB database connection
	 */
	private async getDB(): Promise<IDBDatabase> {
		return new Promise((resolve, reject) => {
			const request = indexedDB.open(this.dbName, this.version);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve(request.result);

			request.onupgradeneeded = (event) => {
				const db = (event.target as IDBOpenDBRequest).result;
				if (!db.objectStoreNames.contains(this.storeName)) {
					db.createObjectStore(this.storeName);
				}
			};
		});
	}

	/**
	 * Get current session data from IndexedDB
	 */
	async getSession(): Promise<AppSessionData> {
		try {
			const db = await this.getDB();
			const transaction = db.transaction([this.storeName], 'readonly');
			const store = transaction.objectStore(this.storeName);

			return new Promise((resolve, reject) => {
				const request = store.get(this.sessionKey);

				request.onerror = () => reject(request.error);
				request.onsuccess = () => {
					const session = request.result || this.createEmptySession();
					// Update lastAccessed
					session.lastAccessed = Date.now();
					resolve(session);
				};
			});
		} catch (error) {
			console.warn('Failed to get session from IndexedDB:', error);
			return this.createEmptySession();
		}
	}

	/**
	 * Save session data to IndexedDB
	 */
	async saveSession(session: AppSessionData): Promise<void> {
		try {
			session.lastAccessed = Date.now();

			const db = await this.getDB();
			const transaction = db.transaction([this.storeName], 'readwrite');
			const store = transaction.objectStore(this.storeName);

			return new Promise((resolve, reject) => {
				const request = store.put(session, this.sessionKey);

				request.onerror = () => reject(request.error);
				request.onsuccess = () => resolve();
			});
		} catch (error) {
			console.warn('Failed to save session to IndexedDB:', error);
			throw error;
		}
	}

	/**
	 * Update specific part of session
	 */
	async updateSession(updates: Partial<AppSessionData>): Promise<void> {
		const session = await this.getSession();
		const updatedSession = { ...session, ...updates };
		await this.saveSession(updatedSession);
	}

	/**
	 * Create empty session structure
	 */
	private createEmptySession(): AppSessionData {
		return {
			cipher_token: null,
			nonce_token: null,
			hmac_key: null,
			prehashSeeds: [],
			auth_user: null,
			access_token: null,
			token_expires_at: null,
			created: Date.now(),
			lastAccessed: Date.now()
		};
	}

	/**
	 * Clear ALL session data (logout + dual expiry)
	 */
	async clearSession(): Promise<void> {
		try {
			const db = await this.getDB();
			const transaction = db.transaction([this.storeName], 'readwrite');
			const store = transaction.objectStore(this.storeName);

			return new Promise((resolve, reject) => {
				const request = store.delete(this.sessionKey);

				request.onerror = () => reject(request.error);
				request.onsuccess = () => {
					console.log('🧹 IndexedDB session cleared completely');
					resolve();
				};
			});
		} catch (error) {
			console.warn('Failed to clear session from IndexedDB:', error);
			throw error;
		}
	}

	/**
	 * Migrate existing sessionStorage data to IndexedDB (one-time)
	 */
	async migrateFromSessionStorage(): Promise<void> {
		if (typeof window === 'undefined') return;

		console.log('🔄 Migrating session data from sessionStorage to IndexedDB...');

		const session = await this.getSession();

		// Migrate crypto tokens
		session.cipher_token = sessionStorage.getItem('cipher_token');
		session.nonce_token = sessionStorage.getItem('nonce_token');
		session.hmac_key = sessionStorage.getItem('hmac_key');

		// Migrate combined crypto_tokens format if exists
		const cryptoTokens = sessionStorage.getItem('crypto_tokens');
		if (cryptoTokens) {
			try {
				const tokens = JSON.parse(cryptoTokens);
				session.cipher_token = tokens.cipher || session.cipher_token;
				session.nonce_token = tokens.nonce || session.nonce_token;
				session.hmac_key = tokens.hmacKey || session.hmac_key;
			} catch (error) {
				console.warn('Failed to parse crypto_tokens:', error);
			}
		}

		// Migrate prehashseeds
		const prehashSeedsJson = sessionStorage.getItem('prehashseeds');
		if (prehashSeedsJson) {
			try {
				const seeds = JSON.parse(prehashSeedsJson);
				session.prehashSeeds = seeds.map((seed: { k: string; v: string }) => ({
					key: seed.k,
					prehashSeed: seed.v,
					timestamp: Date.now()
				}));
			} catch (error) {
				console.warn('Failed to parse prehashseeds:', error);
			}
		}

		// Migrate auth data
		const authUser = sessionStorage.getItem('auth_user');
		if (authUser) {
			try {
				session.auth_user = JSON.parse(authUser);
			} catch (error) {
				console.warn('Failed to parse auth_user:', error);
			}
		}

		session.access_token = sessionStorage.getItem('access_token');

		// Save migrated session
		await this.saveSession(session);

		// Clear sessionStorage after successful migration
		this.clearSessionStorage();

		console.log('✅ Migration from sessionStorage to IndexedDB completed');
	}

	/**
	 * Initialize with automatic migration
	 */
	async init(): Promise<void> {
		// Check if we need to migrate from sessionStorage
		const hasSessionData =
			typeof window !== 'undefined' &&
			(sessionStorage.getItem('access_token') ||
				sessionStorage.getItem('auth_user') ||
				sessionStorage.getItem('cipher_token') ||
				sessionStorage.getItem('crypto_tokens'));

		if (hasSessionData) {
			console.log('🔄 Detected sessionStorage data, performing one-time migration...');
			await this.migrateFromSessionStorage();
		}
	}

	/**
	 * Clear sessionStorage (migration cleanup)
	 */
	private clearSessionStorage(): void {
		if (typeof window === 'undefined') return;

		const keysToRemove = [
			'cipher_token',
			'nonce_token',
			'hmac_key',
			'crypto_tokens',
			'prehashseeds',
			'auth_user',
			'access_token'
		];

		keysToRemove.forEach((key) => {
			sessionStorage.removeItem(key);
		});
	}

	/**
	 * Get crypto tokens for URL encryption
	 */
	async getCryptoTokens(): Promise<{
		cipher: string | null;
		nonce: string | null;
		hmac: string | null;
	}> {
		const session = await this.getSession();
		return {
			cipher: session.cipher_token,
			nonce: session.nonce_token,
			hmac: session.hmac_key
		};
	}

	/**
	 * Set crypto tokens for URL encryption
	 */
	async setCryptoTokens(cipher: string, nonce: string, hmac: string): Promise<void> {
		await this.updateSession({
			cipher_token: cipher,
			nonce_token: nonce,
			hmac_key: hmac
		});
	}

	/**
	 * Check if crypto tokens exist
	 */
	async hasCryptoTokens(): Promise<boolean> {
		const tokens = await this.getCryptoTokens();
		return !!(tokens.cipher && tokens.nonce && tokens.hmac);
	}

	/**
	 * Add prehash seed to FIFO store
	 */
	async addPrehashSeed(key: string, prehashSeed: string): Promise<void> {
		const session = await this.getSession();

		// Add new seed
		session.prehashSeeds.push({
			key,
			prehashSeed,
			timestamp: Date.now()
		});

		// Enforce FIFO limit of 20
		if (session.prehashSeeds.length > 20) {
			session.prehashSeeds = session.prehashSeeds.slice(-20);
		}

		await this.saveSession(session);
	}

	/**
	 * Get prehash seed by key
	 */
	async getPrehashSeed(key: string): Promise<string | null> {
		const session = await this.getSession();
		const seedEntry = session.prehashSeeds.find((entry) => entry.key === key);
		return seedEntry ? seedEntry.prehashSeed : null;
	}

	/**
	 * Get auth data
	 */
	async getAuthData(): Promise<{
		user: { user_id: string; isAuthenticated: boolean } | null;
		access_token: string | null;
		expires_at: number | null;
	}> {
		const session = await this.getSession();
		return {
			user: session.auth_user,
			access_token: session.access_token,
			expires_at: session.token_expires_at
		};
	}

	/**
	 * Set auth data
	 */
	async setAuthData(
		user: { user_id: string; isAuthenticated: boolean },
		access_token: string,
		expires_at?: number
	): Promise<void> {
		await this.updateSession({
			auth_user: user,
			access_token,
			token_expires_at: expires_at || null
		});
	}

	/**
	 * Check if user is authenticated
	 */
	async isAuthenticated(): Promise<boolean> {
		const authData = await this.getAuthData();
		return !!(authData.user?.isAuthenticated && authData.access_token);
	}
}

// Export singleton instance
export const sessionManager = new SessionManager();
