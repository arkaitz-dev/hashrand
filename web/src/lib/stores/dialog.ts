import { writable } from 'svelte/store';

export interface DialogData {
	type: 'auth' | 'confirmation' | 'custom';
	props?: Record<string, unknown>;
	id: string;
}

function createDialogStore() {
	const { subscribe, set, update } = writable<DialogData | null>(null);

	return {
		subscribe,

		// Show a dialog
		show: (type: DialogData['type'], props?: Record<string, unknown>) => {
			console.log('[DEBUG] DialogStore: show() called with:', { type, props });
			const dialog: DialogData = {
				type,
				props: props || {},
				id: globalThis.crypto?.randomUUID() || Math.random().toString(36).substring(2, 15)
			};
			console.log('[DEBUG] DialogStore: setting dialog to:', dialog);
			set(dialog);
			console.log('[DEBUG] DialogStore: dialog set completed');
		},

		// Close/clear dialog
		close: () => {
			console.log(new Date().toISOString() + ': dialogStore.close() called');
			set(null);
			console.log(new Date().toISOString() + ': dialogStore.close() set(null) completed');
		},

		// Update dialog props
		update: (props: Record<string, unknown>) => {
			update(current => 
				current ? { ...current, props: { ...current.props, ...props } } : null
			);
		}
	};
}

export const dialogStore = createDialogStore();