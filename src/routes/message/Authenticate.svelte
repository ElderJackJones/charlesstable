<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
    import { open } from "@tauri-apps/plugin-shell";
	import { Check, ClipboardCopy } from "@lucide/svelte";

	let downloaded = false;
	let renderButton = true;
	let clipboardText = "";
    let copied = false;

    async function handleOpen(url: string, e: MouseEvent) {
    e.preventDefault(); // stop the browser from following the href
    await open(url);    // let Tauri handle it
  }

	async function handleDownload() {
		try {
			downloaded = true;
			localStorage.setItem("chromeExtensionDownloaded", "true");
			const resp = await invoke("download_extension");
			alert(resp);
			renderButton = false;
		} catch (err) {
			alert("Download failed. Please try again.");
			downloaded = false;
		}
	}

	onMount(() => {
		const extensionExists = localStorage.getItem("chromeExtensionDownloaded");
		if (extensionExists === "true") renderButton = false;
	});

    function copyExtensionsURL() {
    navigator.clipboard.writeText("chrome://extensions");
    copied = true
    setTimeout(() => copied = false, 10000)
  }
</script>

<!--
  This sits inside the step card wrapper.
  Avoid double padding / double cards.
-->
<div class="flex flex-col justify-start items-stretch w-full h-full space-y-6 overflow-y-auto p-2 sm:p-4 md:p-6 text-left">
	<header class="space-y-1">
		<h2 class="text-2xl font-bold text-primary">Authenticate</h2>
		<p class="text-sm text-surface-500">
			Install the Chrome extension and link it to your account.
		</p>
	</header>

	<!-- Download button -->
	<div>
		{#if renderButton}
			<button
				on:click={handleDownload}
				class="btn btn-primary w-full sm:w-auto font-medium rounded-lg transition-all duration-200 disabled:opacity-50"
				disabled={downloaded}
				aria-label="download extension"
			>
				{downloaded ? "Extension Downloaded!" : "Download Chrome Extension"}
			</button>
		{:else}
			<p class="text-success font-medium mt-2">
				✅ Extension already downloaded
			</p>
		{/if}
	</div>

	<!-- Step instructions -->
	<ol class="list-decimal list-inside space-y-2 text-sm leading-relaxed">
		<li>
			Open Chrome and go to 
			<code class="text-primary select-all">chrome://extensions</code>
			<button
				class="btn-icon ml-2 p-1 leading-none aspect-square preset-tonal"
				on:click={copyExtensionsURL}
				aria-label="Copy Chrome extensions URL"
			>
                {#if copied}
                    <Check size={12} />
                {/if}
                {#if !copied}
                <ClipboardCopy size={12} />
                {/if}
			</button>
		</li>
		<li>
			Click <strong>“Load unpacked”</strong>, open your
			<code>Downloads</code> folder, and select the
			<strong>CharlesExtension</strong> folder.
		</li>
		<li>
			Log in at
			<a href="https://referralmanager.churchofjesuschrist.org/dashboard/" on:click={(e) => handleOpen("https://referralmanager.churchofjesuschrist.org/dashboard/", e)} rel="noopener noreferrer">
				referralmanager.churchofjesuschrist.org
			</a>.
		</li>
		<li>
			Open your Chrome extensions and click on the
			<strong>Charles Extension</strong>.
		</li>
		<li>
			Paste the text that was automatically copied to your clipboard below.
		</li>
	</ol>

	<!-- Paste area -->
	<div class="space-y-2">
		<textarea
			id="userobj"
			name="userobj"
			bind:value={clipboardText}
			class="textarea w-full min-h-[120px] bg-surface-950 rounded-lg p-3 transition-all resize-y"
			placeholder="Paste your authentication token here..."
		></textarea>
	</div>
</div>
