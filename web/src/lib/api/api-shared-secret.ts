/**
 * API Shared Secret Operations Module
 *
 * Single Responsibility: Handle all shared secret endpoints
 * Part of api.ts architecture following DRY principles
 */

import type {
	CreateSharedSecretRequest,
	CreateSharedSecretResponse,
	ViewSharedSecretRequest,
	ViewSharedSecretResponse
} from '../types';

const API_BASE = '/api';

/**
 * Create a shared secret (POST)
 * Requires authentication and Ed25519 signature
 */
export async function createSharedSecret(
	request: CreateSharedSecretRequest
): Promise<CreateSharedSecretResponse> {
	const { httpAuthenticatedSignedPOSTRequest } = await import('../httpSignedRequests');
	return await httpAuthenticatedSignedPOSTRequest<
		CreateSharedSecretRequest,
		CreateSharedSecretResponse
	>(`${API_BASE}/shared-secret/create`, request);
}

/**
 * View a shared secret (GET with optional OTP in POST body)
 * Requires authentication and Ed25519 signature
 */
export async function viewSharedSecret(
	hash: string,
	otpRequest?: ViewSharedSecretRequest
): Promise<ViewSharedSecretResponse> {
	if (otpRequest && otpRequest.otp) {
		// POST request with OTP in body
		const { httpAuthenticatedSignedPOSTRequest } = await import('../httpSignedRequests');
		return await httpAuthenticatedSignedPOSTRequest<
			ViewSharedSecretRequest,
			ViewSharedSecretResponse
		>(`${API_BASE}/shared-secret/${hash}`, otpRequest);
	} else {
		// GET request without OTP
		const { httpAuthenticatedSignedGETRequest } = await import('../httpSignedRequests');
		return await httpAuthenticatedSignedGETRequest<ViewSharedSecretResponse>(
			`${API_BASE}/shared-secret/${hash}`
		);
	}
}

/**
 * Delete a shared secret (DELETE)
 * Requires authentication and Ed25519 signature
 * Only works if pending_reads > 0
 */
export async function deleteSharedSecret(hash: string): Promise<void> {
	const { httpSignedAuthenticatedDELETE } = await import('../httpSignedRequests');
	await httpSignedAuthenticatedDELETE(`${API_BASE}/shared-secret/${hash}`);
}
