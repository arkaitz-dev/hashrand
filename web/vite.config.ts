import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		port: 5173,
		host: '0.0.0.0',
		allowedHosts: [
			'elite.faun-pirate.ts.net',
			'localhost',
			'127.0.0.1'
		],
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:3000',
				changeOrigin: true,
				secure: false
			}
		}
	}
});