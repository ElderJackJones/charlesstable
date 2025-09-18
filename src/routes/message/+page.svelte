<script lang="ts">
	let chatInput = "";
</script>

<!-- wrapper that fills the whole window -->
<div class="w-full h-full p-4 grid gap-4 
	grid-cols-1 md:grid-cols-2 lg:grid-cols-3 
	auto-rows-fr
  [grid-auto-flow:dense]">

	<!-- Charles settings -->
	<div class="preset-filled-surface-100-900 card p-4 flex flex-col">
		<h2 class="font-bold mb-2">Charles Settings</h2>
		<div class="flex-1 flex flex-col gap-2">
			<div class="skeleton h-8 w-2/3"></div>
			<div class="skeleton h-6 w-1/2"></div>
			<div class="skeleton h-6 w-1/4"></div>
		</div>
	</div>

	<!-- Messenger control -->
	<div class="preset-filled-surface-100-900 card p-4 flex flex-col">
		<h2 class="font-bold mb-2">Messenger Control</h2>
		<div class="flex-1 flex flex-col gap-2">
			<div class="skeleton h-6 w-2/3"></div>
			<div class="skeleton h-10 w-24"></div>
		</div>
	</div>

	<!-- User progress -->
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
