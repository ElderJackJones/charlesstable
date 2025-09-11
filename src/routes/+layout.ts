// src/routes/+layout.ts
import { redirect } from '@sveltejs/kit';
import { browser } from '$app/environment';
import type { LoadEvent } from '@sveltejs/kit';

export const load = async ({ url }: LoadEvent) => {
    if (url.pathname === '/noOllama') return;

    if (browser) {
        const { invoke } = await import('@tauri-apps/api/core');
        const installed = await invoke<boolean>('check_ollama_installed');
        if (!installed) {
            throw redirect(307, '/noOllama');
        }
    }
};
