import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import typescript from '@typescript-eslint/eslint-plugin';
import parser from '@typescript-eslint/parser';
import svelteParser from 'svelte-eslint-parser';
import prettier from 'eslint-config-prettier';

export default [
	js.configs.recommended,
	{
		ignores: ['dist/', 'build/', '.svelte-kit/', 'node_modules/']
	},
	{
		files: ['**/*.{ts,js}'],
		languageOptions: {
			parser: parser,
			parserOptions: {
				ecmaVersion: 2022,
				sourceType: 'module'
			},
			globals: {
				console: 'readonly',
				fetch: 'readonly',
				Response: 'readonly',
				URLSearchParams: 'readonly',
				window: 'readonly',
				document: 'readonly',
				navigator: 'readonly',
				localStorage: 'readonly',
				setTimeout: 'readonly',
				clearTimeout: 'readonly',
				setInterval: 'readonly',
				clearInterval: 'readonly',
				Event: 'readonly',
				MouseEvent: 'readonly',
				Element: 'readonly',
				HTMLTextAreaElement: 'readonly',
				HTMLSelectElement: 'readonly',
				HTMLInputElement: 'readonly',
				TextEncoder: 'readonly',
				TextDecoder: 'readonly',
				RequestInit: 'readonly',
				crypto: 'readonly',
				EventListener: 'readonly'
			}
		},
		plugins: {
			'@typescript-eslint': typescript
		},
		rules: {
			...typescript.configs.recommended.rules,
			'no-unused-vars': 'warn',
			'prefer-const': 'warn',
			'@typescript-eslint/no-explicit-any': 'warn',
			'@typescript-eslint/ban-ts-comment': 'warn'
		}
	},
	{
		files: ['**/*.svelte'],
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: parser,
				ecmaVersion: 2022,
				sourceType: 'module'
			},
			globals: {
				console: 'readonly',
				fetch: 'readonly',
				Response: 'readonly',
				URLSearchParams: 'readonly',
				window: 'readonly',
				document: 'readonly',
				navigator: 'readonly',
				localStorage: 'readonly',
				setTimeout: 'readonly',
				clearTimeout: 'readonly',
				setInterval: 'readonly',
				clearInterval: 'readonly',
				Event: 'readonly',
				MouseEvent: 'readonly',
				Element: 'readonly',
				HTMLTextAreaElement: 'readonly',
				HTMLSelectElement: 'readonly',
				HTMLInputElement: 'readonly',
				TextEncoder: 'readonly',
				TextDecoder: 'readonly',
				RequestInit: 'readonly',
				crypto: 'readonly',
				EventListener: 'readonly'
			}
		},
		plugins: {
			svelte,
			'@typescript-eslint': typescript
		},
		rules: {
			...svelte.configs.recommended.rules,
			'no-unused-vars': 'off',
			'@typescript-eslint/no-unused-vars': 'warn',
			'@typescript-eslint/no-explicit-any': 'warn',
			'@typescript-eslint/ban-ts-comment': 'warn'
		}
	},
	prettier
];
