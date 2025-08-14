// @ts-check
/**
 * Utility functions for API calls with base path support
 */

/**
 * @typedef {Object} ImportMetaEnv
 * @property {string} [BASE_URL] - Vite base URL
 */

/**
 * @typedef {Object} ImportMeta
 * @property {ImportMetaEnv} env - Vite environment variables
 */

/**
 * Get the base path from Vite's configuration
 * This handles both development and production environments
 * @returns {string} The base path for the application
 */
export function getBasePath() {
    // In development, Vite provides import.meta.env.BASE_URL
    // In production, we use the base path from the build
    return (/** @type {ImportMeta} */ (/** @type {unknown} */ (import.meta))).env.BASE_URL || '/';
}

/**
 * Build a full API URL with the correct base path
 * @param {string} endpoint - The API endpoint (e.g., '/api/version')
 * @returns {string} The full URL with base path
 */
export function buildApiUrl(endpoint) {
    // Simply use the base path from Vite configuration
    // This works correctly in both development and production
    const base = getBasePath();
    const cleanBase = base.endsWith('/') ? base.slice(0, -1) : base;
    const cleanEndpoint = endpoint.startsWith('/') ? endpoint : '/' + endpoint;
    return cleanBase + cleanEndpoint;
}

/**
 * Wrapper for fetch that automatically adds the base path
 * @param {string} endpoint - The API endpoint
 * @param {RequestInit} [options={}] - Optional fetch options
 * @returns {Promise<Response>} The fetch response
 */
export async function apiFetch(endpoint, options = {}) {
    const url = buildApiUrl(endpoint);
    return fetch(url, options);
}