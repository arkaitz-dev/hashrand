/**
 * Utility functions for auth operations
 */

import { get } from 'svelte/store';

/**
 * Get current language from i18n store or browser fallback (DRY utility)
 */
export async function getCurrentLanguage(): Promise<string> {
	try {
		const { currentLanguage } = await import('../../stores/i18n');
		return get(currentLanguage);
	} catch {
		// Fallback to browser language detection
		if (typeof navigator !== 'undefined') {
			return navigator.language.split('-')[0].toLowerCase();
		}
		return 'en'; // Final fallback
	}
}

export const API_BASE = '/api';
