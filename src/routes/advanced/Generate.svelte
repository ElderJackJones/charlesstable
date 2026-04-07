<script lang="ts">
	import { Check, OctagonAlert, CircleCheckBig, Copy, ChevronRight, LoaderCircle } from "@lucide/svelte";
	import { onMount } from "svelte";

	interface Payload {
		[zone: string]: {
			[zone: string]: string[];
		};
	}

	interface PromptData {
		zone: string;
		prompt: string;
		userResponse: string;
		sent: boolean;
	}

	let payload: Payload;
	let avgTimes: Record<string, string> = {};
	let payloadFlag = false;
	let currentStep = 0; // 0: initial, 1: input prompts, 2: showing messages
	let prompts: PromptData[] = [];
	let messages: Array<{ zone: string; message: string; sent: boolean }> = [];
	let currentMessageIndex = 0;
	let copied = false;
	let currentPromptIndex = 0;
    let promptCopy = false

	let getPersonCount = (zone: Record<string, string[]>) => {
		return Object.values(zone).reduce((sum, names) => sum + names.length, 0);
	};

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

	function generatePrompts() {
		if (!payload || typeof payload !== "object") {
			messages = [];
			payloadFlag = true;
			currentStep = 0;
			return;
		}

		const zones = Object.keys(payload);
		prompts = [];

		for (const zone of zones) {
			const uncontactedZoneNumber = getPersonCount(payload[zone]);
			let waitTimeInfo = "";
			
			const zoneKeyMatch = Object.keys(avgTimes).find(k => k.toLowerCase() === zone.toLowerCase());
			if (zoneKeyMatch) {
				waitTimeInfo = ` The current average referral contact time is ${avgTimes[zoneKeyMatch]}.`;
			}

			const prompt = `The ${zone} zone has ${uncontactedZoneNumber} referrals to contact.${waitTimeInfo} Generate a message to get them going!`;
			prompts.push({
				zone,
				prompt,
				userResponse: "",
				sent: false
			});
		}

		currentStep = 1;
		currentPromptIndex = 0;
	}

	function copyPrompt(index: number) {
		navigator.clipboard.writeText(prompts[index].prompt);
		promptCopy = true;
	}

	function moveToNextPrompt() {
		if (currentPromptIndex < prompts.length - 1) {
			currentPromptIndex++;
            promptCopy = false
		}
	}

	function finishPrompts() {
		// Build messages from the prompts and responses
		messages = [];

		for (const promptData of prompts) {
			messages.push({
				zone: promptData.zone,
				message: promptData.userResponse.trim(),
				sent: false
			});
		}

		currentStep = 2;
		currentMessageIndex = 0;
	}

	function copyMessage(index: number) {
		navigator.clipboard.writeText(messages[index].message);
		copied = true;
	}

	function markAsSent(index: number) {
		messages[index].sent = true;
		if (index < messages.length - 1) {
			currentMessageIndex = index + 1;
		}
		copied = false;
	}

	function resetFlow() {
		currentStep = 0;
		messages = [];
		prompts = [];
		currentMessageIndex = 0;
		currentPromptIndex = 0;
	}

	function allPromptsAnswered() {
		return prompts.every((p) => p.userResponse.trim().length > 0);
	}

	onMount(checkAndUpdatePayload);
</script>

<div class="flex flex-col w-full space-y-8 p-2 sm:p-4 md:p-6 text-left">
	<!-- Header -->
	<header class="space-y-1">
		<h2 class="text-2xl font-bold text-[#005175] dark:text-[#339acc]">Construct Messages</h2>
		<p class="text-sm text-gray-500 dark:text-gray-400">
			Generate prompts and input AI responses to build your messages to the zones.
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
			<p>Payload loaded and ready to generate prompts!</p>
		</div>
		<div class="space-y-4">
			<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
				<h3 class="font-semibold mb-2 text-gray-900 dark:text-gray-100">What will happen:</h3>
				<ol class="list-decimal list-inside space-y-1 text-sm text-gray-600 dark:text-gray-400">
					<li>You'll see prompts for each zone</li>
					<li>Copy each prompt to your favorite LLM (ChatGPT, Claude, etc.)</li>
					<li>Paste the AI-generated response back here</li>
					<li>Review the final formatted messages and send them</li>
				</ol>
			</div>
			
			<div class="flex w-full justify-center pt-2">
				<button on:click={generatePrompts} class="w-full sm:w-1/2 bg-[#005175] hover:bg-[#003d58] text-white px-4 py-2 rounded-md font-semibold transition-colors shadow-sm">
					Generate Prompts
				</button>
			</div>
		</div>

	{:else if currentStep === 1}
		<!-- Input Prompts State -->
		<div class="space-y-4">
			<!-- Progress Indicator -->
			<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-semibold text-gray-900 dark:text-gray-100">Progress</span>
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{prompts.filter((p) => p.userResponse.trim().length > 0).length} / {prompts.length} answered
					</span>
				</div>
				<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
					<div class="bg-[#005175] dark:bg-[#339acc] h-2 rounded-full transition-all duration-300" style="width: {(prompts.filter((p) => p.userResponse.trim().length > 0).length / prompts.length) * 100}%"></div>
				</div>
			</div>

			<!-- Current Prompt Card -->
			{#each prompts as prompt, index}
				{#if index === currentPromptIndex}
					<div class="bg-white dark:bg-gray-800 rounded-md border-2 border-[#005175] dark:border-[#339acc] shadow-md p-6">
						<div class="mb-4">
							<h3 class="text-lg font-bold text-gray-900 dark:text-gray-100">Zone: {prompt.zone}</h3>
							<p class="text-sm text-gray-500 dark:text-gray-400">Prompt {index + 1} of {prompts.length}</p>
						</div>

						<div class="space-y-4">
							<!-- Prompt Display -->
							<div>
								<p class="block text-sm font-semibold mb-2 text-gray-900 dark:text-gray-100">Prompt to use:</p>
								<div class="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 p-4 rounded-lg flex items-start justify-between gap-4">
									<p class="text-sm font-mono whitespace-pre-wrap flex-1 text-gray-800 dark:text-gray-200">{prompt.prompt}</p>
                                    {#if !promptCopy}
									<button
										on:click={() => copyPrompt(index)}
										class="bg-[#005175] hover:bg-[#003d58] text-white px-3 py-2 rounded-md font-semibold transition-colors flex-shrink-0"
										title="Copy prompt"
									>
										<Copy size={16} />
									</button>
                                    {:else}
                                    <button
										class="bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 border border-gray-300 dark:border-gray-600 px-3 py-2 rounded-md font-semibold flex-shrink-0 cursor-not-allowed"
										title="Copy prompt"
                                        disabled
									>
										<Check size={16} />
									</button>
                                    {/if}
								</div>
							</div>

							<!-- Response Input -->
							<div>
								<label for="response-{index}" class="block text-sm font-semibold mb-2 text-gray-900 dark:text-gray-100">
									Paste the AI-generated response here:
								</label>
								<textarea
									id="response-{index}"
									bind:value={prompt.userResponse}
									class="w-full text-sm rounded-md border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 shadow-sm focus:border-[#005175] focus:ring focus:ring-[#005175] focus:ring-opacity-50 p-3"
									placeholder="Paste the AI response here..."
									rows="6"
								></textarea>
							</div>

							<!-- Navigation Buttons -->
							<div class="flex gap-2">
								<button
									on:click={moveToNextPrompt}
									class="flex-1 bg-white hover:bg-gray-50 text-[#005175] dark:text-[#339acc] border-2 border-[#005175] dark:border-[#339acc] px-4 py-2 rounded-md font-semibold transition-colors flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
									disabled={currentPromptIndex >= prompts.length - 1 || prompt.userResponse.trim().length === 0}
								>
									Next Prompt
									<ChevronRight size={16} class="ml-2" />
								</button>
								<button
									on:click={finishPrompts}
									class="flex-1 bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-md font-semibold transition-colors flex items-center justify-center shadow-sm disabled:opacity-50 disabled:cursor-not-allowed"
									disabled={!allPromptsAnswered()}
								>
									<Check size={16} class="mr-2" />
									All Done
								</button>
							</div>

							<p class="text-xs text-gray-500 dark:text-gray-400 text-center">
								Fill in all prompts before you can continue
							</p>
						</div>
					</div>
				{/if}
			{/each}

			<!-- Completed Prompts (Collapsed) -->
			{#if prompts.filter((p) => p.userResponse.trim().length > 0).length > 0}
				<div class="bg-gray-50 dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700 p-4">
					<h4 class="font-semibold mb-2 text-sm text-gray-900 dark:text-gray-100">Completed Zones</h4>
					<div class="space-y-1">
						{#each prompts as prompt, index}
							{#if prompt.userResponse.trim().length > 0 && index !== currentPromptIndex}
								<div class="flex items-center justify-between text-sm py-1">
									<span class="text-gray-600 dark:text-gray-400">{prompt.zone}</span>
									<CircleCheckBig size={16} class="text-green-600 dark:text-green-500" />
								</div>
							{/if}
						{/each}
					</div>
				</div>
			{/if}
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