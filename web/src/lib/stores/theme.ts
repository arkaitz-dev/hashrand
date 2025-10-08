import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import { sessionManager } from '$lib/session-manager';
import { logger } from '$lib/utils/logger';

export type Theme = 'light' | 'dark';

/**
 * Theme Store Behavior:
 * 1. On first visit: Uses system preference (prefers-color-scheme)
 * 2. When user toggles: Saves preference to IndexedDB and uses it
 * 3. On subsequent visits: Uses saved preference (overrides system)
 * 4. Optional reset function available to return to system preference
 */

// Initialize theme with system preference (will be updated from IndexedDB async)
function getInitialTheme(): Theme {
	if (!browser) return 'dark'; // SSR fallback

	// Return system preference for immediate initialization
	// IndexedDB data will be loaded asynchronously and update the store
	return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

// Create the theme store
export const theme = writable<Theme>(getInitialTheme());

// Apply theme to document
export function applyTheme(newTheme: Theme) {
	if (!browser) return;

	const html = document.documentElement;

	// Add a smooth transition class for theme changes
	html.style.transition = 'background-color 0.3s ease, color 0.3s ease';

	if (newTheme === 'dark') {
		html.classList.add('dark');
	} else {
		html.classList.remove('dark');
	}

	// Store in IndexedDB (async, fire-and-forget)
	sessionManager.setThemePreference(newTheme).catch((error) => {
		logger.warn('Failed to save theme preference to IndexedDB:', error);
	});

	// Update meta theme-color for mobile browser UI
	const metaThemeColor = document.querySelector('meta[name="theme-color"]');
	if (metaThemeColor) {
		metaThemeColor.setAttribute('content', newTheme === 'dark' ? '#1e293b' : '#3b82f6');
	}
}

// Toggle theme function - this represents a manual user choice
export function toggleTheme() {
	theme.update((current) => {
		const newTheme = current === 'dark' ? 'light' : 'dark';
		// The store subscription will handle calling applyTheme
		return newTheme;
	});
}

// Optional: Reset to system preference (for future use)
export async function resetToSystemTheme() {
	if (!browser) return;

	// Remove manual preference from IndexedDB
	await sessionManager.setThemePreference(null);

	// Use system preference
	const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
	theme.set(systemTheme);
	// The store subscription will handle calling applyTheme
}

// Load theme preference from IndexedDB (async initialization)
async function loadThemeFromIndexedDB() {
	if (!browser) return;

	try {
		const preferences = await sessionManager.getUserPreferences();
		if (preferences.theme === 'light' || preferences.theme === 'dark') {
			// Update store with stored preference
			theme.set(preferences.theme);
		} else if (!preferences.theme) {
			// No stored preference, save current system preference
			const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
				? 'dark'
				: 'light';
			await sessionManager.setThemePreference(systemTheme);
			theme.set(systemTheme);
		}
	} catch (error) {
		logger.warn('Failed to load theme from IndexedDB:', error);
	}
}

// Initialize theme on store creation and subscribe to changes
if (browser) {
	// Subscribe to theme changes (this will also apply the initial theme)
	theme.subscribe(applyTheme);

	// Load theme preference from IndexedDB after store creation
	loadThemeFromIndexedDB();
}
