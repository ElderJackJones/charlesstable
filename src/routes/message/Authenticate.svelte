<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import { Check, ClipboardCopy, Save } from "@lucide/svelte";
	import { listen } from '@tauri-apps/api/event';

	let port : Number
	let buttonName = "ugly"

	let activateBridge = async (authToken: string) => {
		port = await invoke("start_bridge")
		buttonName = "bridge started"
		console.log(port)
	}

	let requestPeople = async (data: String) => {
		await invoke("get_people", {userobj : data})
	}

	onMount(async () => {
		const unlisten = await listen<string>('user_auth', async (event) => {
		console.log("This is the event payload:  " + event.payload)
		await requestPeople(event.payload)

		const otherlisten = await listen<string>('people_list', (event) => {
			console.log(event.payload)
		})
	})
	})

	let downloaded = false;
	let renderButton = true;
	let clipboardText = "";
    let authToken = "";
	let copied = false;
	let tokenSaved = false;

	async function handleOpen(url: string, e: MouseEvent) {
		e.preventDefault();
		await open(url);
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
		copied = true;
		setTimeout(() => (copied = false), 3000);
	}

	async function saveAuthToken() {
		// 🔧 TODO: Add logic to handle token saving (e.g., invoke or localStorage)
        localStorage.setItem('auth', authToken)
		tokenSaved = true;
	}
</script>

<!-- ========================== -->
<!-- Authenticate Card Content -->
<!-- ========================== -->
<div class="flex flex-col w-full space-y-8 p-2 sm:p-4 md:p-6 text-left">
	<!-- Header -->
	<header class="space-y-1">
		<h2 class="text-2xl font-bold text-primary">Authenticate</h2>
		<p class="text-sm text-surface-500">
			Install the Chrome extension and link it to your account.
		</p>
	</header>

	<!-- Download Section -->
	<section>
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
			<p class="text-success font-medium mt-2">✅ Extension already downloaded</p>
		{/if}
	</section>

	<!-- Instructions -->
	<section>
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
					{:else}
						<ClipboardCopy size={12} />
					{/if}
				</button>
			</li>
			<li>
				Click <strong>“Load unpacked”</strong>, open your
				<code>Downloads</code> folder, and select
				<strong>CharlesExtension</strong>.
			</li>
			<li>
				Log in at
				<a
					href="https://referralmanager.churchofjesuschrist.org/dashboard/"
					on:click={(e) =>
						handleOpen(
							"https://referralmanager.churchofjesuschrist.org/dashboard/",
							e
						)}
					rel="noopener noreferrer"
					class="text-primary underline hover:text-primary-500"
					>referralmanager.churchofjesuschrist.org</a
				>.
			</li>
			<li>
				Open your Chrome extensions and click on the
				<strong>Charles Extension</strong>.
			</li>
			<li>Paste your authentication token below and save it.</li>
		</ol>
	</section>

	<!-- Token Entry -->
	<section
	class="relative max-h-[70vh] pr-1"
>
	<div class="relative max-h-[55vh]">
		<textarea
			id="userobj"
			name="userobj"
			bind:value={authToken}
			class="textarea w-full h-full max-h-[55vh] bg-surface-950 border border-surface-700 rounded-xl p-4 focus:outline-none focus:ring-2 focus:ring-primary resize-y overflow-auto"
			placeholder="Paste your authentication token here..."
		></textarea>

		<button
			on:click={saveAuthToken}
			class="absolute bottom-3 right-3 rounded-full bg-primary text-white p-2 shadow-md transition-all duration-200 hover:scale-105 hover:shadow-lg disabled:opacity-50"
			aria-label="Save authentication token"
			disabled={!authToken.trim()}
		>
			{#if tokenSaved}
				<Check size={18} class="transition-opacity duration-300" />
			{:else}
				<Save size={18} class="transition-opacity duration-300" />
			{/if}
		</button>
	</div>
</section>
      <button class="btn" on:click={() => activateBridge(authToken)}>{buttonName}</button>

</div>
