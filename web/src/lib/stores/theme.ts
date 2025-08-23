import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark';

/**
 * Theme Store Behavior:
 * 1. On first visit: Uses system preference (prefers-color-scheme)
 * 2. When user toggles: Saves preference to localStorage and uses it
 * 3. On subsequent visits: Uses saved preference (overrides system)
 * 4. Optional reset function available to return to system preference
 */

// Initialize theme from localStorage or system preference
function getInitialTheme(): Theme {
	if (!browser) return 'dark'; // SSR fallback

	// Check if user has manually set a preference
	const stored = localStorage.getItem('theme') as Theme | null;
	if (stored === 'light' || stored === 'dark') {
		// User has made a manual choice, use it
		return stored;
	}

	// No manual preference stored, use system preference
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

	// Store in localStorage
	localStorage.setItem('theme', newTheme);

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
export function resetToSystemTheme() {
	if (!browser) return;

	// Remove manual preference
	localStorage.removeItem('theme');

	// Use system preference
	const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
	theme.set(systemTheme);
	// The store subscription will handle calling applyTheme
}

// Initialize theme on store creation and subscribe to changes
if (browser) {
	// Subscribe to theme changes (this will also apply the initial theme)
	theme.subscribe(applyTheme);
}
