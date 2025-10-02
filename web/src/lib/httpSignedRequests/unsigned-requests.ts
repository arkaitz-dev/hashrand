/**
 * Non-authenticated signed requests
 * For endpoints that require Ed25519 signature but not JWT authentication
 */

import { createSignedRequest, signQueryParams } from '../signedRequest';
import { handleHttpError, handleCatchError, validateSignedResponse } from './utilities';

/**
 * Universal signed POST request
 *
 * ALL POST requests to /api/* (except /api/version) MUST use this function
 * Automatically signs payload and validates SignedResponse
 *
 * @param url - API endpoint URL
 * @param payload - Data to send (will be signed)
 * @param isFirstSignedResponse - If true, extracts server_pub_key
 * @param fetchOptions - Additional fetch options (e.g., credentials)
 * @returns Promise with validated response payload
 */
export async function httpSignedPOSTRequest<TRequest, TResponse>(
	url: string,
	payload: TRequest,
	isFirstSignedResponse: boolean = false,
	fetchOptions: RequestInit = {}
): Promise<TResponse> {
	try {
		// Create signed request with Ed25519 signature
		const signedRequest = await createSignedRequest(payload);

		// Send signed POST request with additional options
		// SECURITY: credentials 'omit' by default to prevent cookie leakage to authenticated endpoints
		const response = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(signedRequest),
			credentials: 'omit', // Default: no cookies sent
			...fetchOptions // Can override with credentials: 'include' if needed
		});

		// Handle HTTP errors
		if (!response.ok) {
			await handleHttpError(response, url);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await validateSignedResponse<TResponse>(responseData, isFirstSignedResponse);
	} catch (error) {
		handleCatchError(error, url, 'Signed POST request');
	}
}

/**
 * Universal signed GET request
 *
 * ALL GET requests to /api/* (except /api/version) MUST use this function
 * Automatically signs query parameters and validates SignedResponse
 *
 * @param baseUrl - API endpoint URL (without query params)
 * @param params - Query parameters to sign
 * @param isFirstSignedResponse - If true, extracts server_pub_key
 * @returns Promise with validated response payload
 */
export async function httpSignedGETRequest<TResponse>(
	baseUrl: string,
	params: Record<string, string> = {},
	isFirstSignedResponse: boolean = false
): Promise<TResponse> {
	try {
		// Generate Ed25519 signature for query parameters
		const signature = await signQueryParams(params);

		// Add signature to parameters
		const signedParams = { ...params, signature };

		// Build final URL with signed parameters
		const searchParams = new URLSearchParams(signedParams);
		const url = `${baseUrl}?${searchParams}`;

		// Send signed GET request
		// SECURITY: credentials 'omit' to prevent cookie leakage to authenticated endpoints
		const response = await fetch(url, {
			credentials: 'omit' // No cookies sent to authenticated endpoints
		});

		// Handle HTTP errors
		if (!response.ok) {
			await handleHttpError(response, url);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await validateSignedResponse<TResponse>(responseData, isFirstSignedResponse);
	} catch (error) {
		handleCatchError(error, baseUrl, 'Signed GET request');
	}
}

/**
 * Universal signed DELETE request (without authentication or response validation)
 *
 * For DELETE requests that need Ed25519 signature but don't require JWT auth
 * NOTE: Most DELETE endpoints should use httpSignedAuthenticatedDELETE instead
 *
 * @param url - API endpoint URL
 * @param fetchOptions - Additional fetch options (e.g., credentials)
 * @returns Promise<void> - DELETE requests typically don't return data
 */
export async function httpSignedDELETERequest(
	url: string,
	fetchOptions: RequestInit = {}
): Promise<void> {
	try {
		// Generate Ed25519 signature for empty payload (DELETE requests have no body)
		const signature = await signQueryParams({});

		// Add signature as query parameter
		const urlWithSignature = `${url}?signature=${signature}`;

		// Send signed DELETE request with additional options
		// SECURITY: credentials 'omit' by default to prevent cookie leakage
		const response = await fetch(urlWithSignature, {
			method: 'DELETE',
			credentials: 'omit', // Default: no cookies sent
			...fetchOptions // Can override if needed
		});

		// Handle HTTP errors
		if (!response.ok) {
			await handleHttpError(response, url);
		}

		// DELETE requests typically don't return data, so no SignedResponse validation needed
	} catch (error) {
		handleCatchError(error, url, 'Signed DELETE request');
	}
}
