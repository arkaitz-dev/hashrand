/**
 * API Helpers Module - Shared Utilities
 *
 * Single Responsibility: Common API utilities and error handling
 * Part of api.ts refactorization to apply DRY principles
 */

/**
 * API Error class for consistent error handling
 */
export class ApiError extends Error {
	constructor(
		message: string,
		public readonly status: number
	) {
		super(message);
		this.name = 'ApiError';
		this.status = status; // Preserve status for error handling
	}

	get statusCode(): number {
		return this.status;
	}
}

/**
 * Handle JSON response with STRICT SignedResponse validation
 *
 * ALL responses (except /api/version) MUST be SignedResponse
 * No fallback - redirects to "/" on validation failure
 */
export async function handleJsonResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}

	const responseData = await response.json();

	// STRICT validation - NO fallback for security
	const { handleSignedResponseStrict } = await import('../universalSignedResponseHandler');
	return await handleSignedResponseStrict<T>(responseData, false);
}

/**
 * Build URLSearchParams from parameter object (DRY utility)
 */
export function buildSearchParams(params: Record<string, unknown>): URLSearchParams {
	const searchParams = new URLSearchParams();

	Object.entries(params).forEach(([key, value]) => {
		if (value !== undefined && value !== null) {
			if (typeof value === 'boolean') {
				searchParams.set(key, value.toString());
			} else if (typeof value === 'number') {
				searchParams.set(key, value.toString());
			} else if (typeof value === 'string') {
				searchParams.set(key, value);
			}
		}
	});

	return searchParams;
}
