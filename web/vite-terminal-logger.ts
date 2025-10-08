/**
 * Vite Plugin: Terminal Logger
 *
 * Redirects browser console logs to server terminal during development.
 * Critical for tablet-based development without DevTools access.
 *
 * Architecture:
 * - Browser: logger.* sends logs via WebSocket
 * - Server: Vite WebSocket receives and prints to terminal
 * - Production: Code eliminated by terser (zero overhead)
 *
 * Usage: Add to vite.config.ts plugins array (dev only)
 */

import type { Plugin, ViteDevServer } from 'vite';

interface LogMessage {
	type: 'log';
	level: 'error' | 'warn' | 'info' | 'debug';
	args: unknown[];
	timestamp: number;
}

/**
 * Terminal logger plugin for Vite development server
 * Receives browser logs via WebSocket and prints to terminal
 */
export function terminalLogger(): Plugin {
	let server: ViteDevServer;

	return {
		name: 'vite-terminal-logger',
		apply: 'serve', // Only in development

		configureServer(devServer) {
			server = devServer;

			// Listen for custom WebSocket messages
			server.ws.on('terminal:log', (data: LogMessage) => {
				const { level, args, timestamp } = data;
				const time = new Date(timestamp).toISOString().split('T')[1].split('.')[0];

				// Format with color codes for terminal
				const colors = {
					error: '\x1b[31m', // Red
					warn: '\x1b[33m', // Yellow
					info: '\x1b[36m', // Cyan
					debug: '\x1b[90m' // Gray
				};
				const reset = '\x1b[0m';
				const color = colors[level];

				// Print to server terminal (only show [BROWSER ERROR] prefix for errors)
				const prefix =
					level === 'error'
						? `${color}[${time}] [BROWSER ERROR]${reset}`
						: `${color}[${time}]${reset}`;
				console.log(prefix, ...args);
			});
		}
	};
}
