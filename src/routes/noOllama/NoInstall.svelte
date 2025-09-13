<script lang="ts">
	import { Download, RefreshCw } from "@lucide/svelte";

    let { openDownloadPage, recheck } = $props();
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
					onclick={openDownloadPage}
					color="primary"
					class="w-full flex items-center justify-center gap-2 btn variant-filled"
				>
					<Download class="w-4 h-4" /> Download Ollama
				</button>

				<button
					onclick={recheck}
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
