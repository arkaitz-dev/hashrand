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
