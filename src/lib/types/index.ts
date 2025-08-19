// API types matching the backend
export type AlphabetType = 'base58' | 'no-look-alike' | 'full' | 'full-with-symbols';

export interface GenerateParams {
	length?: number;
	alphabet?: AlphabetType;
	prefix?: string;
	suffix?: string;
	raw?: boolean;
}

export interface PasswordParams {
	length?: number;
	alphabet?: 'no-look-alike' | 'full-with-symbols';
	raw?: boolean;
}

export interface ApiKeyParams {
	length?: number;
	alphabet?: 'no-look-alike' | 'full';
	raw?: boolean;
}

export interface VersionResponse {
	api_version: string;
	ui_version: string;
}

// Navigation types
export interface NavItem {
	id: string;
	title: string;
	description: string;
	path: string;
	icon: string;
}

// Result state
export interface ResultState {
	value: string;
	params: Record<string, any>;
	endpoint: string;
	timestamp: Date;
}

// Translation support
export interface I18nTexts {
	[key: string]: string | I18nTexts;
}