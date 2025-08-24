import type { I18nTexts } from '$lib/types';

export const en: I18nTexts = {
	common: {
		back: 'Back',
		generate: 'Generate',
		copy: 'Copy',
		copied: 'Copied!',
		backToMenu: 'Back to Menu',
		loading: 'Generating...',
		error: 'Error occurred',
		result: 'Result',
		choose: 'Choose',
		type: 'Type',
		length: 'Length',
		generated: 'Generated',
		format: 'Format',
		security: 'Security',
		loadingVersion: 'Loading version...',
		versionsUnavailable: 'Versions unavailable',
		generationDetails: 'Generation Details',
		parametersUsed: 'Parameters Used',
		generateAnother: 'Generate Another',
		adjustSettings: 'Adjust Settings',
		navigateTo: 'Navigate to',
		selectLanguage: 'Select language',
		switchToLightMode: 'Switch to light mode',
		switchToDarkMode: 'Switch to dark mode',
		characters: 'letters',
		alphabet: 'Alphabet',
		generatedValue: 'Generated Value',
		clickToSelect: 'Click the text area to select all, or use the copy button',
		waitGenerating: 'Please wait while generating new value...',
		unknownEndpoint: 'Unknown endpoint type',
		failedToCopy: 'Failed to copy',
		fallbackCopyFailed: 'Fallback copy failed',
		failedToRegenerate: 'Failed to regenerate',
		failedToLoadVersions: 'Failed to load versions',
		mustBeBetween: 'must be between',
		and: 'and',
		cannotExceed: 'cannot exceed',
		optionalPrefix: 'Optional prefix',
		optionalSuffix: 'Optional suffix',
		prefixCannotExceed: 'Prefix cannot exceed 32 letters',
		suffixCannotExceed: 'Suffix cannot exceed 32 letters',
		yes: 'Yes',
		no: 'No',
		seedUsed: 'Seed Used',
		copySeed: 'Copy Seed',
		optionalSeed: 'Optional seed (64 hex characters)',
		seedInvalid: 'Seed must be exactly 64 hexadecimal characters',
		reuseSeedTitle: 'Reuse the same seed?',
		reuseSeedMessage:
			'Do you want to reuse the same seed to generate the same result, or do you prefer to generate a new random seed?',
		keepSameSeed: 'Keep the same seed',
		generateNewSeed: 'Generate new seed',
		seed: 'Seed',
		otp: 'OTP'
	},
	menu: {
		title: 'Hash Generator',
		subtitle: 'Choose a generation method',
		version: 'Version',
		brandName: 'HashRand Spin',
		description: 'Cryptographically secure hash, password, and API key generator'
	},
	custom: {
		title: 'Custom Hash Generator',
		description: 'Generate customized random hashes',
		generateHash: 'Generate Hash',
		length: 'Length',
		alphabet: 'Alphabet Type',
		prefix: 'Prefix',
		suffix: 'Suffix',
		lengthMustBeBetween: 'Length must be between 2 and 128',
		bitcoinDescription: 'Bitcoin alphabet, excludes confusing letters',
		maxReadabilityDescription: 'Maximum readability, 49 letters',
		completeAlphanumericDescription: 'Complete alphanumeric set',
		maxEntropyDescription: 'Maximum entropy with symbols',
		numericDescription: 'Only digits 0-9, requires longer length',
		failedToGenerateHash: 'Failed to generate hash'
	},
	password: {
		title: 'Secure Password Generator',
		description: 'Generate secure passwords',
		generatePassword: 'Generate Password',
		length: 'Length',
		alphabet: 'Character Set',
		maxSecurityDescription: 'Maximum security with symbols (73 chars)',
		easyReadDescription: 'Easy to read and type (49 chars)',
		securityNote: 'Security Note:',
		securityDescription:
			'Passwords are generated using cryptographically secure random generation. They are not stored or logged anywhere.',
		noLookAlikeNote:
			'No Look-alike alphabet excludes confusable letters. Minimum {0} letters for equivalent security.',
		fullAlphabetNote:
			'Full alphabet with symbols provides maximum entropy. Minimum {0} letters for strong security.',
		failedToGeneratePassword: 'Failed to generate password'
	},
	apiKey: {
		title: 'API Key Generator',
		description: 'Generate API keys with ak_ prefix',
		generateApiKey: 'Generate API Key',
		length: 'Length',
		alphabet: 'Character Set',
		standardAlphanumericDescription: 'Standard alphanumeric (62 chars)',
		noConfusingDescription: 'No confusing letters (49 chars)',
		formatNotice:
			'All API keys are generated with the "ak_" prefix for easy identification. The specified length refers only to the random letters generated (prefix not counted).',
		securityNotice:
			'Store API keys securely and never expose them in client-side code or version control. Treat them with the same care as passwords.',
		randomCharacters: 'random characters using',
		noLookAlikeAlphabet: '(easy to type)',
		fullAlphanumericAlphabet: '(maximum compatibility)',
		noLookAlikeNote:
			'No Look-alike excludes confusing characters. Minimum {0} characters for equivalent security.',
		fullAlphanumericNote:
			'Full alphanumeric provides maximum compatibility. Minimum {0} characters for strong security.',
		failedToGenerateApiKey: 'Failed to generate API key'
	},
	alphabets: {
		base58: 'Base58 (Bitcoin alphabet)',
		'no-look-alike': 'No Look-alike',
		full: 'Full Alphanumeric',
		'full-with-symbols': 'Full with Symbols',
		numeric: 'Numeric (0-9)'
	}
};
