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
		public readonly status: number // eslint-disable-line no-unused-vars
	) {
		super(message);
		this.name = 'ApiError';
	}

	get statusCode(): number {
		return this.status;
	}
}

/**
 * Handle JSON response with consistent error handling
 */
export async function handleJsonResponse<T>(response: Response): Promise<T> {
	if (!response.ok) {
		const errorText = await response.text();
		throw new ApiError(errorText || `HTTP ${response.status}`, response.status);
	}
	return response.json();
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

/**
 * Generic GET request handler (DRY for generation endpoints)
 */
export async function handleGetRequest<T>(
	endpoint: string,
	params: Record<string, unknown>,
	// eslint-disable-next-line no-unused-vars
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<T> {
	const searchParams = buildSearchParams(params);
	const url = searchParams.toString() ? `${endpoint}?${searchParams}` : endpoint;

	const response = await authenticatedFetch(url);
	return await handleJsonResponse<T>(response);
}

/**
 * Generic POST request handler (DRY for seed endpoints)
 */
export async function handlePostRequest<T>(
	endpoint: string,
	body: unknown,
	// eslint-disable-next-line no-unused-vars
	authenticatedFetch: (url: string, options?: RequestInit) => Promise<Response>
): Promise<T> {
	const response = await authenticatedFetch(endpoint, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(body)
	});

	return await handleJsonResponse<T>(response);
}
