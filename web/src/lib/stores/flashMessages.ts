import { writable } from 'svelte/store';

export interface FlashMessage {
	id: string;
	content: string;
	timestamp: number;
}

function createFlashMessagesStore() {
	const { subscribe, set, update } = writable<FlashMessage[]>([]);

	return {
		subscribe,

		// Add a single message
		addMessage: (content: string) => {
			const message: FlashMessage = {
				id: globalThis.crypto?.randomUUID() || Math.random().toString(36).substring(2, 15),
				content,
				timestamp: Date.now()
			};

			update((messages) => [...messages, message]);
		},

		// Add multiple messages
		addMessages: (contents: string[]) => {
			const newMessages: FlashMessage[] = contents.map((content) => ({
				id: globalThis.crypto?.randomUUID() || Math.random().toString(36).substring(2, 15),
				content,
				timestamp: Date.now()
			}));

			update((messages) => [...messages, ...newMessages]);
		},

		// Clear all messages
		clear: () => {
			set([]);
		},

		// Set messages (replace all)
		set: (contents: string[]) => {
			const newMessages: FlashMessage[] = contents.map((content) => ({
				id: globalThis.crypto?.randomUUID() || Math.random().toString(36).substring(2, 15),
				content,
				timestamp: Date.now()
			}));

			set(newMessages);
		}
	};
}

export const flashMessagesStore = createFlashMessagesStore();
