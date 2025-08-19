import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// Configure as SPA with fallback for client-side routing
		adapter: adapter({
			pages: 'web/dist',
			assets: 'web/dist',
			fallback: 'index.html',
			precompress: false,
			strict: false
		})
	}
};

export default config;
