<script lang="ts">
	import { OctagonAlert, CircleCheckBig, Copy, ChevronRight, LoaderCircle, FerrisWheel, Brain } from "@lucide/svelte";
	import { Switch } from "@skeletonlabs/skeleton-svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";

	interface Payload {
		[zone: string]: {
			[zone: string]: string[];
		};
	}

	let payload: Payload;
	let payloadFlag = false;
	let currentStep = 0; // 0: initial, 1: generating, 2: showing messages
	let messages: Array<{ zone: string; message: string; sent: boolean }> = [];
	let isGenerating = false;
	let currentMessageIndex = 0;
	let model = "charlesSage:latest"

	let toggleModel = () => {
		if (model === "charlesJest:latest") {
			model = "charlesSage:latest"
		} else {
			model = "charlesJest:latest"
		}
		console.log(model)
	}

    let getPersonCount = (zone: Record<string, string[]>) => {
    	return Object.values(zone).reduce((sum, names) => sum + names.length, 0)
	}

	let checkAndUpdatePayload = () => {
		let middleground = sessionStorage.getItem("payload");
		if (middleground) {
			payload = JSON.parse(middleground);
			payloadFlag = false;
		} else {
			payloadFlag = true;
		}
	};

	async function generateMessages() {
		isGenerating = true
		currentStep = 1

		if (payload && typeof payload === 'object') {
			messages = []

            for (const zone in payload) {
                const uncontactedZoneNumber = getPersonCount(payload[zone])
				let msgInProgress = ""
				
				const AIMessage = await new Promise(async (resolve) => {
					const unlisten = await listen("response", (e) => {
						unlisten() // Clean up immediately after receiving
						resolve(e.payload)
					})
					
					invoke('generate', {
						prompt: `The ${zone} zone has ${uncontactedZoneNumber} referrals to contact. Generate a message to get them going!`, 
						model: model
					})
				})

				msgInProgress += AIMessage
				msgInProgress += "\n"
				const areas = payload[zone];
				
				for (const area in areas) {
					msgInProgress += "- " + area.trim() + "\n"
					const names = areas[area];
					for (const name of names) {
						msgInProgress += "	* " + name.trim() + "\n"
					}
				}

				messages.push({zone: zone, message: msgInProgress, sent: false})
			}

			
		} else {
			// Fallback if payload structure is unexpected
			messages = [];
			payloadFlag = true;
			currentStep = 0;
			return;
		}

		isGenerating = false;
		currentStep = 2;
	}

	function copyMessage(index: number) {
		navigator.clipboard.writeText(messages[index].message);
		// Optional: Add a toast notification here
	}

	function markAsSent(index: number) {
		messages[index].sent = true;
		if (index < messages.length - 1) {
			currentMessageIndex = index + 1;
		}
	}

	function resetFlow() {
		currentStep = 0;
		messages = [];
		currentMessageIndex = 0;
	}

	onMount(checkAndUpdatePayload);
</script>

<div class="flex flex-col w-full space-y-8 p-2 sm:p-4 md:p-6 text-left">
	<!-- Header -->
	<header class="space-y-1">
		<h2 class="text-2xl font-bold text-primary">Construct Messages</h2>
		<p class="text-sm text-surface-500">
			Summon Charles and build your messages to the zones.
		</p>
	</header>

	{#if payloadFlag}
		<div class="card p-4 preset-tonal-error flex items-center space-x-2">
			<OctagonAlert size={24} />
			<p>No payload detected, fetch data again or restart the app</p>
		</div>
		<div class="flex w-full justify-center">
			<button on:click={checkAndUpdatePayload} class="btn preset-filled w-1/2">
				Try again
			</button>
		</div>

	{:else if currentStep === 0}
		<!-- Initial State: Ready to Generate -->
		<div class="card p-6 preset-tonal-success flex items-center space-x-2">
			<CircleCheckBig size={24} />
			<p>Payload loaded and ready to generate messages!</p>
		</div>
		<div class="space-y-4">
			<div class="card p-4 bg-surface-200-800">
				<h3 class="font-semibold mb-2">What will happen:</h3>
				<ol class="list-decimal list-inside space-y-1 text-sm text-surface-600-400">
					<li>Charles will generate AI-assisted messages for each zone</li>
					<li>You'll be shown one message at a time</li>
					<li>Copy and paste each message into the corresponding zone chat</li>
					<li>Mark each message as sent to proceed to the next one</li>
				</ol>
			</div>
			
			<!-- AI Model Selection -->
			<div class="card p-6 bg-surface-100-900 border-2 border-surface-300-700">
				<h3 class="font-semibold mb-4 text-center">Select AI Model</h3>
				<div class="flex items-center justify-center gap-4">
					<div class="flex items-center gap-3">
						<div class="flex flex-col items-center gap-2 text-center min-w-[100px]">
							<Brain size={32} class="text-primary" />
							<span class="text-sm font-medium">Sage</span>
							<span class="text-xs text-surface-500">Meditative & Wise</span>
						</div>
						
						<Switch onCheckedChange={toggleModel}>
							<Switch.Control>
								<Switch.Thumb  />
							</Switch.Control>
							<Switch.HiddenInput />
						</Switch>
						
						<div class="flex flex-col items-center gap-2 text-center min-w-[100px]">
							<FerrisWheel size={32} class="text-secondary" />
							<span class="text-sm font-medium">Jester</span>
							<span class="text-xs text-surface-500">Funny & Clever</span>
						</div>
					</div>
				</div>
			</div>
			
			<div class="flex w-full justify-center pt-2">
				<button on:click={generateMessages} class="btn preset-filled-primary-500 w-full sm:w-1/2">
					Generate Messages
				</button>
			</div>
		</div>
	{:else if currentStep === 1}
		<!-- Generating State -->
		<div class="flex flex-col items-center justify-center space-y-4 py-12">
			<LoaderCircle size={48} class="animate-spin text-primary" />
			<p class="text-lg font-semibold">Generating your messages...</p>
			<p class="text-sm text-surface-500">Charles is crafting the perfect messages</p>
		</div>
	{:else if currentStep === 2}
		<!-- Message Display State -->
		<div class="space-y-4">
			<!-- Progress Indicator -->
			<div class="card p-4 bg-surface-200-800">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-semibold">Progress</span>
					<span class="text-sm text-surface-600-400">
						{messages.filter((m) => m.sent).length} / {messages.length} sent
					</span>
				</div>
				<div class="w-full bg-surface-300-700 rounded-full h-2">
					<div
						class="bg-primary h-2 rounded-full transition-all duration-300"
						style="width: {(messages.filter((m) => m.sent).length / messages.length) * 100}%"
					/>
				</div>
			</div>

			<!-- Current Message Card -->
			{#each messages as msg, index}
				{#if index === currentMessageIndex}
					<div class="card p-6 preset-tonal-primary border-2 border-primary">
						<div class="flex items-center justify-between mb-4">
							<div>
								<h3 class="text-lg font-bold">Zone: {msg.zone}</h3>
								<p class="text-sm text-surface-500">Message {index + 1} of {messages.length}</p>
							</div>
							{#if msg.sent}
								<CircleCheckBig size={24} class="text-success" />
							{/if}
						</div>

						<div class="bg-surface-100-900 p-4 rounded-lg mb-4">
							<p class="text-sm font-mono whitespace-pre-wrap">{msg.message}</p>
						</div>

						<div class="flex gap-2">
							<button
								on:click={() => copyMessage(index)}
								class="btn preset-filled flex-1"
								disabled={msg.sent}
							>
								<Copy size={16} class="mr-2" />
								Copy Message
							</button>
							<button
								on:click={() => markAsSent(index)}
								class="btn preset-filled-success flex-1"
								disabled={msg.sent}
							>
								<CircleCheckBig size={16} class="mr-2" />
								Mark as Sent
							</button>
						</div>

						{#if !msg.sent}
							<p class="text-xs text-surface-500 mt-3 text-center">
								Copy this message and paste it into the <strong>{msg.zone}</strong> chat, then mark
								it as sent
							</p>
						{/if}
					</div>
				{/if}
			{/each}

			<!-- Completed Messages (Collapsed) -->
			{#if messages.filter((m) => m.sent).length > 0}
				<div class="card p-4 bg-surface-200-800">
					<h4 class="font-semibold mb-2 text-sm">Completed Messages</h4>
					<div class="space-y-1">
						{#each messages as msg, index}
							{#if msg.sent && index !== currentMessageIndex}
								<div class="flex items-center justify-between text-sm py-1">
									<span class="text-surface-600-400">{msg.zone}</span>
									<CircleCheckBig size={16} class="text-success" />
								</div>
							{/if}
						{/each}
					</div>
				</div>
			{/if}

			<!-- All Complete -->
			{#if messages.every((m) => m.sent)}
				<div class="card p-6 preset-tonal-success text-center">
					<CircleCheckBig size={48} class="mx-auto mb-3 text-success" />
					<h3 class="text-xl font-bold mb-2">All Messages Sent!</h3>
					<p class="text-sm text-surface-600-400 mb-4">
						You've successfully sent all {messages.length} messages to their zones.
					</p>
					<button on:click={resetFlow} class="btn preset-filled-primary">
						Generate New Messages
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>