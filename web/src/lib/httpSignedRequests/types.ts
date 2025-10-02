/**
 * Type definitions for HTTP signed requests
 */

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
		// Preserve status and url for error handling
		this.status = status;
		this.url = url;
	}
}
