<script lang="ts">
	import { Check, OctagonAlert, CircleCheckBig, Copy, ChevronRight, LoaderCircle, FerrisWheel, Brain, MoveDownLeft } from "@lucide/svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";

	interface Payload {
		[zone: string]: {
			[zone: string]: string[];
		};
	}

	function waitForEvent<T = unknown>(eventName: string): Promise<T> {
	return new Promise(async (resolve) => {
		const unlisten = await listen<T>(eventName, (e) => {
		unlisten();      // stop listening after first event
		resolve(e.payload);
		});
	});
	}


	let payload: Payload;
	let avgTimes: Record<string, string> = {};
	let payloadFlag = false;
	let currentStep = 0; // 0: initial, 1: generating, 2: showing messages
	let messages: Array<{ zone: string; message: string; sent: boolean }> = [];
	let isGenerating = false;
	let currentMessageIndex = 0;
	let model = "sage"
	let copied = false
    let isJesterModelToggle = false;

	let toggleModel = () => {
		if (isJesterModelToggle) {
			model = "jest"
		} else {
			model = "sage"
		}
		console.log(model)
	}

    let getPersonCount = (zone: Record<string, string[]>) => {
    	return Object.values(zone).reduce((sum, names) => sum + names.length, 0)
	}

	let checkAndUpdatePayload = () => {
		let middleground = sessionStorage.getItem("payload");
		let avgStorage = sessionStorage.getItem("avg");
		if (middleground) {
			payload = JSON.parse(middleground);
			if (avgStorage) {
				avgTimes = JSON.parse(avgStorage);
			}
			payloadFlag = false;
		} else {
			payloadFlag = true;
		}
	};

	function waitForNEvents<T>(
		event: string,
		n: number,
		onEach: (payload: T, index: number) => void
	) {
		return new Promise<void>(async (resolve) => {
			let count = 0;
			const unlisten = await listen<T>(event, (e) => {
				onEach(e.payload, count);
				count++;

				if (count >= n) {
					unlisten();
					resolve();
				}
			});
		});
	}


	async function generateMessages() {
		if (isGenerating) return
		isGenerating = true
		currentStep = 1
		let zones = Object.keys(payload)

		if (payload && typeof payload === 'object') {
			messages = []
			let prompts = []

			for (const zone of zones) {
                const uncontactedZoneNumber = getPersonCount(payload[zone])
				let waitTimeInfo = "";
				
				// Ensure case insensitivity or format matching if needed. 
				// Assuming lowercased zone strings or exact map keys. We'll search case-insensitively.
				const zoneKeyMatch = Object.keys(avgTimes).find(k => k.toLowerCase() === zone.toLowerCase());
				if (zoneKeyMatch) {
				    waitTimeInfo = ` The current average referral contact time is ${avgTimes[zoneKeyMatch]}.`;
				}
				
				prompts.push(`The ${zone} zone has ${uncontactedZoneNumber} referrals to contact.${waitTimeInfo} Generate a message to get them going!`)
			}

			invoke('generate', {prompts, mood: model})

			await waitForNEvents<string>("completion", zones.length, (message, i) => {
				messages.push({
					zone: zones[i],
					message,
					sent: false
				})
			})
			
			let count = 0
            for (const zone of zones) {
				let msgInProgress = ""

				msgInProgress += messages[count].message
				msgInProgress += "\n"
				const areas = payload[zone];
				
				for (const area in areas) {
					msgInProgress += "\n"
					msgInProgress += "- " + area.trim() + "\n"
					const names = areas[area];
					for (const name of names) {
						msgInProgress += "	* " + name.trim() + "\n"
					}
				}

				messages[count].message = msgInProgress
				count++
			}
		} else {
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
		copied = true
	}

	function markAsSent(index: number) {
		messages[index].sent = true;
		if (index < messages.length - 1) {
			currentMessageIndex = index + 1;
		}
		copied = false
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
		<h2 class="text-2xl font-bold text-[#005175] dark:text-[#339acc]">Construct Messages</h2>
		<p class="text-sm text-gray-500 dark:text-gray-400">
			Summon Charles and build your messages to the zones.
		</p>
	</header>

	{#if payloadFlag}
		<div class="bg-red-50 dark:bg-red-900/30 text-red-800 dark:text-red-200 border border-red-200 dark:border-red-800 p-4 rounded-md flex items-center space-x-2">
			<OctagonAlert size={24} />
			<p>No payload detected, fetch data again or restart the app</p>
		</div>
		<div class="flex w-full justify-center">
			<button on:click={checkAndUpdatePayload} class="w-1/2 bg-[#005175] hover:bg-[#003d58] text-white px-4 py-2 rounded-md font-semibold transition-colors shadow-sm">
				Try again
			</button>
		</div>

	{:else if currentStep === 0}
		<!-- Initial State: Ready to Generate -->
		<div class="bg-green-50 dark:bg-green-900/30 text-green-800 dark:text-green-200 border border-green-200 dark:border-green-800 p-4 rounded-md flex items-center space-x-2">
			<CircleCheckBig size={24} />
			<p>Payload loaded and ready to generate messages!</p>
		</div>
		<div class="space-y-4">
			<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
				<h3 class="font-semibold mb-2 text-gray-900 dark:text-gray-100">What will happen:</h3>
				<ol class="list-decimal list-inside space-y-1 text-sm text-gray-600 dark:text-gray-400">
					<li>Charles will generate AI-assisted messages for each zone</li>
					<li>You'll be shown one message at a time</li>
					<li>Copy and paste each message into the corresponding zone chat</li>
					<li>Mark each message as sent to proceed to the next one</li>
				</ol>
			</div>
			
			<!-- AI Model Selection -->
			<div class="bg-white dark:bg-gray-800 rounded-md border border-gray-300 dark:border-gray-700 p-6 overflow-hidden">
				<h3 class="font-semibold mb-4 text-center text-gray-900 dark:text-gray-100">Select AI Model</h3>
				<div class="flex items-center justify-center gap-4">
					<div class="flex items-center gap-3">
						<div class="flex flex-col items-center gap-2 text-center min-w-[100px]">
							<Brain size={32} class="text-[#005175] dark:text-[#339acc]" />
							<span class="text-sm font-medium text-gray-900 dark:text-gray-100">Sage</span>
							<span class="text-xs text-gray-500 dark:text-gray-400">Meditative & Wise</span>
						</div>
						
						<label class="relative inline-flex items-center cursor-pointer">
							<input type="checkbox" class="sr-only peer" bind:checked={isJesterModelToggle} on:change={toggleModel}>
							<div class="w-11 h-6 bg-gray-200 peer-focus:outline-none dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-[#005175] dark:peer-checked:bg-[#339acc]"></div>
						</label>
						
						<div class="flex flex-col items-center gap-2 text-center min-w-[100px]">
							<FerrisWheel size={32} class="text-orange-500 dark:text-orange-400" />
							<span class="text-sm font-medium text-gray-900 dark:text-gray-100">Jester</span>
							<span class="text-xs text-gray-500 dark:text-gray-400">Funny & Clever</span>
						</div>
					</div>
				</div>
			</div>
			
			<div class="flex w-full justify-center pt-2">
				<button on:click={generateMessages} class="w-full sm:w-1/2 bg-[#005175] hover:bg-[#003d58] text-white px-4 py-2 rounded-md font-semibold transition-colors shadow-sm disabled:opacity-50 disabled:cursor-not-allowed" disabled={isGenerating}>
					Generate Messages
				</button>
			</div>
		</div>
	{:else if currentStep === 1}
		<!-- Generating State -->
		<div class="flex flex-col items-center justify-center space-y-4 py-12">
			<LoaderCircle size={48} class="animate-spin text-[#005175] dark:text-[#339acc]" />
			<p class="text-lg font-semibold text-gray-900 dark:text-gray-100">Generating your messages...</p>
			<p class="text-sm text-gray-500 dark:text-gray-400">Charles is crafting the perfect messages</p>
		</div>
	{:else if currentStep === 2}
		<!-- Message Display State -->
		<div class="space-y-4">
			<!-- Progress Indicator -->
			<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-semibold text-gray-900 dark:text-gray-100">Progress</span>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{messages.filter((m) => m.sent).length} / {messages.length} sent
					</span>
				</div>
				<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
					<div class="bg-[#005175] dark:bg-[#339acc] h-2 rounded-full transition-all duration-300" style="width: {(messages.filter((m) => m.sent).length / messages.length) * 100}%"></div>
				</div>
			</div>

			<!-- Current Message Card -->
			{#each messages as msg, index}
				{#if index === currentMessageIndex}
					<div class="bg-white dark:bg-gray-800 rounded-md border-2 border-[#005175] dark:border-[#339acc] shadow-md p-6">
						<div class="flex items-center justify-between mb-4">
							<div>
								<h3 class="text-lg font-bold text-gray-900 dark:text-gray-100">Zone: {msg.zone}</h3>
								<p class="text-sm text-gray-500 dark:text-gray-400">Message {index + 1} of {messages.length}</p>
							</div>
							{#if msg.sent}
								<CircleCheckBig size={24} class="text-green-600 dark:text-green-500" />
							{/if}
						</div>

						<div class="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 p-4 rounded-md mb-4 text-gray-800 dark:text-gray-200">
							<p class="text-sm font-mono whitespace-pre-wrap">{msg.message}</p>
						</div>

						<div class="flex gap-2">
							{#if !copied}
							<button
								on:click={() => copyMessage(index)}
								class="flex-1 bg-white hover:bg-gray-50 text-[#005175] dark:text-[#339acc] border-2 border-[#005175] dark:border-[#339acc] px-4 py-2 rounded-md font-semibold transition-colors flex items-center justify-center"
								disabled={msg.sent}
							>
								<Copy size={16} class="mr-2" />
								Copy Message
							</button>
							{:else}
							<button
								class="flex-1 bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 border-2 border-gray-300 dark:border-gray-600 px-4 py-2 rounded-md font-semibold flex items-center justify-center cursor-not-allowed"
								disabled
							>
								<Check size={16} class="mr-2" />
								Copied!
							</button>
							{/if}
							<button
								on:click={() => markAsSent(index)}
								class="flex-1 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-md font-semibold transition-colors flex items-center justify-center shadow-sm"
								disabled={msg.sent}
							>
								<CircleCheckBig size={16} class="mr-2" />
								Mark as Sent
							</button>
						</div>

						{#if !msg.sent}
							<p class="text-xs text-gray-500 dark:text-gray-400 mt-3 text-center">
								Copy this message and paste it into the <strong>{msg.zone}</strong> chat, then mark
								it as sent
							</p>
						{/if}
					</div>
				{/if}
			{/each}

			<!-- Completed Messages (Collapsed) -->
			{#if messages.filter((m) => m.sent).length > 0}
				<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
					<h4 class="font-semibold mb-2 text-sm text-gray-900 dark:text-gray-100">Completed Messages</h4>
					<div class="space-y-1">
						{#each messages as msg, index}
							{#if msg.sent && index !== currentMessageIndex}
								<div class="flex items-center justify-between text-sm py-1">
									<span class="text-gray-600 dark:text-gray-400">{msg.zone}</span>
									<CircleCheckBig size={16} class="text-green-600 dark:text-green-500" />
								</div>
							{/if}
						{/each}
					</div>
				</div>
			{/if}

			<!-- All Complete -->
			{#if messages.every((m) => m.sent) && messages.length > 0}
				<div class="bg-green-50 dark:bg-green-900/30 text-green-800 dark:text-green-200 border border-green-200 dark:border-green-800 p-6 rounded-md text-center">
					<CircleCheckBig size={48} class="mx-auto mb-3 text-green-600 dark:text-green-500" />
					<h3 class="text-xl font-bold mb-2">All Messages Sent!</h3>
					<p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
						You've successfully sent all {messages.length} messages to their zones.
					</p>
					<button on:click={resetFlow} class="bg-[#005175] hover:bg-[#003d58] text-white px-4 py-2 rounded-md font-semibold transition-colors shadow-sm">
						Generate New Messages
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>