import { writable } from 'svelte/store';
import type { ResultState } from '$lib/types';

export const resultState = writable<ResultState | null>(null);
export const isLoading = writable<boolean>(false);
export const error = writable<string | null>(null);

export function setResult(result: ResultState) {
	resultState.set(result);
	error.set(null);
}

export function setError(errorMessage: string) {
	error.set(errorMessage);
	isLoading.set(false);
}

export function clearResult() {
	resultState.set(null);
	error.set(null);
}

export function setLoading(loading: boolean) {
	isLoading.set(loading);
	if (loading) {
		error.set(null);
	}
}