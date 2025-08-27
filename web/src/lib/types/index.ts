// API types matching the backend
export type AlphabetType = 'base58' | 'no-look-alike' | 'full' | 'full-with-symbols' | 'numeric';

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

export interface MnemonicParams {
	language?: string;
	words?: 12 | 24;
	raw?: boolean;
}

// POST request body for seeded generation
export interface SeedGenerateRequest {
	length?: number;
	alphabet?: AlphabetType;
	prefix?: string;
	suffix?: string;
	seed: string; // 64-character hexadecimal string
	endpoint: string; // original endpoint that was requested
}

export interface SeedPasswordRequest {
	length?: number;
	alphabet?: 'no-look-alike' | 'full-with-symbols';
	seed: string; // 64-character hexadecimal string
}

export interface SeedApiKeyRequest {
	length?: number;
	alphabet?: 'no-look-alike' | 'full';
	seed: string; // 64-character hexadecimal string
}

export interface SeedMnemonicRequest {
	language?: string;
	words?: 12 | 24;
	seed: string; // 64-character hexadecimal string
}

// API Response types
// NOTE: HashResponse deprecated in favor of CustomHashResponse for all endpoints

export interface CustomHashResponse {
	hash: string;
	seed: string;
	otp: string; // 9-digit OTP
	timestamp: number; // Unix timestamp in seconds
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
	svgIcon?: string;
}

// Result state - updated to support both string and JSON responses
export interface ResultState {
	value: string;
	seed?: string; // Hexadecimal seed when available
	otp?: string; // 9-digit OTP (only for custom endpoint)
	timestamp: Date;
	params: Record<string, string | number | boolean>;
	endpoint: string;
}

// Translation support
export interface I18nTexts {
	[key: string]: string | I18nTexts;
}
