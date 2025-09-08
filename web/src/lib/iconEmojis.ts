/**
 * Mapping of icon names to UTF emoji placeholders
 * For use as placeholders while SVG sprite is loading
 */

export const iconEmojis: Record<string, string> = {
	// Theme icons
	sun: '☀️', // Light mode icon
	moon: '🌙', // Dark mode icon

	// Navigation icons
	'arrow-left': '>', // Simple arrow for choose buttons (LTR) - flips to < in RTL
	'arrow-right': '>', // Simple arrow for choose buttons (RTL) - flips to < in RTL
	'chevron-down': '🔽', // Chevron down for expandable sections
	home: '🏠', // Home/house icon for back to menu buttons

	// UI icons
	heart: '❤️', // Heart for "made with love"
	check: '✅', // Checkmark for success states
	copy: '📋', // Copy to clipboard
	refresh: '🔄', // Refresh/regenerate icon
	settings: '⚙️', // Settings/gear icon
	play: '▶️', // Play button for generate actions
	user: '👤', // User silhouette for authentication

	// Flag emojis
	// Countries with standard UTF flag emojis
	saudi: '🇸🇦', // Saudi Arabia (Arabic)
	germany: '🇩🇪', // Germany (Deutsch)
	uk: '🇬🇧', // United Kingdom (English)
	spain: '🇪🇸', // Spain (Español)
	france: '🇫🇷', // France (Français)
	india: '🇮🇳', // India (Hindi)
	japan: '🇯🇵', // Japan (日本語)
	portugal: '🇵🇹', // Portugal (Português)
	russia: '🇷🇺', // Russia (Русский)
	china: '🇨🇳', // China (中文)

	// Regional flags without standard UTF emojis - use white flag
	catalonia: '🏳️', // Catalonia (no UTF emoji) - use white flag
	basque: '🏳️', // Basque Country (no UTF emoji) - use white flag
	galicia: '🏳️', // Galicia (no UTF emoji) - use white flag

	// Generic fallback for any unknown icon
	unknown: '❌'
};

/**
 * Get UTF emoji for an icon name
 * @param iconName - The icon name
 * @returns UTF emoji string or fallback
 */
export function getIconEmoji(iconName: string): string {
	return iconEmojis[iconName] || iconEmojis.unknown;
}

/**
 * Check if an icon has a proper UTF emoji (not the generic fallback)
 * @param iconName - The icon name
 * @returns true if has proper emoji, false if using fallback
 */
export function hasProperIconEmoji(iconName: string): boolean {
	const emoji = iconEmojis[iconName];
	return Boolean(emoji && emoji !== iconEmojis.unknown);
}

// Note: This file was renamed from flagEmojis.ts to iconEmojis.ts to better reflect
// its broader scope beyond just flag icons
