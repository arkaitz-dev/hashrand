import { writable } from 'svelte/store';

export interface DialogData {
	type:
		| 'auth'
		| 'auth-confirm'
		| 'seed'
		| 'logout'
		| 'confirmation'
		| 'custom'
		| 'magic-link-error';
	props?: Record<string, unknown>;
	id: string;
}

function createDialogStore() {
	const { subscribe, set, update } = writable<DialogData | null>(null);

	return {
		subscribe,

		// Show a dialog
		show: (type: DialogData['type'], props?: Record<string, unknown>) => {
			const dialog: DialogData = {
				type,
				props: props || undefined, // Use undefined instead of {} to preserve null semantics
				id: globalThis.crypto?.randomUUID() || Math.random().toString(36).substring(2, 15)
			};
			set(dialog);
		},

		// Close/clear dialog
		close: () => {
			set(null);
		},

		// Update dialog props
		update: (props: Record<string, unknown>) => {
			update((current) => (current ? { ...current, props: { ...current.props, ...props } } : null));
		}
	};
}

export const dialogStore = createDialogStore();
