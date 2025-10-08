/**
 * DRY utilities for HTTP signed requests
 * Consolidates repeated patterns across all request types
 */

import { handleSignedResponseStrict } from '../universalSignedResponseHandler';
import { HttpSignedRequestError } from './types';
import { logger } from '../utils/logger';

/**
 * Handle HTTP error response (DRY consolidation for 6 duplicated patterns)
 *
 * @param response - Fetch response object
 * @param url - Request URL for error context
 * @throws HttpSignedRequestError with response status and text
 */
export async function handleHttpError(response: Response, url: string): Promise<never> {
	const errorText = await response.text();
	throw new HttpSignedRequestError(errorText || `HTTP ${response.status}`, response.status, url);
}

/**
 * Handle catch block error (DRY consolidation for 6 duplicated patterns)
 *
 * @param error - Caught error
 * @param url - Request URL for error context
 * @param operation - Operation description (e.g., "Signed POST request")
 * @throws HttpSignedRequestError with proper error message
 */
export function handleCatchError(error: unknown, url: string, operation: string): never {
	if (error instanceof HttpSignedRequestError) {
		throw error;
	}

	logger.error(`${operation} failed: ${url}`, error);
	throw new HttpSignedRequestError(
		`Request failed: ${error instanceof Error ? error.message : 'Unknown error'}`,
		0,
		url
	);
}

/**
 * Validate SignedResponse (DRY consolidation for 6 duplicated patterns)
 *
 * @param responseData - Response JSON data
 * @param isFirstSignedResponse - If true, extracts server_pub_key
 * @returns Promise with validated response payload
 */
export async function validateSignedResponse<TResponse>(
	responseData: unknown,
	isFirstSignedResponse: boolean = false
): Promise<TResponse> {
	return await handleSignedResponseStrict<TResponse>(responseData, isFirstSignedResponse);
}

/**
 * Get authentication data (DRY consolidation for 4 duplicated patterns)
 *
 * @param url - Request URL for error context
 * @returns Promise with auth data containing access_token
 * @throws HttpSignedRequestError if no access token available
 */
export async function getAuthData(url: string): Promise<{ access_token: string }> {
	const { sessionManager } = await import('../session-manager');
	const authData = await sessionManager.getAuthData();

	if (!authData.access_token) {
		throw new HttpSignedRequestError('No access token available', 401, url);
	}

	return { access_token: authData.access_token };
}
