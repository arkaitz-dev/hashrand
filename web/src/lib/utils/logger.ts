/**
 * Professional logging system with level-based filtering + terminal redirection
 *
 * Architecture:
 * - Development: Configurable via VITE_LOG_LEVEL (default: INFO)
 *   - Browser console: Logs visible in DevTools
 *   - Server terminal: Logs redirected via WebSocket (CRITICAL for tablet dev)
 * - Production: ALL console.* eliminated by terser (zero overhead)
 *
 * Log levels (inspired by Rust tracing):
 * - error(0): Critical errors, security violations
 * - warn(1):  Anomalous situations, potential issues
 * - info(2):  Normal operations, successful flows (DEFAULT)
 * - debug(3): Verbose troubleshooting information
 *
 * Usage:
 * ```typescript
 * import { logger } from '$lib/utils/logger';
 *
 * logger.error('Auth failed:', error);
 * logger.warn('Token expiring soon');
 * logger.info('User logged in');
 * logger.debug('Request payload:', data);
 * ```
 *
 * Environment:
 * - VITE_LOG_LEVEL=error   → Only errors
 * - VITE_LOG_LEVEL=warn    → Errors + warnings
 * - VITE_LOG_LEVEL=info    → Errors + warnings + info (DEFAULT)
 * - VITE_LOG_LEVEL=debug   → ALL logs (verbose)
 *
 * Terminal Redirection (Development Only):
 * - Logs sent to server terminal via Vite WebSocket
 * - Enables debugging from tablet without DevTools
 * - Zero overhead in production (code eliminated by terser)
 */

type LogLevel = 'error' | 'warn' | 'info' | 'debug';

const LOG_LEVELS: Record<LogLevel, number> = {
	error: 0,
	warn: 1,
	info: 2,
	debug: 3
} as const;

/**
 * Get current log level from environment
 * Production: Returns error (irrelevant - all console.* eliminated)
 * Development: Reads VITE_LOG_LEVEL, defaults to INFO
 */
function getCurrentLevel(): number {
	// In production, this code is eliminated by esbuild
	if (import.meta.env.PROD) {
		return LOG_LEVELS.error; // Irrelevant - console.* removed
	}

	// Development: Read from environment
	const envLevel = import.meta.env.VITE_LOG_LEVEL?.toLowerCase() as LogLevel | undefined;
	if (envLevel && envLevel in LOG_LEVELS) {
		return LOG_LEVELS[envLevel];
	}

	// DEFAULT: INFO level
	return LOG_LEVELS.info;
}

/**
 * Check if a message at given level should be logged
 */
function shouldLog(level: LogLevel): boolean {
	const currentLevel = getCurrentLevel();
	const messageLevel = LOG_LEVELS[level];
	return messageLevel <= currentLevel;
}

/**
 * Send log to server terminal via Vite WebSocket
 * Only in development - code eliminated in production by terser
 */
function sendToTerminal(level: LogLevel, args: unknown[]): void {
	// This code is eliminated in production by terser drop_console
	if (import.meta.env.DEV && import.meta.hot) {
		try {
			import.meta.hot.send('terminal:log', {
				type: 'log',
				level,
				args,
				timestamp: Date.now()
			});
		} catch {
			// WebSocket not ready or error - silent fail
		}
	}
}

/**
 * Logger interface with level-based filtering + terminal redirection
 * All console.* calls eliminated in production builds
 */
export const logger = {
	/**
	 * Log critical errors, security violations
	 * Always visible (unless VITE_LOG_LEVEL explicitly excludes)
	 */
	error(...args: unknown[]): void {
		if (shouldLog('error')) {
			console.error('[ERROR]', ...args);
			sendToTerminal('error', args);
		}
	},

	/**
	 * Log warnings, anomalous situations
	 * Visible at warn/info/debug levels
	 */
	warn(...args: unknown[]): void {
		if (shouldLog('warn')) {
			console.warn('[WARN]', ...args);
			sendToTerminal('warn', args);
		}
	},

	/**
	 * Log normal operations, successful flows
	 * Visible at info/debug levels (DEFAULT)
	 */
	info(...args: unknown[]): void {
		if (shouldLog('info')) {
			console.info('[INFO]', ...args);
			sendToTerminal('info', args);
		}
	},

	/**
	 * Log verbose debugging information
	 * Only visible at debug level
	 */
	debug(...args: unknown[]): void {
		if (shouldLog('debug')) {
			console.debug('[DEBUG]', ...args);
			sendToTerminal('debug', args);
		}
	}
};

/**
 * Export type for external use
 */
export type Logger = typeof logger;
