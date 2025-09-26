/**
 * Session Database Module - IndexedDB Core Operations
 *
 * Single Responsibility: Handle core IndexedDB operations for session management
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

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

	// Auth tokens (cleared on auth logout)
	auth_user: {
		user_id: string;
		isAuthenticated: boolean;
	} | null;
	access_token: string | null;

	// Ed25519 server public key for signed response validation (cleared on auth logout)
	server_pub_key: string | null;

	// User Preferences (persistent across auth sessions)
	userPreferences: {
		language: string | null; // preferred-language (null = auto-detect)
		theme: 'light' | 'dark' | null; // theme (null = system preference)
	};

	// Auth Flow Data (temporary, cleared after auth completion)
	authFlow: {
		pending_email: string | null; // pending_auth_email
	};

	// Metadata
	created: number;
	lastAccessed: number;
}

/**
 * Core IndexedDB manager for session data
 */
class SessionDB {
	private dbName = 'hashrand-app-session';
	private storeName = 'session-data';
	private version = 1;
	private sessionKey = 'app-session';

	/**
	 * Get IndexedDB database connection
	 */
	async getDB(): Promise<IDBDatabase> {
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
		} catch {
			// Failed to get session from IndexedDB
			return this.createEmptySession();
		}
	}

	/**
	 * Save session data to IndexedDB
	 */
	async saveSession(session: AppSessionData): Promise<void> {
		session.lastAccessed = Date.now();

		const db = await this.getDB();
		const transaction = db.transaction([this.storeName], 'readwrite');
		const store = transaction.objectStore(this.storeName);

		return new Promise((resolve, reject) => {
			const request = store.put(session, this.sessionKey);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => resolve();
		});
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
	 * Clear ALL session data
	 */
	async clearSession(): Promise<void> {
		const db = await this.getDB();
		const transaction = db.transaction([this.storeName], 'readwrite');
		const store = transaction.objectStore(this.storeName);

		return new Promise((resolve, reject) => {
			const request = store.delete(this.sessionKey);

			request.onerror = () => reject(request.error);
			request.onsuccess = () => {
				// IndexedDB session cleared completely
				resolve();
			};
		});
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
			server_pub_key: null,
			userPreferences: {
				language: null, // Auto-detect browser language
				theme: null // Use system preference
			},
			authFlow: {
				pending_email: null
			},
			created: Date.now(),
			lastAccessed: Date.now()
		};
	}
}

// Export singleton instance
export const sessionDB = new SessionDB();
