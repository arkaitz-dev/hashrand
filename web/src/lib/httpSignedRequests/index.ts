/**
 * Universal HTTP Signed Requests Module
 *
 * Provides DRY functions for ALL API communications with Ed25519 signatures
 * Single source of truth for HTTP requests - prevents unsigned calls
 */

// Export types
export { HttpSignedRequestError } from './types';

// Export unsigned (non-authenticated) signed requests
export {
	httpSignedPOSTRequest,
	httpSignedGETRequest,
	httpSignedDELETERequest
} from './unsigned-requests';

// Export authenticated signed requests
export {
	httpAuthenticatedSignedPOSTRequest,
	httpAuthenticatedSignedGETRequest,
	httpSignedAuthenticatedDELETE
} from './authenticated-requests';
