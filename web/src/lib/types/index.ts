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

// Authentication types
export interface AuthUser {
	user_id: string; // Base58 user_id
	email: string; // User email (for UX display, Zero Knowledge compliant)
	isAuthenticated: boolean;
}

export interface LoginRequest {
	email: string;
	ui_host: string; // Frontend URL for magic link generation (REQUIRED)
	next?: string; // Simple URL path to redirect to after authentication
	email_lang: string; // Language code for email template (e.g., "es", "en") - REQUIRED, matches user selection
	pub_key: string; // Ed25519 public key (64-character hex string, 32 bytes) - REQUIRED
	signature: string; // Ed25519 signature of email + pub_key message (128-character hex string, 64 bytes) - REQUIRED
}

export interface LoginResponse {
	access_token: string;
	token_type: string;
	user_id: string; // Base58 user_id
	next?: string; // Optional next parameter from magic link
	expires_at?: number; // Optional refresh cookie expiration timestamp (when new refresh cookie is set)
	server_pub_key?: string; // Optional server public key (only in 2/3 time window for key rotation)
}

export interface MagicLinkResponse {
	status: string;
	dev_magic_link?: string; // Development-only field for easy testing
}

export interface AuthError {
	error: string;
}

// Shared Secret types
export interface CreateSharedSecretRequest {
	sender_email: string;
	receiver_email: string;
	secret_text: string;
	expires_hours: number; // 1-72
	max_reads: number; // 1-10
	require_otp: boolean;
	send_copy_to_sender: boolean;
	receiver_language?: string; // Optional: language for receiver email (defaults to 'en')
	sender_language?: string; // Optional: language for sender copy email (defaults to 'en')
	ui_host: string; // Required: UI hostname for URL generation (e.g., "localhost" or "app.domain.com")
}

export interface CreateSharedSecretResponse {
	url_sender: string; // Full URL with hash
	url_receiver: string; // Full URL with hash
	reference: string; // Base58 reference hash (16 bytes)
	otp?: string; // 9-digit OTP if require_otp is true
}

export interface SharedSecretPayload {
	sender_email: string;
	receiver_email: string;
	secret_text: string;
	otp?: string; // 9-digit OTP if required
	created_at: number; // Unix timestamp
}

export interface ViewSharedSecretRequest {
	otp?: string; // Required only if secret requires OTP
}

export interface ViewSharedSecretResponse {
	secret_text: string;
	sender_email: string;
	receiver_email: string;
	pending_reads: number; // -1 for sender (unlimited), positive for receiver
	max_reads: number; // Maximum reads allowed (from encrypted payload)
	expires_at: number; // Unix timestamp
	reference: string; // Base58 reference hash
	role: 'sender' | 'receiver';
}

// Shared Secret Error Responses (returned as HTTP 200 with error field in SignedResponse)
export interface SharedSecretOtpRequiredError {
	error: 'OTP_REQUIRED';
	message: string;
}

export interface SharedSecretInvalidOtpError {
	error: 'INVALID_OTP';
	message: string;
}

// Union type for viewSharedSecret response (success or error)
export type ViewSharedSecretResult =
	| ViewSharedSecretResponse
	| SharedSecretOtpRequiredError
	| SharedSecretInvalidOtpError;
