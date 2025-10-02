/**
 * API Auth Operations Module - Authentication Endpoints
 *
 * Single Responsibility: Handle authentication-related API operations
 * Part of api.ts refactorization to apply SRP and organize by domain
 */

import type { LoginResponse, MagicLinkResponse } from '../types';

const API_BASE = '/api';

/**
 * Request magic link for email authentication
 */
export async function requestMagicLink(
	email: string,
	ui_host: string,
	next: string = '/'
): Promise<MagicLinkResponse> {
	// Generate or retrieve Ed25519 keypair
	const { getOrCreateKeyPair, publicKeyToHex } = await import('../ed25519');
	const keyPair = await getOrCreateKeyPair();
	const pubKeyHex = publicKeyToHex(keyPair.publicKeyBytes);

	// Get current language for email template (REQUIRED)
	let email_lang: string = 'en'; // Default fallback
	try {
		const { currentLanguage } = await import('../stores/i18n');
		const { get } = await import('svelte/store');
		email_lang = get(currentLanguage);
		// Email language from i18n store
	} catch {
		// Fallback to browser language detection
		if (typeof navigator !== 'undefined') {
			email_lang = navigator.language.split('-')[0].toLowerCase();
			// Email language from browser fallback
		} else {
			// Email language using default fallback
		}
	}

	// Create payload for SignedRequest
	const payload = {
		email,
		ui_host,
		next,
		email_lang,
		pub_key: pubKeyHex
	};

	// Use universal signed POST request (first signed response to extract server_pub_key)
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
	return await httpSignedPOSTRequest<typeof payload, MagicLinkResponse>(
		`${API_BASE}/login/`,
		payload,
		true
	);
}

/**
 * Validate magic link and complete authentication with SignedResponse handling
 * SECURITY: Uses credentials: 'include' to receive HttpOnly refresh token cookie
 */
export async function validateMagicLink(magicToken: string): Promise<LoginResponse> {
	// Use universal signed POST request with magic link payload
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');

	console.log(
		'🍪 [SECURITY] validateMagicLink: Sending request WITH credentials to receive cookie'
	);

	return await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
		`${API_BASE}/login/magiclink/`,
		{ magiclink: magicToken },
		false,
		{ credentials: 'include' } // CRITICAL: Must receive HttpOnly refresh cookie
	);
}

/**
 * Check authentication status
 */
export async function checkAuthStatus(): Promise<boolean> {
	// Check if we have both user info and access token in IndexedDB
	try {
		const { sessionManager } = await import('../session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.user || !authData.access_token) return false;

		return authData.user.isAuthenticated && !!authData.user.user_id;
	} catch {
		return false;
	}
}

/**
 * Logout user and clear server-side session
 */
export async function logout(): Promise<void> {
	// Call backend to clear HttpOnly refresh token cookie using authenticated signed DELETE request
	// Backend validates Ed25519 signature and emits SignedResponse (Zero Knowledge complete chain)
	try {
		const { httpSignedAuthenticatedDELETE } = await import('../httpSignedRequests');
		await httpSignedAuthenticatedDELETE<{ message: string }>(`${API_BASE}/login`);
	} catch {
		// Failed to clear refresh token cookie
		// Continue with logout even if cookie clearing fails
	}
}

/**
 * Try to refresh the access token using the HttpOnly refresh token cookie
 *
 * 🔄 KEY ROTATION LOGIC:
 * - ALWAYS generates new Ed25519 keypair before refresh request
 * - Backend determines rotation based on 2/3 time window:
 *   - Tramo 1/3 (0 to 1/3 duration): Returns access_token only, NO server_pub_key → No rotation
 *   - Tramo 2/3 (1/3 to full duration): Returns both tokens + server_pub_key → Full rotation
 * - Frontend rotates keys ONLY if server_pub_key is present in response
 *
 * Token durations: Configured in .env (SPIN_VARIABLE_*_TOKEN_DURATION_MINUTES)
 * Backend: api/src/utils/jwt/config.rs::get_refresh_token_duration_minutes()
 */
export async function refreshToken(): Promise<boolean> {
	// Import all dependencies at the top to avoid redeclarations
	const { flashMessagesStore } = await import('../stores/flashMessages');
	const { sessionManager } = await import('../session-manager');
	const { generateEd25519KeyPairFallback, publicKeyToHex } = await import('../ed25519');
	const { privateKeyBytesToHex } = await import('../ed25519/ed25519-core');
	const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
	const { authStore } = await import('../stores/auth');

	try {
		console.log('🔄 [REFRESH] ===== INICIO REFRESH TOKEN =====');
		flashMessagesStore.addMessage('🔄 Iniciando renovación de token...');

		// Get OLD pub_key from IndexedDB for logging
		const oldPrivKey = await sessionManager.getPrivKey();
		if (oldPrivKey) {
			console.log('🔑 [REFRESH] OLD priv_key actual:', oldPrivKey.substring(0, 16) + '...');
		}

		// 🔑 STEP 1: Generate NEW Ed25519 keypair for potential rotation
		console.log('🔑 [REFRESH] STEP 1: Generando nuevo keypair Ed25519...');
		const newKeyPair = await generateEd25519KeyPairFallback();
		const newPubKeyHex = publicKeyToHex(newKeyPair.publicKeyBytes);
		const newPrivKeyHex = privateKeyBytesToHex(newKeyPair.privateKeyBytes!);

		console.log('✅ [REFRESH] Nuevo keypair generado');
		console.log('🔑 [REFRESH] NEW priv_key:', newPrivKeyHex.substring(0, 16) + '...');
		console.log('🔑 [REFRESH] NEW pub_key:', newPubKeyHex.substring(0, 16) + '...');
		flashMessagesStore.addMessage('🔑 Nuevo keypair generado para rotación');

		// 🔒 STEP 2: Send refresh request with new_pub_key
		console.log('📤 [REFRESH] STEP 2: Enviando request a /api/refresh...');
		console.log('📦 [REFRESH] Payload: { new_pub_key:', newPubKeyHex.substring(0, 16) + '... }');
		flashMessagesStore.addMessage('📤 Enviando request a /api/refresh...');

		const data = await httpSignedPOSTRequest<{ new_pub_key: string }, LoginResponse>(
			`${API_BASE}/refresh`,
			{ new_pub_key: newPubKeyHex },
			false,
			{ credentials: 'include' }
		);

		console.log('📥 [REFRESH] Respuesta recibida del servidor');
		console.log('📊 [REFRESH] Response data:', {
			has_access_token: !!data.access_token,
			has_server_pub_key: !!data.server_pub_key,
			has_expires_at: !!data.expires_at
		});
		flashMessagesStore.addMessage('📥 Respuesta recibida del servidor');

		// 📝 STEP 3: Update auth store with new token
		console.log('📝 [REFRESH] STEP 3: Actualizando store con nuevo access_token...');

		const user = {
			user_id: data.user_id,
			isAuthenticated: true
		};

		// Update store and IndexedDB
		authStore.updateTokens(user, data.access_token);
		console.log('✅ [REFRESH] Access token actualizado en store');

		// ⏱️ STEP 4: Update session expiration timestamp if provided (new refresh cookie issued)
		console.log('⏱️ [REFRESH] STEP 4: Verificando expires_at...');
		if (data.expires_at) {
			console.log('✅ [REFRESH] expires_at presente:', data.expires_at);
			try {
				const { storeSessionExpiration } = await import('../session-storage');
				await storeSessionExpiration(data.expires_at);
			} catch (error) {
				console.warn('Failed to store session expiration during refresh:', error);
				// Non-blocking - refresh continues without persistent expiration tracking
			}
		}

		// 🔄 STEP 5: CONDITIONAL KEY ROTATION
		// NOTE: universalSignedResponseHandler already updated server_pub_key in IndexedDB (if present)
		console.log('🔄 [REFRESH] STEP 5: Verificando server_pub_key para rotación...');
		if (data.server_pub_key) {
			// ✅ TRAMO 2/3: Backend sent server_pub_key → Full key rotation
			console.log('🔄 [REFRESH] ===== TRAMO 2/3: KEY ROTATION =====');
			console.log(
				'🔑 [REFRESH] server_pub_key recibido:',
				data.server_pub_key.substring(0, 16) + '...'
			);
			flashMessagesStore.addMessage('🔄 TRAMO 2/3: Iniciando rotación de claves...');

			// Rotate client keypair to match NEW server keypair
			console.log('🔑 [REFRESH] Rotando client priv_key en IndexedDB...');
			await sessionManager.setPrivKey(newPrivKeyHex);
			console.log('✅ [REFRESH] Client priv_key rotado:', newPrivKeyHex.substring(0, 16) + '...');

			// server_pub_key already updated by universalSignedResponseHandler (secure validation flow)
			console.log('✅ [REFRESH] Server pub_key ya actualizado por validador (seguro)');

			console.log('🎉 [REFRESH] Rotación de claves completada exitosamente');
			flashMessagesStore.addMessage('✅ Rotación de claves completada (2/3)');
		} else {
			// ⏭️ TRAMO 1/3: No server_pub_key → Keep existing keys, only token renewed
			console.log('⏭️ [REFRESH] ===== TRAMO 1/3: NO KEY ROTATION =====');
			console.log('ℹ️ [REFRESH] No server_pub_key en respuesta - manteniendo claves existentes');
			flashMessagesStore.addMessage('⏭️ Token renovado sin rotación (1/3)');
		}

		// Note: Crypto tokens are NOT generated during refresh
		// They are only generated during initial login (magic link validation)
		// If tokens are missing, it means session is corrupted and should restart
		const tokensExist = await sessionManager.hasCryptoTokens();
		if (!tokensExist) {
			console.warn('⚠️ [REFRESH] Crypto tokens missing - session may be corrupted');
		}

		console.log('🎉 [REFRESH] ===== REFRESH COMPLETADO EXITOSAMENTE =====');
		flashMessagesStore.addMessage('✅ Token renovado exitosamente');
		return true;
	} catch (error) {
		console.error('❌ [REFRESH] ===== ERROR EN REFRESH =====');
		console.error('❌ [REFRESH] Error:', error);
		flashMessagesStore.addMessage('❌ Error en renovación de token');

		// Check for dual token expiry in the error
		if (
			error instanceof Error &&
			error.message.includes('Both access and refresh tokens have expired')
		) {
			console.error('💥 [REFRESH] DUAL EXPIRY detectado');
			flashMessagesStore.addMessage('⚠️ Sesión expirada - requiere nuevo login');
			// DUAL EXPIRY detected during refresh
			await handleDualTokenExpiry();
		}

		// Token refresh failed
		console.log('❌ [REFRESH] Token refresh failed - retornando false');
		return false;
	}
}

// Function removed - was not being used anywhere in the codebase

/**
 * Handle dual token expiry scenario
 */
async function handleDualTokenExpiry(): Promise<void> {
	// DUAL EXPIRY detected - clearing all auth data and requesting new login

	const { authStore } = await import('../stores/auth');
	const { dialogStore } = await import('../stores/dialog');

	// Complete logout with ALL IndexedDB cleanup
	await authStore.logout();

	// Clear all crypto tokens and auth data (defensive security)
	await authStore.clearPreventiveAuthData();

	// Clear session expiration timestamp (defensive security)
	try {
		const { clearSessionExpiration } = await import('../session-storage');
		await clearSessionExpiration();
	} catch (error) {
		console.warn('Failed to clear session expiration during dual token expiry:', error);
		// Non-blocking - continue with auth dialog
	}

	// Show auth dialog to request new email authentication
	const authConfig = {
		destination: { route: '/' }
	};
	dialogStore.show('auth', authConfig);
}
