import { derived } from 'svelte/store';
import { currentLanguage } from './i18n';

// Languages that use Right-to-Left text direction
const RTL_LANGUAGES = new Set(['ar']);

// Derived store to determine if current language uses RTL
export const isRTL = derived(currentLanguage, (lang) => {
	return RTL_LANGUAGES.has(lang);
});

// Derived store for text direction attribute
export const textDirection = derived(isRTL, (rtl) => {
	return rtl ? 'rtl' : 'ltr';
});

// Helper function to get appropriate CSS classes based on direction
export const getDirectionClasses = (ltrClasses: string, rtlClasses: string = '') => {
	return derived(isRTL, (rtl) => {
		return rtl
			? rtlClasses ||
					ltrClasses.replace(/left|right/g, (match) => (match === 'left' ? 'right' : 'left'))
			: ltrClasses;
	});
};

// Helper to swap left/right in class strings
export const swapDirection = (classes: string): string => {
	return classes
		.replace(/\bleft\b/g, 'TEMP_LEFT')
		.replace(/\bright\b/g, 'left')
		.replace(/TEMP_LEFT/g, 'right');
};
