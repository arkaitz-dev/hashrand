/// <reference types="vite/client" />

declare module 'vite-plugin-eslint' {
	interface ESLintOptions {
		cache?: boolean;
		include?: string[];
		exclude?: string[];
		emitWarning?: boolean;
		emitError?: boolean;
		failOnError?: boolean;
		failOnWarning?: boolean;
	}

	function eslint(options?: ESLintOptions): unknown;
	export default eslint;
}

// Logging system environment variables
interface ImportMetaEnv {
	readonly VITE_LOG_LEVEL?: 'error' | 'warn' | 'info' | 'debug';
}

interface ImportMeta {
	readonly env: ImportMetaEnv;
}
