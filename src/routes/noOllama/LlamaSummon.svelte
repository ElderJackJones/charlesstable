<script lang="ts">
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import { LoaderCircle, Play } from "@lucide/svelte";
    import { invoke } from "@tauri-apps/api/core";
	import { Progress } from "@skeletonlabs/skeleton-svelte";

	let progress = 0;
	let status = "Ready to install llama3.";
	let isInstalling = false;
	let steps = [
		"Downloading llama3 model...",
		"Plugging in Charles...", 
		"Doing some black magic...",
		"Checking the fridge for food (again)...",
		"Teaching Llama to love Charles...",
	]
	let stepCount = 0;
	let interval : number = 0;

	// Listen for progress updates
	onMount(async () => {
		await listen("install-progress", (event) => {
			progress = event.payload as number;
			status = progress < 100 ? "Downloading llama3 model..." : "Installation complete!";
			if (progress >= 100) {
				isInstalling = false;
				clearInterval(interval);
			}
		});
	});

	async function beginInstall() {
		isInstalling = true;
		status = "Starting install...";
		progress = 0;
		interval = setInterval(() => {
			if (isInstalling && progress < 95) {
				stepCount = (stepCount + 1) % steps.length;
				status = steps[stepCount];
			}
		}, 15000); // change status every 15 seconds
		try {
			await invoke("install_models");
		} catch (err) {
			console.error("Install failed", err);
			status = "Installation failed. Please try again.";
			isInstalling = false;
		}
	}
</script>

<div class="flex flex-col items-center justify-center w-full h-full">
	<div class="card bg-surface-900 w-full max-w-md p-6 rounded-2xl shadow-lg">
		<div class="space-y-4 text-center">
			<h2 class="text-2xl font-bold">Install Charles</h2>
			<p class="text-sm opacity-80">
				Charles requires the <span class="font-semibold">llama3</span> model. Click below to begin the installation.
			</p>

			<!-- show button until install starts -->
			{#if !isInstalling && progress === 0}
				<button
					on:click={beginInstall}
					class="w-full flex items-center justify-center gap-2 btn variant-filled"
				>
					<Play class="w-4 h-4" /> Begin Install
				</button>
			{/if}

			<!-- show progress once installing -->
			{#if isInstalling || progress > 0}
				<div class="space-y-2">
					<Progress value={progress} max={100} />
					<p class="text-sm flex items-center justify-center gap-2">
						{#if isInstalling && progress < 100}
							<LoaderCircle class="w-4 h-4 animate-spin" />
						{/if}
						<span>{status}</span>
						{#if progress > 0}<span class="font-semibold">{progress}%</span>{/if}
					</p>
				</div>
			{/if}

			<p class="text-xs opacity-70">
				This may take a few minutes depending on your internet speed.
			</p>
		</div>
	</div>
</div>
