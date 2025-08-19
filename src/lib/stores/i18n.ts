import { writable } from 'svelte/store';
import type { I18nTexts } from '$lib/types';

// Current language store
export const currentLanguage = writable<string>('en');

// Translation texts - ready for expansion
export const translations: Record<string, I18nTexts> = {
	en: {
		common: {
			back: 'Back',
			generate: 'Generate',
			copy: 'Copy',
			copied: 'Copied!',
			backToMenu: 'Back to Menu',
			loading: 'Generating...',
			error: 'Error occurred',
			result: 'Result'
		},
		menu: {
			title: 'Hash Generator',
			subtitle: 'Choose a generation method',
			version: 'Version'
		},
		generate: {
			title: 'Custom Hash Generator',
			description: 'Generate customizable random hashes',
			length: 'Length',
			alphabet: 'Alphabet Type',
			prefix: 'Prefix',
			suffix: 'Suffix',
			raw: 'Raw Output'
		},
		password: {
			title: 'Secure Password Generator',
			description: 'Generate secure passwords',
			length: 'Length',
			alphabet: 'Character Set',
			raw: 'Raw Output'
		},
		apiKey: {
			title: 'API Key Generator',
			description: 'Generate API keys with ak_ prefix',
			length: 'Length',
			alphabet: 'Character Set',
			raw: 'Raw Output'
		},
		alphabets: {
			'base58': 'Base58 (58 chars)',
			'no-look-alike': 'No Look-alike (49 chars)',
			'full': 'Full Alphanumeric (62 chars)',
			'full-with-symbols': 'Full with Symbols (73 chars)'
		}
	}
};

// Translation function
export function t(key: string, lang: string = 'en'): string {
	const keys = key.split('.');
	let value: any = translations[lang];
	
	for (const k of keys) {
		if (value && typeof value === 'object' && k in value) {
			value = value[k];
		} else {
			return key; // Return key if translation not found
		}
	}
	
	return typeof value === 'string' ? value : key;
}