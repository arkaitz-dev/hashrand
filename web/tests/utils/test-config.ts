/**
 * Test Configuration Helper
 *
 * Single Source of Truth: Reads token durations directly from .env
 *
 * This ensures tests always use the exact same values as the backend,
 * eliminating hardcoded values and manual synchronization.
 *
 * DRY Principle: .env is the only place where durations are defined
 */

import { config } from 'dotenv';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Load .env from project root (two levels up from this file)
const envPath = resolve(__dirname, '../../../.env');
config({ path: envPath });

/**
 * Get access token duration in seconds
 *
 * Source: .env::SPIN_VARIABLE_ACCESS_TOKEN_DURATION_MINUTES
 *
 * @returns Duration in seconds (minutes * 60)
 */
export function getAccessTokenDurationSeconds(): number {
	const minutes = parseInt(process.env.SPIN_VARIABLE_ACCESS_TOKEN_DURATION_MINUTES || '1', 10);
	return minutes * 60;
}

/**
 * Get refresh token duration in seconds
 *
 * Source: .env::SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES
 *
 * @returns Duration in seconds (minutes * 60)
 */
export function getRefreshTokenDurationSeconds(): number {
	const minutes = parseInt(process.env.SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES || '5', 10);
	return minutes * 60;
}

/**
 * Get key rotation threshold in seconds
 *
 * Calculated as 1/3 of refresh token duration (2/3 system)
 *
 * @returns Threshold in seconds when key rotation begins
 */
export function getKeyRotationThresholdSeconds(): number {
	return Math.floor(getRefreshTokenDurationSeconds() / 3);
}

/**
 * Get safe wait time for access token expiration
 *
 * Access token duration + 5 second buffer for safety
 *
 * @returns Wait time in seconds
 */
export function getAccessTokenExpirationWaitSeconds(): number {
	return getAccessTokenDurationSeconds() + 5;
}

/**
 * Display current test configuration
 *
 * Useful for debugging and test output
 */
export function logTestConfiguration(): void {
	console.log('📋 Test Configuration (from .env):');
	console.log(`   Access Token Duration: ${getAccessTokenDurationSeconds()}s`);
	console.log(`   Refresh Token Duration: ${getRefreshTokenDurationSeconds()}s`);
	console.log(`   Key Rotation Threshold: ${getKeyRotationThresholdSeconds()}s (1/3 of refresh)`);
	console.log(
		`   Safe Expiration Wait: ${getAccessTokenExpirationWaitSeconds()}s (access + 5s buffer)`
	);
	console.log(`   .env location: ${envPath}`);
}
