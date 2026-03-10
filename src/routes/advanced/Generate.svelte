<script lang="ts">
	import { Check, OctagonAlert, CircleCheckBig, Copy, ChevronRight } from "@lucide/svelte";
	import { Progress } from "@skeletonlabs/skeleton-svelte";
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
	let payloadFlag = false;
	let currentStep = 0; // 0: initial, 1: input prompts, 2: showing messages
	let prompts: PromptData[] = [];
	let messages: Array<{ zone: string; message: string; sent: boolean }> = [];
	let currentMessageIndex = 0;
	let copied = false;
	let currentPromptIndex = 0;

	let getPersonCount = (zone: Record<string, string[]>) => {
		return Object.values(zone).reduce((sum, names) => sum + names.length, 0);
	};

	let checkAndUpdatePayload = () => {
		let middleground = sessionStorage.getItem("payload");
		if (middleground) {
			payload = JSON.parse(middleground);
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
			const prompt = `The ${zone} zone has ${uncontactedZoneNumber} referrals to contact. Generate a message to get them going!`;
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
		copied = true;
	}

	function moveToNextPrompt() {
		if (currentPromptIndex < prompts.length - 1) {
			currentPromptIndex++;
		}
	}

	function finishPrompts() {
		// Build messages from the prompts and responses
		messages = [];
		let count = 0;

		for (const promptData of prompts) {
			const zone = promptData.zone;
			let msgInProgress = promptData.userResponse;
			msgInProgress += "\n";
			const areas = payload[zone];

			for (const area in areas) {
				msgInProgress += "\n";
				msgInProgress += "- " + area.trim() + "\n";
				const names = areas[area];
				for (const name of names) {
					msgInProgress += "	* " + name.trim() + "\n";
				}
			}

			messages.push({
				zone,
				message: msgInProgress,
				sent: false
			});
			count++;
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
		<h2 class="text-2xl font-bold text-primary">Construct Messages</h2>
		<p class="text-sm text-surface-500">
			Generate prompts and input AI responses to build your messages to the zones.
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
			<p>Payload loaded and ready to generate prompts!</p>
		</div>
		<div class="space-y-4">
			<div class="card p-4 bg-surface-200-800">
				<h3 class="font-semibold mb-2">What will happen:</h3>
				<ol class="list-decimal list-inside space-y-1 text-sm text-surface-600-400">
					<li>You'll see prompts for each zone</li>
					<li>Copy each prompt to your favorite LLM (ChatGPT, Claude, etc.)</li>
					<li>Paste the AI-generated response back here</li>
					<li>Review the final formatted messages and send them</li>
				</ol>
			</div>
			
			<div class="flex w-full justify-center pt-2">
				<button on:click={generatePrompts} class="btn preset-filled-primary-500 w-full sm:w-1/2">
					Generate Prompts
				</button>
			</div>
		</div>

	{:else if currentStep === 1}
		<!-- Input Prompts State -->
		<div class="space-y-4">
			<!-- Progress Indicator -->
			<div class="card p-4 bg-surface-200-800">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-semibold">Progress</span>
					<span class="text-sm text-surface-600-400">
						{prompts.filter((p) => p.userResponse.trim().length > 0).length} / {prompts.length} answered
					</span>
				</div>
				<Progress value={prompts.filter((p) => p.userResponse.trim().length > 0).length} max={prompts.length}>
					<Progress.Track class="fill-secondary-50-950">
						<Progress.Range />
					</Progress.Track>
				</Progress>
			</div>

			<!-- Current Prompt Card -->
			{#each prompts as prompt, index}
				{#if index === currentPromptIndex}
					<div class="card p-6 preset-tonal-primary border-2 border-primary">
						<div class="mb-4">
							<h3 class="text-lg font-bold">Zone: {prompt.zone}</h3>
							<p class="text-sm text-surface-500">Prompt {index + 1} of {prompts.length}</p>
						</div>

						<div class="space-y-4">
							<!-- Prompt Display -->
							<div>
								<label class="block text-sm font-semibold mb-2">Prompt to use:</label>
								<div class="bg-surface-100-900 p-4 rounded-lg flex items-start justify-between gap-4">
									<p class="text-sm font-mono whitespace-pre-wrap flex-1">{prompt.prompt}</p>
									<button
										on:click={() => copyPrompt(index)}
										class="btn btn-sm preset-filled flex-shrink-0"
										title="Copy prompt"
									>
										<Copy size={16} />
									</button>
								</div>
							</div>

							<!-- Response Input -->
							<div>
								<label for="response-{index}" class="block text-sm font-semibold mb-2">
									Paste the AI-generated response here:
								</label>
								<textarea
									id="response-{index}"
									bind:value={prompt.userResponse}
									class="textarea text-sm"
									placeholder="Paste the AI response here..."
									rows="6"
								></textarea>
							</div>

							<!-- Navigation Buttons -->
							<div class="flex gap-2">
								<button
									on:click={moveToNextPrompt}
									class="btn preset-filled flex-1"
									disabled={currentPromptIndex >= prompts.length - 1 || prompt.userResponse.trim().length === 0}
								>
									Next Prompt
									<ChevronRight size={16} class="ml-2" />
								</button>
								<button
									on:click={finishPrompts}
									class="btn preset-filled-success flex-1"
									disabled={!allPromptsAnswered()}
								>
									<Check size={16} class="mr-2" />
									All Done
								</button>
							</div>

							<p class="text-xs text-surface-500 text-center">
								Fill in all prompts before you can continue
							</p>
						</div>
					</div>
				{/if}
			{/each}

			<!-- Completed Prompts (Collapsed) -->
			{#if prompts.filter((p) => p.userResponse.trim().length > 0).length > 0}
				<div class="card p-4 bg-surface-200-800">
					<h4 class="font-semibold mb-2 text-sm">Completed Zones</h4>
					<div class="space-y-1">
						{#each prompts as prompt, index}
							{#if prompt.userResponse.trim().length > 0 && index !== currentPromptIndex}
								<div class="flex items-center justify-between text-sm py-1">
									<span class="text-surface-600-400">{prompt.zone}</span>
									<CircleCheckBig size={16} class="text-success" />
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
			<div class="card p-4 bg-surface-200-800">
				<div class="flex items-center justify-between mb-2">
					<span class="text-sm font-semibold">Progress</span>
					<span class="text-sm text-surface-600-400">
						{messages.filter((m) => m.sent).length} / {messages.length} sent
					</span>
				</div>
				<Progress value={messages.filter((m) => m.sent).length} max={messages.length}>
					<Progress.Track class="fill-secondary-50-950">
						<Progress.Range />
					</Progress.Track>
				</Progress>
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
							{#if !copied}
							<button
								on:click={() => copyMessage(index)}
								class="btn preset-filled flex-1"
								disabled={msg.sent}
							>
								<Copy size={16} class="mr-2" />
								Copy Message
							</button>
							{:else}
							<button
								class="btn preset-filled flex-1"
								disabled
							>
								<Check size={16} class="mr-2" />
							</button>
							{/if}
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