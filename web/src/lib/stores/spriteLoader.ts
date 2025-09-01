/**
 * Sprite loader store for managing SVG sprite state
 * Works with global variable set by HTML defer script
 */

/* eslint-env browser */
import { writable } from 'svelte/store';

// Sprite loading state interface
export interface SpriteState {
	loaded: boolean;
	loading: boolean;
	error: boolean;
}

// Get initial state from global variable or default
function getInitialState(): SpriteState {
	if (typeof window !== 'undefined' && window.__SPRITE_STATE__) {
		return window.__SPRITE_STATE__;
	}
	return {
		loaded: false,
		loading: true,
		error: false
	};
}

// Create the store with initial state
export const spriteState = writable<SpriteState>(getInitialState());

/**
 * Initialize sprite state synchronization with global variable
 * Listens for custom events dispatched by the defer script
 */
export function initializeSpriteLoader(): void {
	if (typeof window === 'undefined') return;

	// Sync initial state
	if (window.__SPRITE_STATE__) {
		spriteState.set(window.__SPRITE_STATE__);
	}

	// Listen for sprite loaded event
	window.addEventListener('sprite-loaded', () => {
		spriteState.set(window.__SPRITE_STATE__);
	});

	// Listen for sprite error event
	window.addEventListener('sprite-error', (event: Event) => {
		spriteState.set(window.__SPRITE_STATE__);

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const customEvent = event as any;
		console.error('[SpriteLoader] Sprite loading failed:', customEvent.detail);
	});

	// Fallback: poll global state until loaded
	// eslint-disable-next-line no-undef
	const pollInterval = setInterval(() => {
		if (window.__SPRITE_STATE__ && window.__SPRITE_STATE__.loaded) {
			spriteState.set(window.__SPRITE_STATE__);
			// eslint-disable-next-line no-undef
			clearInterval(pollInterval);
		} else if (window.__SPRITE_STATE__ && window.__SPRITE_STATE__.error) {
			spriteState.set(window.__SPRITE_STATE__);
			// eslint-disable-next-line no-undef
			clearInterval(pollInterval);
		}
	}, 100);

	// Clear polling after reasonable timeout
	setTimeout(() => {
		// eslint-disable-next-line no-undef
		clearInterval(pollInterval);
	}, 5000);
}
