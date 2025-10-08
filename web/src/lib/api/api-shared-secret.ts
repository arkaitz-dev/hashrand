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
	ViewSharedSecretResult
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
 *
 * NOTE: Backend returns errors (OTP_REQUIRED, INVALID_OTP) as HTTP 200 with error field
 * Caller must check for 'error' field in response to detect these cases
 */
export async function viewSharedSecret(
	hash: string,
	otpRequest?: ViewSharedSecretRequest
): Promise<ViewSharedSecretResult> {
	if (otpRequest && otpRequest.otp) {
		// POST request with OTP in body
		const { httpAuthenticatedSignedPOSTRequest } = await import('../httpSignedRequests');
		return await httpAuthenticatedSignedPOSTRequest<
			ViewSharedSecretRequest,
			ViewSharedSecretResult
		>(`${API_BASE}/shared-secret/${hash}`, otpRequest);
	} else {
		// GET request without OTP
		const { httpAuthenticatedSignedGETRequest } = await import('../httpSignedRequests');
		return await httpAuthenticatedSignedGETRequest<ViewSharedSecretResult>(
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
 * Updates read_at timestamp and decrements pending_reads in tracking table
 * Idempotent: multiple calls are safe
 *
 * @param hash - Base58 encoded shared secret hash
 * @returns Promise with new pending_reads count
 */
export async function confirmRead(hash: string): Promise<{
	success: boolean;
	pending_reads: number;
	read_confirmed: boolean;
	role: string;
	message: string;
}> {
	const { httpAuthenticatedSignedGETRequest } = await import('../httpSignedRequests');
	return await httpAuthenticatedSignedGETRequest<{
		success: boolean;
		pending_reads: number;
		read_confirmed: boolean;
		role: string;
		message: string;
	}>(`${API_BASE}/shared-secret/confirm-read`, { hash }); // Pass hash as param, not in URL
}
