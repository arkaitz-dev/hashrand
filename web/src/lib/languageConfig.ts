/**
 * Shared language configuration for the application
 * Contains the list of supported languages with their metadata
 */

export interface Language {
	code: string;
	name: string;
	flag: string;
}

/**
 * Complete list of supported languages
 * Ordered alphabetically by native name transcribed to Latin alphabet
 */
export const languages: Language[] = [
	{ code: 'ar', name: 'العربية', flag: 'saudi' }, // Arabiya
	{ code: 'ca', name: 'Català', flag: 'catalonia' }, // Catala
	{ code: 'de', name: 'Deutsch', flag: 'germany' }, // Deutsch
	{ code: 'en', name: 'English', flag: 'uk' }, // English
	{ code: 'es', name: 'Español', flag: 'spain' }, // Espanol
	{ code: 'eu', name: 'Euskera', flag: 'basque' }, // Euskera
	{ code: 'fr', name: 'Français', flag: 'france' }, // Francais
	{ code: 'gl', name: 'Galego', flag: 'galicia' }, // Galego
	{ code: 'hi', name: 'हिंदी', flag: 'india' }, // Hindi
	{ code: 'ja', name: '日本語', flag: 'japan' }, // Nihongo
	{ code: 'pt', name: 'Português', flag: 'portugal' }, // Portugues
	{ code: 'ru', name: 'Русский', flag: 'russia' }, // Russkiy
	{ code: 'zh', name: '中文', flag: 'china' } // Zhongwen
];

/**
 * Find language object by code
 * @param code Language code to search for
 * @returns Language object or first language as fallback
 */
export function findLanguageByCode(code: string): Language {
	return languages.find((lang) => lang.code === code) || languages[0];
}

/**
 * Get array of supported language codes
 * @returns Array of language codes
 */
export function getSupportedLanguageCodes(): string[] {
	return languages.map((lang) => lang.code);
}
