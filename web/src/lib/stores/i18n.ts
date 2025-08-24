import { writable, derived } from 'svelte/store';
import type { I18nTexts } from '$lib/types';
import { getSupportedLanguageCodes } from '$lib/languageConfig';

// Import individual language files
import { en } from './translations/en';
import { hi } from './translations/hi';
import { ja } from './translations/ja';
import { es } from './translations/es';
import { pt } from './translations/pt';
import { fr } from './translations/fr';
import { de } from './translations/de';
import { ru } from './translations/ru';
import { zh } from './translations/zh';
import { ar } from './translations/ar';
import { eu } from './translations/eu';
import { ca } from './translations/ca';
import { gl } from './translations/gl';

// Language detection function
function detectBrowserLanguage(): string {
	if (typeof window === 'undefined') {
		return 'en'; // SSR fallback
	}

	// Get browser language preferences
	const browserLanguages = [navigator.language, ...(navigator.languages || [])];

	// Map of supported languages
	const supportedLanguages = new Set(getSupportedLanguageCodes());

	// Check each browser language preference
	for (const browserLang of browserLanguages) {
		// Extract language code (e.g., 'es-ES' -> 'es')
		const langCode = browserLang.split('-')[0].toLowerCase();

		if (supportedLanguages.has(langCode)) {
			return langCode;
		}
	}

	// Default fallback to English
	return 'en';
}

// Initialize with detected language or fallback to English
function initializeLanguage(): string {
	if (typeof window === 'undefined') {
		return 'en'; // SSR fallback
	}

	// First check localStorage for user preference
	const storedLang = localStorage.getItem('preferred-language');
	if (storedLang && getSupportedLanguageCodes().includes(storedLang)) {
		return storedLang;
	}

	// If no stored preference, detect browser language
	const detectedLang = detectBrowserLanguage();

	// Store the detected language as user preference
	localStorage.setItem('preferred-language', detectedLang);

	return detectedLang;
}

// Current language store with automatic detection
export const currentLanguage = writable<string>(initializeLanguage());

// Subscribe to language changes and persist them
currentLanguage.subscribe((language) => {
	if (typeof window !== 'undefined') {
		localStorage.setItem('preferred-language', language);

		// Debug information in development
		if (import.meta.env.DEV) {
			console.log(`[i18n] Language changed to: ${language}`);
		}
	}
});

// Debug functions for browser console (development only)
if (typeof window !== 'undefined' && import.meta.env.DEV) {
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
		getStoredLanguage: () => localStorage.getItem('preferred-language'),
		detectLanguage: () => detectBrowserLanguage(),
		getSupportedLanguages: () => getSupportedLanguageCodes(),
		resetLanguage: () => {
			localStorage.removeItem('preferred-language');
			const detected = detectBrowserLanguage();
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
			console.log('[i18n] Testing RTL - switching to Arabic');
			currentLanguage.set('ar');
		},
		testLTR: () => {
			console.log('[i18n] Testing LTR - switching to English');
			currentLanguage.set('en');
		}
	};

	console.log('[i18n] Debug functions available at window.debugI18n');
	console.log('[i18n] Try: debugI18n.getBrowserLanguages(), debugI18n.getCurrentLanguage(), etc.');
}

// Comprehensive translation texts with proper grammar for all languages
export const translations: Record<string, I18nTexts> = {
	// Clean modular translations
	en,
	es,
	pt,
	fr,
	de,
	ru,
	zh,
	ar,
	eu,
	ca,
	gl,
	hi,
	ja
};

// Translation function with intelligent fallback system
export function t(key: string, lang: string = 'en'): string {
	const keys = key.split('.');

	// Helper function to get translation from a specific language
	function getTranslationFromLang(targetLang: string): string | null {
		let value: unknown = translations[targetLang];

		for (const k of keys) {
			if (value && typeof value === 'object' && k in value) {
				value = (value as Record<string, unknown>)[k];
			} else {
				return null; // Translation not found in this language
			}
		}

		return typeof value === 'string' ? value : null;
	}

	// Try the requested language first
	let result = getTranslationFromLang(lang);
	if (result) return result;

	// If not found and requested language is not English, try English as fallback
	if (lang !== 'en') {
		result = getTranslationFromLang('en');
		if (result) return result;
	}

	// If still not found, return the key itself
	return key;
}

// Reactive translation function that works with the store
export const _ = derived(currentLanguage, (lang) => {
	return (key: string) => t(key, lang);
});

// Get current translations object
export const currentTexts = derived(currentLanguage, (lang) => {
	return translations[lang] || translations.en;
});
