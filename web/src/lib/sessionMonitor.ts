/**
 * Session Monitor - Automatic session expiration monitoring
 *
 * Single Responsibility: Monitor session expiration in background and trigger auto-logout
 *
 * Features:
 * - Checks session expiration every 10 seconds
 * - Pauses monitoring when tab is in background (battery savings)
 * - Performs immediate check when tab becomes active again
 * - Shows flash message in user's language before logout
 * - Redirects to home page after logout
 * - Only activates when user is authenticated
 */

import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { get } from 'svelte/store';
import { isSessionExpired, handleExpiredSession } from './session-expiry-manager';
import { flashMessagesStore } from './stores/flashMessages';
import { _ } from './stores/i18n';
import { logger } from './utils/logger';

/**
 * Session monitor configuration
 */
const CHECK_INTERVAL_MS = 10000; // 10 seconds

/**
 * Internal state
 */
let intervalId: number | null = null;
let isMonitoring = false;

/**
 * Check if user is currently authenticated
 * Only checks local existence - no HTTP calls
 */
async function isUserAuthenticated(): Promise<boolean> {
	try {
		const { sessionManager } = await import('./session-manager');
		const authData = await sessionManager.getAuthData();
		return !!(authData.access_token && authData.user?.user_id);
	} catch {
		return false;
	}
}

/**
 * Check if session has expired and handle auto-logout
 * CRITICAL: Only runs if user is authenticated
 */
async function checkAndHandleExpiration(): Promise<void> {
	try {
		// CRITICAL: First check if user is authenticated
		const authenticated = await isUserAuthenticated();

		if (!authenticated) {
			// User not authenticated - stop monitoring silently
			stopMonitoring();
			return;
		}

		// User is authenticated - check expiration
		const expired = await isSessionExpired();

		if (expired) {
			await performAutoLogout();
		}
	} catch (error) {
		logger.warn('Session monitor check failed:', error);
		// Don't logout on check errors - could be temporary
	}
}

/**
 * Perform automatic logout on session expiration
 */
async function performAutoLogout(): Promise<void> {
	// Stop monitoring immediately
	stopMonitoring();

	try {
		// Get translated message before cleanup (i18n store needs to be accessible)
		const translateFn = get(_);
		const message = translateFn('common.sessionExpired');

		// Show flash message to user
		flashMessagesStore.addMessage(message);

		// Clean up all session data (IndexedDB, Ed25519 keys, etc)
		await handleExpiredSession();

		// Redirect to home page
		await goto('/');
	} catch (error) {
		logger.error('Auto-logout failed:', error);
		// Fallback - force redirect even if cleanup failed
		if (browser) {
			window.location.href = '/';
		}
	}
}

/**
 * Start monitoring session expiration
 */
function startMonitoring(): void {
	if (isMonitoring || !browser) return;

	// Clear any existing interval
	stopMonitoring();

	// Start periodic checks
	intervalId = setInterval(checkAndHandleExpiration, CHECK_INTERVAL_MS) as unknown as number;
	isMonitoring = true;
}

/**
 * Stop monitoring session expiration
 */
function stopMonitoring(): void {
	if (intervalId !== null) {
		clearInterval(intervalId);
		intervalId = null;
		isMonitoring = false;
	}
}

/**
 * Handle visibility change (pause/resume monitoring)
 * CRITICAL: Only resume if user is authenticated
 */
async function handleVisibilityChange(): Promise<void> {
	if (!browser) return;

	if (document.hidden) {
		// Tab is hidden - stop monitoring to save resources
		stopMonitoring();
	} else {
		// Tab is visible again - check if we should resume monitoring
		const authenticated = await isUserAuthenticated();

		if (!authenticated) {
			// User not authenticated - don't start monitoring
			return;
		}

		// User is authenticated - perform immediate check
		checkAndHandleExpiration().catch(() => {
			// Check failed - continue monitoring
		});

		// Resume monitoring
		startMonitoring();
	}
}

/**
 * Initialize session monitor
 * Call this from the root layout to set up listeners
 * CRITICAL: Does NOT start monitoring automatically - only sets up infrastructure
 * Monitoring starts only when user authenticates (call startMonitoringIfAuthenticated)
 */
export function initSessionMonitor(): void {
	if (!browser) return;

	// Set up visibility change listener only
	document.addEventListener('visibilitychange', handleVisibilityChange);
}

/**
 * Start monitoring if user is authenticated
 * Call this after successful login to activate the monitor
 */
export async function startMonitoringIfAuthenticated(): Promise<void> {
	if (!browser) return;

	const authenticated = await isUserAuthenticated();

	if (authenticated) {
		// User is authenticated - start monitoring
		if (!document.hidden) {
			startMonitoring();
		}
	}
}

/**
 * Cleanup session monitor
 * Call this when unmounting root layout (rare in SPA, but good practice)
 */
export function destroySessionMonitor(): void {
	if (!browser) return;

	stopMonitoring();
	document.removeEventListener('visibilitychange', handleVisibilityChange);
}

/**
 * Check if monitor is currently active
 */
export function isMonitorActive(): boolean {
	return isMonitoring;
}
