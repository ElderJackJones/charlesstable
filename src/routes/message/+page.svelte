<script lang="ts">
	let includeDay = false;
	let includeSeason = false;
	let includeInstructions = false;
	let selectedModel: "Sage" | "Jest" = "Sage";
</script>

<div class="w-full h-full p-4 grid gap-4
	grid-cols-1 md:grid-cols-2 lg:grid-cols-3
	auto-rows-fr
	[grid-auto-flow:dense]">

	<!-- Charles Settings -->
	<div class="preset-filled-surface-100-900 card p-4 flex flex-col">
		<h2 class="font-bold mb-4 text-lg">Charles Settings</h2>
		<div class="flex-1 flex flex-col gap-4">

			<!-- Checkboxes grid -->
			<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
				<label class="flex items-center gap-2">
					<input type="checkbox" class="checkbox" bind:checked={includeDay} />
					<span>Include Day</span>
				</label>
				<label class="flex items-center gap-2">
					<input type="checkbox" class="checkbox" bind:checked={includeSeason} />
					<span>Include Season</span>
				</label>
				<label class="flex items-center gap-2">
					<input type="checkbox" class="checkbox" bind:checked={includeInstructions} />
					<span>Include Additional Instructions</span>
				</label>
			</div>

			<!-- Model Dropdown -->
			<div class="flex flex-col sm:flex-row sm:items-center sm:gap-4 mt-2">
				<span class="font-medium mb-1 sm:mb-0">Select Model:</span>
				<select bind:value={selectedModel} class="select w-full sm:w-auto p-2 rounded border">
					<option value="Sage">Sage</option>
					<option value="Jest">Jest</option>
				</select>
			</div>

		</div>
	</div>

	<!-- Messenger Control -->
	<div class="preset-filled-surface-100-900 card p-4 flex flex-col">
		<h2 class="font-bold mb-2">Messenger Control</h2>
		<div class="flex-1 flex flex-col gap-2">
			<div class="skeleton h-6 w-2/3"></div>
			<div class="skeleton h-10 w-24"></div>
		</div>
	</div>

	<!-- User Progress -->
	<div class="md:col-span-2 lg:col-span-1 preset-filled-surface-100-900 card p-4 flex flex-col">
		<h2 class="font-bold mb-2">Progress</h2>
		<div class="flex-1 flex flex-col gap-2">
			<div class="skeleton h-6 w-3/4"></div>
			<div class="skeleton h-6 w-1/2"></div>
			<div class="skeleton h-8 w-28 mt-auto"></div>
		</div>
	</div>

	<!-- Chat with Charles (full width) -->
	<div class="preset-filled-surface-100-900 card p-4 flex flex-col md:col-span-2 lg:col-span-3">
		<h2 class="font-bold mb-2">Chat with Charles</h2>
		<div class="flex-1 flex flex-col gap-2">
			<div class="flex-1 grid gap-2">
				<div class="skeleton h-6 w-2/3"></div>
				<div class="skeleton h-6 w-1/2 ml-auto"></div>
				<div class="skeleton h-6 w-3/4"></div>
			</div>
			<div class="flex gap-2">
				<div class="skeleton h-10 flex-1"></div>
				<div class="skeleton h-10 w-20"></div>
			</div>
		</div>
	</div>
</div>
