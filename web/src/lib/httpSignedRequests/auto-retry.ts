/**
 * Automatic 401 retry logic with token refresh
 */

import { HttpSignedRequestError } from './types';
import { logger } from '../utils/logger';

/**
 * Global flag to prevent concurrent refresh attempts
 * Only one refresh can run at a time to avoid race conditions
 */
let isCurrentlyRefreshing = false;

/**
 * Universal 401 handler with automatic token refresh and retry
 *
 * Implements reactive authentication pattern:
 * 1. Execute request function
 * 2. If 401 error and not already refreshing → attempt token refresh
 * 3. If refresh succeeds → retry original request ONCE
 * 4. If refresh fails or second attempt fails → propagate error
 *
 * @param requestFn - Function that performs the HTTP request
 * @returns Promise with response data
 * @throws HttpSignedRequestError if request fails after retry
 */
export async function handleRequestWithAutoRetry<TResponse>(
	requestFn: () => Promise<TResponse>
): Promise<TResponse> {
	try {
		return await requestFn();
	} catch (error) {
		// Only handle 401 errors from authenticated requests
		if (error instanceof HttpSignedRequestError && error.status === 401 && !isCurrentlyRefreshing) {
			isCurrentlyRefreshing = true;
			try {
				// Attempt to refresh access token using HttpOnly refresh cookie
				const { refreshToken } = await import('../api/api-auth-operations');
				const refreshSuccess = await refreshToken();

				if (refreshSuccess) {
					// Retry original request with new access token
					return await requestFn();
				} else {
					logger.error('Token refresh failed');
					throw error; // Propagate original 401 error
				}
			} catch (refreshError) {
				logger.error('Token refresh error:', refreshError);
				// If refresh itself fails, propagate original 401 error
				throw error;
			} finally {
				isCurrentlyRefreshing = false;
			}
		}

		// Not a 401 or already refreshing - propagate error
		throw error;
	}
}
