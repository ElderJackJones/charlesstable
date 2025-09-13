<script lang="ts">
	import { goto } from '$app/navigation';
    import { invoke } from '@tauri-apps/api/core';
	import NoInstall from './NoInstall.svelte';
	import LlamaSummon from './LlamaSummon.svelte';

    let downloadStep = true;

	async function recheck() {
		const installed = await invoke<boolean>('check_ollama_installed');

		if (installed) {
			downloadStep = false // redirect back to main app
		} else {
			alert('Ollama is still not installed. Please install it first.');
		}
	}

    async function openDownloadPage() {
        await invoke<boolean>('open_in_chrome', { url: 'https://ollama.com/download' });
    }


</script>

{#if downloadStep}
	<NoInstall {openDownloadPage} {recheck} />
{:else}
	<LlamaSummon />
{/if}
