import { redirect } from '@sveltejs/kit';
import { browser } from '$app/environment';
import type { LoadEvent } from '@sveltejs/kit';

const DAY_MS = 24 * 60 * 60 * 1000; // 1 day in milliseconds

export const load = async ({ url }: LoadEvent) => {
	if (url.pathname === '/noOllama') return;

	if (browser) {
		const cacheKey = 'ollama_check_cache';
		const cached = localStorage.getItem(cacheKey);

		// Parse cache if it exists
		let cache: { timestamp: number; installed: boolean } | null = null;
		try {
			if (cached) cache = JSON.parse(cached);
		} catch {
			localStorage.removeItem(cacheKey);
		}

		const needsCheck =
			!cache || Date.now() - cache.timestamp > DAY_MS;

		if (needsCheck) {
			const { invoke } = await import('@tauri-apps/api/core');
			const installed = await invoke<boolean>('check_ollama_installed');

			localStorage.setItem(
				cacheKey,
				JSON.stringify({ timestamp: Date.now(), installed })
			);

			if (!installed) throw redirect(307, '/noOllama');
		} else {
			// use cached result
			// ensure cache is not null before accessing its properties
			if (!cache || !cache.installed) throw redirect(307, '/noOllama');
		}
	}
};
