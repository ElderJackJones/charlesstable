<script lang="ts">
	import { goto } from '$app/navigation';
    import { invoke } from '@tauri-apps/api/core';
	import { Download, RefreshCw } from '@lucide/svelte';

	async function recheck() {
		const installed = await invoke<boolean>('check_ollama_installed');

		if (installed) {
			goto('/'); // redirect back to main app
		} else {
			alert('Ollama is still not installed. Please install it first.');
		}
	}

    async function openDownloadPage() {
        await invoke<boolean>('open_in_chrome', { url: 'https://ollama.com/download' });
    }
</script>

<!-- flex wrapper that fills available space and centers the card -->
<div class="flex flex-col items-center justify-center w-full h-full">
	<div class="card bg-surface-900 w-full max-w-md p-6 rounded-2xl shadow-lg">
		<div class="space-y-4 text-center">
			<h2 class="text-2xl font-bold">Ollama Not Installed</h2>
			<p class="text-sm opacity-80">
				Charles requires <span class="font-semibold">Ollama</span> to run local AI models.
				Please install it before continuing.
			</p>

			<div class="space-y-2">
				<button
                    on:click={openDownloadPage}
					color="primary"
					class="w-full flex items-center justify-center gap-2 btn variant-filled"
				>
					<Download class="w-4 h-4" /> Download Ollama
				</button>

				<button
					on:click={recheck}
					class="w-full flex items-center justify-center gap-2 btn variant-outlined"
				>
					<RefreshCw class="w-4 h-4" /> Recheck Installation
				</button>
			</div>

			<p class="text-xs opacity-70">
				After installing, restart Charles or click “Recheck Installation”.
			</p>
		</div>
	</div>
</div>
