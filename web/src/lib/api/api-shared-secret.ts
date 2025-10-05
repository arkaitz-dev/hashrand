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

/**
 * Confirm read of a shared secret (GET)
 * Requires authentication and Ed25519 signature
 * Updates read_at timestamp in tracking table
 * Idempotent: multiple calls are safe
 *
 * @param hash - Base58 encoded shared secret hash
 * @returns Promise<void>
 */
export async function confirmRead(hash: string): Promise<void> {
	const { httpAuthenticatedSignedGETRequest } = await import('../httpSignedRequests');
	await httpAuthenticatedSignedGETRequest<{
		success: boolean;
		updated: boolean;
		role: string;
		message: string;
	}>(`${API_BASE}/shared-secret/confirm-read?hash=${hash}`);
}
