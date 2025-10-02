/**
 * Authenticated signed requests
 * For endpoints that require JWT authentication + Ed25519 signature
 */

import { createSignedRequest, signQueryParams } from '../signedRequest';
import { handleRequestWithAutoRetry } from './auto-retry';
import {
	handleHttpError,
	handleCatchError,
	validateSignedResponse,
	getAuthData
} from './utilities';

/**
 * Universal authenticated signed POST request
 *
 * For endpoints that require JWT authentication + signing
 * Automatically adds Authorization header with access token
 * Implements automatic 401 retry with token refresh
 *
 * @param url - API endpoint URL
 * @param payload - Data to send (will be signed)
 * @returns Promise with validated response payload
 */
export async function httpAuthenticatedSignedPOSTRequest<TRequest, TResponse>(
	url: string,
	payload: TRequest
): Promise<TResponse> {
	// Wrap request logic with auto-retry on 401
	return handleRequestWithAutoRetry(async () => {
		try {
			// Get authentication headers
			const authData = await getAuthData(url);

			// Create signed request
			const signedRequest = await createSignedRequest(payload);

			// Send authenticated signed POST request
			// SECURITY: credentials 'omit' because JWT is sent via Authorization header, not cookies
			const response = await fetch(url, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${authData.access_token}`
				},
				body: JSON.stringify(signedRequest),
				credentials: 'omit' // No cookies with JWT endpoints
			});

			// Handle HTTP errors
			if (!response.ok) {
				await handleHttpError(response, url);
			}

			// Parse and validate SignedResponse
			const responseData = await response.json();
			return await validateSignedResponse<TResponse>(responseData, false);
		} catch (error) {
			handleCatchError(error, url, 'Authenticated signed POST request');
		}
	});
}

/**
 * Universal authenticated signed GET request
 *
 * For endpoints that require JWT authentication + signing
 * Automatically adds Authorization header with access token
 * Implements automatic 401 retry with token refresh
 *
 * @param baseUrl - API endpoint URL (without query params)
 * @param params - Query parameters to sign
 * @returns Promise with validated response payload
 */
export async function httpAuthenticatedSignedGETRequest<TResponse>(
	baseUrl: string,
	params: Record<string, string> = {}
): Promise<TResponse> {
	// Wrap request logic with auto-retry on 401
	return handleRequestWithAutoRetry(async () => {
		try {
			// Get authentication headers
			const authData = await getAuthData(baseUrl);

			// Generate Ed25519 signature for query parameters
			const signature = await signQueryParams(params);

			// Add signature to parameters
			const signedParams = { ...params, signature };

			// Build final URL with signed parameters
			const searchParams = new URLSearchParams(signedParams);
			const url = `${baseUrl}?${searchParams}`;

			// Send authenticated signed GET request
			// SECURITY: credentials 'omit' because JWT is sent via Authorization header, not cookies
			const response = await fetch(url, {
				headers: {
					Authorization: `Bearer ${authData.access_token}`
				},
				credentials: 'omit' // No cookies with JWT endpoints
			});

			// Handle HTTP errors
			if (!response.ok) {
				await handleHttpError(response, url);
			}

			// Parse and validate SignedResponse
			const responseData = await response.json();
			return await validateSignedResponse<TResponse>(responseData, false);
		} catch (error) {
			handleCatchError(error, baseUrl, 'Authenticated signed GET request');
		}
	});
}

/**
 * Universal authenticated signed DELETE request with SignedResponse validation
 *
 * For DELETE endpoints that require JWT authentication + Ed25519 signature
 * Automatically adds Authorization header and validates SignedResponse
 * Implements automatic 401 retry with token refresh
 *
 * @param url - API endpoint URL
 * @returns Promise with validated response payload
 */
export async function httpSignedAuthenticatedDELETE<TResponse>(url: string): Promise<TResponse> {
	// Wrap request logic with auto-retry on 401
	return handleRequestWithAutoRetry(async () => {
		try {
			// Get authentication headers
			const authData = await getAuthData(url);

			// Generate Ed25519 signature for empty payload (DELETE requests have no body)
			const signature = await signQueryParams({});

			// Add signature as query parameter
			const urlWithSignature = `${url}?signature=${signature}`;

			// Send authenticated signed DELETE request with credentials for HttpOnly cookies
			const response = await fetch(urlWithSignature, {
				method: 'DELETE',
				headers: {
					Authorization: `Bearer ${authData.access_token}`
				},
				credentials: 'include'
			});

			// Handle HTTP errors
			if (!response.ok) {
				await handleHttpError(response, urlWithSignature);
			}

			// Parse and validate SignedResponse
			const responseData = await response.json();
			return await validateSignedResponse<TResponse>(responseData, false);
		} catch (error) {
			handleCatchError(error, url, 'Authenticated signed DELETE request');
		}
	});
}
