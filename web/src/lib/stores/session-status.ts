/**
 * Session Status Store - Global session expiration state
 *
 * Single source of truth for session expiration status across all components.
 * Updated by layout on route changes, consumed by AuthStatusButton for styling.
 */

import { writable } from 'svelte/store';

interface SessionStatus {
	/** True if session is expired or no valid session exists */
	isExpired: boolean;
	/** Timestamp of last check */
	lastChecked: number;
}

const defaultStatus: SessionStatus = {
	isExpired: false,
	lastChecked: 0
};

function createSessionStatusStore() {
	const { subscribe, set, update } = writable<SessionStatus>(defaultStatus);

	return {
		subscribe,

		/**
		 * Mark session as expired (triggers yellow background in AuthStatus)
		 */
		markExpired(): void {
			update((status) => ({
				...status,
				isExpired: true,
				lastChecked: Date.now()
			}));
		},

		/**
		 * Mark session as valid (clears yellow background in AuthStatus)
		 */
		markValid(): void {
			update((status) => ({
				...status,
				isExpired: false,
				lastChecked: Date.now()
			}));
		},

		/**
		 * Reset to default state
		 */
		reset(): void {
			set(defaultStatus);
		}
	};
}

export const sessionStatusStore = createSessionStatusStore();
