// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	// Global sprite state from HTML defer script
	interface Window {
		__SPRITE_STATE__: {
			loaded: boolean;
			loading: boolean;
			error: boolean;
		};
	}
}

export {};
