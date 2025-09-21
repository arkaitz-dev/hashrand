/**
 * Session Preferences Module - User Preferences Management
 *
 * Single Responsibility: Handle user preferences (language, theme)
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

import { sessionDB } from './session-db';

/**
 * Get user preferences from IndexedDB
 */
export async function getUserPreferences(): Promise<{
	language: string | null;
	theme: 'light' | 'dark' | null;
}> {
	const session = await sessionDB.getSession();
	return session.userPreferences;
}

/**
 * Set language preference
 */
export async function setLanguagePreference(language: string | null): Promise<void> {
	const session = await sessionDB.getSession();
	session.userPreferences.language = language;
	await sessionDB.saveSession(session);
}

/**
 * Set theme preference
 */
export async function setThemePreference(theme: 'light' | 'dark' | null): Promise<void> {
	const session = await sessionDB.getSession();
	session.userPreferences.theme = theme;
	await sessionDB.saveSession(session);
}

/**
 * Set user preferences (batch update)
 */
export async function setUserPreferences(preferences: {
	language?: string | null;
	theme?: 'light' | 'dark' | null;
}): Promise<void> {
	const session = await sessionDB.getSession();
	if (preferences.language !== undefined) {
		session.userPreferences.language = preferences.language;
	}
	if (preferences.theme !== undefined) {
		session.userPreferences.theme = preferences.theme;
	}
	await sessionDB.saveSession(session);
}
