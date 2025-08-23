import { writable } from 'svelte/store';
import type { NavItem } from '$lib/types';

export const currentRoute = writable<string>('/');
export const previousRoute = writable<string | null>(null);

export const navigationItems: NavItem[] = [
	{
		id: 'custom',
		title: 'Custom Hash',
		description: 'Generate customized random hashes with various parameters',
		path: '/custom',
		icon: '🎲'
	},
	{
		id: 'password',
		title: 'Secure Password',
		description: 'Generate secure passwords with guaranteed entropy',
		path: '/password',
		icon: '🔐'
	},
	{
		id: 'api-key',
		title: 'API Key',
		description: 'Generate API keys with ak_ prefix for applications',
		path: '/api-key',
		icon: '🔑'
	}
];

export function navigateTo(path: string) {
	previousRoute.update((current) => {
		currentRoute.set(path);
		return current;
	});
}
