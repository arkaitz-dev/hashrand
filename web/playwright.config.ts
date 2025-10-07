/**
 * Playwright Configuration for HashRand Spin E2E Tests
 *
 * Configuration for timing-sensitive authentication and key rotation tests
 * Matches patterns from bash test scripts (scripts/final_test.sh, scripts/test_2_3_system.sh)
 */

import { defineConfig, devices } from '@playwright/test';

/**
 * Environment configuration
 */
const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3000';
const WEB_BASE_URL = process.env.WEB_BASE_URL || 'http://localhost:5173';
const CI = !!process.env.CI;

export default defineConfig({
	/**
	 * Test directory
	 */
	testDir: './tests',

	/**
	 * Global setup/teardown for email dry-run mode
	 * Executed ONCE before all tests start and after all tests complete
	 */
	globalSetup: './tests/global-setup.ts',
	globalTeardown: './tests/global-teardown.ts',

	/**
	 * Maximum time one test can run for (5 minutes for key rotation tests)
	 * Key rotation tests need ~3 minutes for full 2/3 system cycle
	 */
	timeout: 300_000, // 5 minutes

	/**
	 * Expect timeout for assertions
	 */
	expect: {
		timeout: 10_000 // 10 seconds
	},

	/**
	 * Run tests sequentially (not in parallel)
	 * Required because:
	 * 1. Timing-sensitive tests (token expiration, key rotation)
	 * 2. Shared backend state (JWT tokens, sessions)
	 * 3. Tests depend on specific timing windows
	 */
	fullyParallel: false,

	/**
	 * Fail the build on CI if you accidentally left test.only in the source code
	 */
	forbidOnly: CI,

	/**
	 * Retry on CI only
	 */
	retries: CI ? 2 : 0,

	/**
	 * Single worker for timing-sensitive tests
	 * Key rotation tests require precise timing control
	 */
	workers: 1,

	/**
	 * Reporter configuration
	 */
	reporter: [
		['html', { outputFolder: './tests/playwright-report' }],
		['list'] // Show progress in terminal
	],

	/**
	 * Shared settings for all projects
	 */
	use: {
		/**
		 * Base URL for navigation
		 */
		baseURL: WEB_BASE_URL,

		/**
		 * Trace collection (for debugging)
		 */
		trace: 'on-first-retry',

		/**
		 * Screenshot on failure
		 */
		screenshot: 'only-on-failure',

		/**
		 * Video recording
		 */
		video: 'retain-on-failure',

		/**
		 * Action timeout (e.g., click, fill)
		 */
		actionTimeout: 10_000 // 10 seconds
	},

	/**
	 * Test projects (browsers)
	 * Start with Chromium only, can add more later
	 */
	projects: [
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		}
	],

	/**
	 * Web server configuration
	 * Automatically starts dev servers before running tests
	 *
	 * IMPORTANT: Tests assume both API and Web servers are running
	 * Alternative: Use `just dev` to start all services manually
	 */
	webServer: CI
		? [
				// CI: Start both servers explicitly
				{
					command: 'cd ../api && spin-cli up --listen 127.0.0.1:3000',
					url: API_BASE_URL,
					reuseExistingServer: false,
					timeout: 120_000,
					stdout: 'pipe',
					stderr: 'pipe'
				},
				{
					command: 'npm run dev',
					url: WEB_BASE_URL,
					reuseExistingServer: false,
					timeout: 120_000,
					stdout: 'pipe',
					stderr: 'pipe'
				}
			]
		: [
				// Local development: Reuse existing servers from `just dev`
				{
					command: 'echo "Reusing existing dev servers (started with just dev)"',
					url: WEB_BASE_URL,
					reuseExistingServer: true,
					timeout: 5_000
				}
			]
});
