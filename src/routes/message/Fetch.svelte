<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { open } from "@tauri-apps/plugin-shell";
	import { listen } from '@tauri-apps/api/event';

	let port : Number
	let buttonName = "Start Bridge"
	let bridgeActive = false
	let payloadSaved = false
	let avgSaved = false

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

	let handleAvg = (avgMap: Record<string, string>) => {
		sessionStorage.setItem("avg", JSON.stringify(avgMap))
		avgSaved = true
	}

	let requestPeople = async (data: String) => {
		await invoke("get_people", {userobj : data})
	}

	interface Payload {
		map: Record<string, Record<string, string[]>>;
	}

	onMount(() => {
		let cleanupPayload: (() => void) | undefined;
		let cleanupAvg: (() => void) | undefined;

		(async () => {
			cleanupPayload = await listen<Payload>('payload', (e) => {
				handlePayload(e.payload)
			})
			cleanupAvg = await listen<Record<string, string>>('avg', (e) => {
				handleAvg(e.payload)
			})
		})()

		return () => {
			if (cleanupPayload) cleanupPayload();
			if (cleanupAvg) cleanupAvg();
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
			<li>
				On the Referral Manager page, open the Charles Connect extension.
				<div class="mt-2 p-3 bg-warning-50-950 text-warning-900-50 rounded-md border-l-4 border-warning-500">
					<strong>Crucial Order:</strong> If you are sending Average Contact Times, you <em>must</em> send them <strong>before</strong> sending the main Payload. Sending the Payload automatically closes the connection bridge!
				</div>
			</li>
		</ol>
	</section>

	<div class="space-y-4">
		<!-- Status Indicators -->
		{#if bridgeActive && !payloadSaved}
			<div class="flex flex-col sm:flex-row gap-4 justify-center items-center p-4 bg-surface-100-900 rounded-lg">
				<div class="flex items-center gap-2">
					<div class={`w-3 h-3 rounded-full ${avgSaved ? "bg-success-500" : "bg-surface-400 animate-pulse"}`}></div>
					<span class="text-sm font-medium">{avgSaved ? "Average Times Received" : "Waiting for Avg Times..."}</span>
				</div>
				<div class="hidden sm:block border-l border-surface-300-600 h-6"></div>
				<div class="flex items-center gap-2">
					<div class={`w-3 h-3 rounded-full ${payloadSaved ? "bg-success-500" : "bg-surface-400 animate-pulse"}`}></div>
					<span class="text-sm font-medium">{payloadSaved ? "Payload Received" : "Waiting for Payload..."}</span>
				</div>
			</div>
		{/if}

		<div class="grid w-full justify-items-center">
			{#if !payloadSaved}
			  <button class={`w-64 btn rounded-full ${bridgeActive ? "preset-filled-success-500" : "preset-filled-tertiary-500"}`} on:click={() => activateBridge(authToken)} disabled={bridgeActive}>{buttonName}</button>
			{:else}
			<div class="w-full card preset-tonal-success flex items-center justify-center p-4 gap-2">
				<p class="font-bold">
					Data received successfully!
				</p>
			</div>
			{/if}
		</div>
	</div>

</div>
