/**
 * Key Rotation Module - Sistema B Permanent Keys Publication
 *
 * Single Responsibility: Manage publication of permanent user public keys to backend.
 * Part of auth-actions refactorization to apply SOLID principles.
 *
 * PURPOSE:
 * - Publish Ed25519/X25519 public keys to backend database
 * - Enable user-to-user E2EE (Perfect Forward Secrecy)
 * - Idempotent operations (INSERT OR IGNORE in backend)
 *
 * WHEN TO CALL:
 * - After successful magic link validation (validateMagicLink)
 * - Only when JWT + crypto tokens already exist
 *
 * @see sistema-b.ts for key derivation
 * @see user-key-derivation.ts for deterministic key generation
 */

import type { deriveUserKeys } from '../../../crypto/user-key-derivation';
import { logger } from '../../../utils/logger';

/**
 * Publish permanent public keys to backend (Sistema B - E2EE)
 *
 * Sends Ed25519 + X25519 public keys to /api/keys/rotate endpoint.
 * Backend stores these in database tables for user-to-user E2EE.
 *
 * REQUIREMENTS:
 * - JWT token must exist (already logged in)
 * - Crypto tokens must exist (Sistema A keys generated)
 * - Derived keys must be available (from validateMagicLink)
 *
 * ERROR HANDLING:
 * - Non-blocking: Authentication continues even if publication fails
 * - Keys will be republished on next login attempt
 *
 * @param derivedKeys - Derived user keypairs from user-key-derivation.ts
 * @returns Promise<void>
 */
export async function publishPermanentKeys(
	derivedKeys: Awaited<ReturnType<typeof deriveUserKeys>>
): Promise<void> {
	try {
		logger.debug('[key-rotation] 📤 Starting permanent keys publication to /api/keys/rotate');
		logger.debug('[key-rotation] 🔑 Keys to publish:', {
			ed25519_pub_prefix: derivedKeys.ed25519.publicKeyHex.substring(0, 16) + '...',
			x25519_pub_prefix: derivedKeys.x25519.publicKeyHex.substring(0, 16) + '...',
			ed25519_length: derivedKeys.ed25519.publicKeyHex.length,
			x25519_length: derivedKeys.x25519.publicKeyHex.length
		});

		const { httpAuthenticatedSignedPOSTRequest } = await import(
			'../../../httpSignedRequests/authenticated-requests'
		);

		const payload = {
			ed25519_pub: derivedKeys.ed25519.publicKeyHex,
			x25519_pub: derivedKeys.x25519.publicKeyHex
		};

		logger.debug('[key-rotation] 📨 Sending POST request to /api/keys/rotate with payload');
		const response = await httpAuthenticatedSignedPOSTRequest('/api/keys/rotate', payload);

		logger.debug('[key-rotation] ✅ Permanent public keys published successfully, response:', response);
	} catch (publishError) {
		logger.error('[key-rotation] ❌ Failed to publish permanent keys:', publishError);
		logger.error('[key-rotation] Error details:', {
			message: publishError instanceof Error ? publishError.message : String(publishError),
			stack: publishError instanceof Error ? publishError.stack : undefined
		});
		// Non-blocking - authentication continues even if publication fails
		// Keys will be republished on next login attempt
		throw publishError; // Re-throw for caller to handle
	}
}
