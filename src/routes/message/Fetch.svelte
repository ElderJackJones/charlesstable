<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import { listen } from '@tauri-apps/api/event';

	let port : Number
	let buttonName = "Start Bridge"
	let bridgeActive = false
	let payloadSaved = false

	let activateBridge = async (authToken: string) => {
		port = await invoke("start_server")
		buttonName = "Bridge Open"
		bridgeActive = true
		console.log(port)
	}

	let handlePayload = (payload: Payload) => {
		sessionStorage.setItem("payload", JSON.stringify(payload.map))
		payloadSaved = true
	}

	let requestPeople = async (data: String) => {
		await invoke("get_people", {userobj : data})
	}

	interface Payload {
		map: Record<string, Record<string, string[]>>;
	}

	onMount(() => {
		let cleanup: (() => void) | undefined;

		(async () => {
			cleanup = await listen<Payload>('payload', (e) => {
				handlePayload(e.payload)
			})
		})()

		return () => {
			if (cleanup) cleanup();
		}
	})


    let authToken = "";


	async function handleOpen(url: string, e: MouseEvent) {
		e.preventDefault();
		await open(url);
	}


</script>

<div class="flex flex-col w-full space-y-8 p-2 sm:p-4 md:p-6 text-left">
	<!-- Header -->
	<header class="space-y-1">
		<h2 class="text-2xl font-bold text-primary">Fetch Data</h2>
		<p class="text-sm text-surface-500">
			Install the Chrome extension extract online data.
		</p>
	</header>

	<!-- Instructions -->
	<section>
		<ol class="list-decimal list-inside space-y-2 text-sm leading-relaxed">
			<li>
				Add <a href="https://chromewebstore.google.com/detail/charles-connect/ebmkaffdfclekgoaclphjidbefhnidki"
				on:click={(e) => {
					handleOpen(
						"https://chromewebstore.google.com/detail/charles-connect/ebmkaffdfclekgoaclphjidbefhnidki",
						e
					)
				}}
				rel="noopener noreferrer"
				class="text-primary underline hover:text-primary-500"
				
				>Charles Connect</a> to chrome
			</li>
			<li>
				Open
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
				> and log in.
			</li>
			<li>
				Click the button below to start the bridge.
			</li>
			<li>On the Referral Manager page, open the Charles Connect extension and follow its instructions.</li>
		</ol>
	</section>

	<div class="grid w-full justify-items-center">
		{#if !payloadSaved}
      	<button class={`w-64 btn rounded-full ${bridgeActive ? "btn preset-filled-success-500" : "preset-filled-tertiary-500"}`} on:click={() => activateBridge(authToken)} disabled={bridgeActive}>{buttonName}</button>
		{:else}
		<div class="w-full card bg-surface-400 grid justify-items-center">
			<p class="my-4">
				Data received!
			</p>
		</div>
		{/if}
	</div>

</div>
