/**
 * i18n Debug Utilities for Browser Console (Development Only)
 *
 * Provides debugging helpers exposed via window.debugI18n
 * Only available when import.meta.env.DEV === true
 */

import type { Writable } from 'svelte/store';
import { getSupportedLanguageCodes } from '$lib/languageConfig';
import { sessionManager } from '$lib/session-manager';

/**
 * Initialize debug utilities in browser console (development only)
 * Exposes window.debugI18n with helper functions
 */
export function initializeDebugUtilities(currentLanguage: Writable<string>): void {
	if (typeof window === 'undefined' || !import.meta.env.DEV) return;

	// @ts-expect-error - Adding to window for debugging
	window.debugI18n = {
		getCurrentLanguage: () => {
			let current;
			const unsubscribe = currentLanguage.subscribe((lang) => (current = lang));
			unsubscribe();
			return current;
		},
		getBrowserLanguages: () => {
			if (typeof navigator !== 'undefined') {
				return {
					language: navigator.language,
					languages: navigator.languages
				};
			}
			return null;
		},
		getStoredLanguage: async () => {
			const preferences = await sessionManager.getUserPreferences();
			return preferences.language;
		},
		detectLanguage: () => detectBrowserLanguageForDebug(),
		getSupportedLanguages: () => getSupportedLanguageCodes(),
		resetLanguage: async () => {
			await sessionManager.setLanguagePreference(null);
			const detected = detectBrowserLanguageForDebug();
			currentLanguage.set(detected);
			return detected;
		},
		setLanguage: (lang: string) => {
			if (getSupportedLanguageCodes().includes(lang)) {
				currentLanguage.set(lang);
				return lang;
			}
			console.error('[i18n] Unsupported language:', lang);
			return null;
		},
		testRTL: () => {
			currentLanguage.set('ar');
		},
		testLTR: () => {
			currentLanguage.set('en');
		}
	};
}

/**
 * Browser language detection for debug utilities
 * Separate from main i18n to avoid circular dependencies
 */
function detectBrowserLanguageForDebug(): string {
	if (typeof window === 'undefined') {
		return 'en';
	}

	const browserLanguages = [navigator.language, ...(navigator.languages || [])];
	const supportedLanguages = new Set(getSupportedLanguageCodes());

	for (const browserLang of browserLanguages) {
		const langCode = browserLang.split('-')[0].toLowerCase();
		if (supportedLanguages.has(langCode)) {
			return langCode;
		}
	}

	return 'en';
}
