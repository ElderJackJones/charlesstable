<script>
    import { Rabbit, Pilcrow, Telescope } from "@lucide/svelte";

	// Add any reactive variables or functions here
	let currentYear = new Date().getFullYear();
	
</script>

<div class="container mx-auto px-4 py-8 max-w-6xl">
	<!-- Header Section -->
	<header class="text-center mb-12">
		<div class="flex justify-center items-center mb-6">
			<!-- Bot Avatar/Icon -->
			<div class="w-20 h-20 bg-primary-500 rounded-full flex items-center justify-center mb-4">
				<span class="text-3xl font-bold text-white">C</span>
			</div>
		</div>
		<h1 class="h1 mb-4">Meet Charles</h1>
		<p class="text-xl text-surface-600-300-token mb-6 max-w-2xl mx-auto">
			Your automated media referral assistant for the South Carolina Charleston LDS Mission
		</p>
	</header>

	<!-- Main Content Grid -->
	<div class="grid lg:grid-cols-2 gap-12 mb-16">
		<!-- Left Column - What is Charles -->
		<div class="card p-8">
			<h2 class="h2 mb-6">What is Charles?</h2>
			<div class="space-y-4 text-surface-700-200-token">
				<p>
					Charles is an intelligent automated messaging system designed specifically for missionaries 
					serving in the South Carolina Charleston LDS Mission. He streamlines the process of sharing 
					media referral information, making it easier than ever to connect with potential investigators.
				</p>
				<p>
					With Charles, you can quickly send referral details, contact information, and follow-up 
					instructions through a simple, guided process that ensures nothing gets missed.
				</p>
			</div>
		</div>

		<!-- Right Column - How it Works -->
		<div class="card p-8">
			<h2 class="h2 mb-6">How It Works</h2>
			<div class="space-y-6">
				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">1</div>
					<div>
						<h3 class="h4 mb-2">Select Message</h3>
						<p class="text-surface-700-200-token">Open the app and select the 'Message' option to begin</p>
					</div>
				</div>
				
				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">2</div>
					<div>
						<h3 class="h4 mb-2">Follow Prompts</h3>
						<p class="text-surface-700-200-token">Charles will guide you through each step with clear prompts</p>
					</div>
				</div>
				
				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">3</div>
					<div>
						<h3 class="h4 mb-2">Send Referral</h3>
						<p class="text-surface-700-200-token">Your media referral information is automatically formatted and sent</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Features Section -->
	<section class="mb-16">
		<h2 class="h2 text-center mb-10">Why We Love Charles</h2>
		<div class="grid md:grid-cols-3 gap-8">
			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-secondary-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Rabbit size="30"/>
				</div>
				<h3 class="h4 mb-3">Simple & Fast</h3>
				<p class="text-surface-600-300-token">No complex forms or confusing interfaces. Just follow the prompts and you're done.</p>
			</div>

			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-tertiary-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Pilcrow  size="30"/>
				</div>
				<h3 class="h4 mb-3">Automated Formatting</h3>
				<p class="text-surface-600-300-token">Charles ensures your referral messages are properly formatted and professional.</p>
			</div>

			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-success-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Telescope size="30"/>
				</div>
				<h3 class="h4 mb-3">Mission Focused</h3>
				<p class="text-surface-600-300-token">Built specifically for Charleston Mission needs with local context in mind.</p>
			</div>
		</div>
	</section>

	<!-- Getting Started Section -->
	<section class="text-center mb-16">
		<div class="card p-12">
			<h2 class="h2 mb-6">Ready to Get Started?</h2>
			<p class="text-xl text-surface-600-300-token max-w-2xl mx-auto">
				Begin using Charles to streamline your media referral process. Simply select 'Message' 
				in the app and follow the guided prompts.
			</p>

		</div>
	</section>

	<!-- Footer -->
	<footer class="text-center pt-8 border-t border-surface-300-600-token mb-4">
		<p class="text-surface-500-400-token">
			© {currentYear} Charles Bot - South Carolina Charleston Mission
		</p>
		<p class="text-sm text-surface-400-500-token mt-2 ">
			Serving missionaries with automated referral assistance
		</p>
	</footer>
</div>

<style>
	/* Custom styles if needed - Skeleton UI handles most styling */
	.container {
		min-height: 100vh;
	}
</style>