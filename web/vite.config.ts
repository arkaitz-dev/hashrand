import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import eslint from 'vite-plugin-eslint';

export default defineConfig(({ command, mode }) => {
	const isProduction = mode === 'production';
	const isLintOnly = process.env.VITE_LINT_ONLY === 'true';

	return {
		plugins: [
			tailwindcss(),
			sveltekit(),
			eslint({
				cache: false,
				include: ['**/*.{ts,js,svelte}'],
				exclude: ['node_modules/**', 'dist/**', 'build/**', '.svelte-kit/**'],
				emitWarning: true,
				emitError: true,
				// En modo lint-only o producción, fallar en errores
				failOnError: isLintOnly || isProduction,
				failOnWarning: false
			})
		],
		server: {
			port: 5173,
			host: '0.0.0.0',
			allowedHosts: ['elite.faun-pirate.ts.net', 'localhost', '127.0.0.1'],
			proxy: {
				'/api': {
					target: 'http://127.0.0.1:3000',
					changeOrigin: true,
					secure: false
				}
			}
		}
	};
});
