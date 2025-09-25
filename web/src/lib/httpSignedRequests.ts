/**
 * Universal HTTP Signed Requests Module
 *
 * Provides DRY functions for ALL API communications with Ed25519 signatures
 * Single source of truth for HTTP requests - prevents unsigned calls
 */

import { createSignedRequest, signQueryParams } from './signedRequest';
import { handleSignedResponseStrict } from './universalSignedResponseHandler';

/**
 * Universal error class for HTTP signed requests
 */
export class HttpSignedRequestError extends Error {
	constructor(
		message: string,
		public readonly status: number,
		public readonly url: string
	) {
		super(message);
		this.name = 'HttpSignedRequestError';
	}
}

// TODO: Implement reactive 401 handling when integrating with HTTP request functions

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
		const response = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(signedRequest),
			...fetchOptions
		});

		// Handle HTTP errors
		if (!response.ok) {
			const errorText = await response.text();
			throw new HttpSignedRequestError(
				errorText || `HTTP ${response.status}`,
				response.status,
				url
			);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await handleSignedResponseStrict<TResponse>(responseData, isFirstSignedResponse);
	} catch (error) {
		if (error instanceof HttpSignedRequestError) {
			throw error;
		}

		console.error(`Signed POST request failed: ${url}`, error);
		throw new HttpSignedRequestError(
			`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
			0,
			url
		);
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
		const response = await fetch(url);

		// Handle HTTP errors
		if (!response.ok) {
			const errorText = await response.text();
			throw new HttpSignedRequestError(
				errorText || `HTTP ${response.status}`,
				response.status,
				url
			);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await handleSignedResponseStrict<TResponse>(responseData, isFirstSignedResponse);
	} catch (error) {
		if (error instanceof HttpSignedRequestError) {
			throw error;
		}

		console.error(`Signed GET request failed: ${baseUrl}`, error);
		throw new HttpSignedRequestError(
			`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
			0,
			baseUrl
		);
	}
}

/**
 * Universal authenticated signed POST request
 *
 * For endpoints that require JWT authentication + signing
 * Automatically adds Authorization header with access token
 *
 * @param url - API endpoint URL
 * @param payload - Data to send (will be signed)
 * @returns Promise with validated response payload
 */
export async function httpAuthenticatedSignedPOSTRequest<TRequest, TResponse>(
	url: string,
	payload: TRequest
): Promise<TResponse> {
	try {
		// Get authentication headers
		const { sessionManager } = await import('./session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.access_token) {
			throw new HttpSignedRequestError('No access token available', 401, url);
		}

		// Create signed request
		const signedRequest = await createSignedRequest(payload);

		// Send authenticated signed POST request
		const response = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${authData.access_token}`
			},
			body: JSON.stringify(signedRequest)
		});

		// Handle HTTP errors
		if (!response.ok) {
			const errorText = await response.text();
			throw new HttpSignedRequestError(
				errorText || `HTTP ${response.status}`,
				response.status,
				url
			);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await handleSignedResponseStrict<TResponse>(responseData, false);
	} catch (error) {
		if (error instanceof HttpSignedRequestError) {
			throw error;
		}

		console.error(`Authenticated signed POST request failed: ${url}`, error);
		throw new HttpSignedRequestError(
			`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
			0,
			url
		);
	}
}

/**
 * Universal authenticated signed GET request
 *
 * For endpoints that require JWT authentication + signing
 * Automatically adds Authorization header with access token
 *
 * @param baseUrl - API endpoint URL (without query params)
 * @param params - Query parameters to sign
 * @returns Promise with validated response payload
 */
export async function httpAuthenticatedSignedGETRequest<TResponse>(
	baseUrl: string,
	params: Record<string, string> = {}
): Promise<TResponse> {
	try {
		// Get authentication headers
		const { sessionManager } = await import('./session-manager');
		const authData = await sessionManager.getAuthData();

		if (!authData.access_token) {
			throw new HttpSignedRequestError('No access token available', 401, baseUrl);
		}

		// Generate Ed25519 signature for query parameters
		const signature = await signQueryParams(params);

		// Add signature to parameters
		const signedParams = { ...params, signature };

		// Build final URL with signed parameters
		const searchParams = new URLSearchParams(signedParams);
		const url = `${baseUrl}?${searchParams}`;

		// Send authenticated signed GET request
		const response = await fetch(url, {
			headers: {
				Authorization: `Bearer ${authData.access_token}`
			}
		});

		// Handle HTTP errors
		if (!response.ok) {
			const errorText = await response.text();
			throw new HttpSignedRequestError(
				errorText || `HTTP ${response.status}`,
				response.status,
				url
			);
		}

		// Parse and validate SignedResponse
		const responseData = await response.json();
		return await handleSignedResponseStrict<TResponse>(responseData, false);
	} catch (error) {
		if (error instanceof HttpSignedRequestError) {
			throw error;
		}

		console.error(`Authenticated signed GET request failed: ${baseUrl}`, error);
		throw new HttpSignedRequestError(
			`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
			0,
			baseUrl
		);
	}
}

/**
 * Universal signed DELETE request
 *
 * For DELETE requests that need Ed25519 signature
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
		const response = await fetch(urlWithSignature, {
			method: 'DELETE',
			...fetchOptions
		});

		// Handle HTTP errors
		if (!response.ok) {
			const errorText = await response.text();
			throw new HttpSignedRequestError(
				errorText || `HTTP ${response.status}`,
				response.status,
				url
			);
		}

		// DELETE requests typically don't return data, so no SignedResponse validation needed
	} catch (error) {
		if (error instanceof HttpSignedRequestError) {
			throw error;
		}

		console.error(`Signed DELETE request failed: ${url}`, error);
		throw new HttpSignedRequestError(
			`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
			0,
			url
		);
	}
}
