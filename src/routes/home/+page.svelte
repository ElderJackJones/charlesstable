<script>
	import { Rabbit, Pilcrow, Telescope, Bell, Users } from "@lucide/svelte";

	let currentYear = new Date().getFullYear();
</script>

<div class="container mx-auto px-4 py-8 max-w-6xl">
	<!-- Header Section -->
	<header class="text-center mb-12">
		<div class="flex justify-center items-center mb-6">
			<div class="w-20 h-20 rounded-full border-4 border-[#005175] dark:border-[#339acc] flex items-center justify-center bg-[#005175]/8 dark:bg-[#339acc]/10">
				<span class="text-3xl font-bold text-[#005175] dark:text-[#339acc] select-none leading-none">C</span>
			</div>
		</div>
		<h1 class="text-3xl md:text-4xl font-bold text-[#005175] dark:text-[#339acc] mb-4">Meet Charles</h1>
		<p class="text-xl text-gray-600 dark:text-gray-300 mb-6 max-w-2xl mx-auto">
			An internal AI-assisted tool that helps social media leaders follow up on media referrals
			in the South Carolina Charleston Mission
		</p>
	</header>

	<!-- Main Content Grid -->
	<div class="grid lg:grid-cols-2 gap-8 lg:gap-12 mb-16">
		<!-- Left Column -->
		<div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-8">
			<h2 class="text-2xl font-semibold text-[#005175] dark:text-[#339acc] mb-6">What is Charles?</h2>
			<div class="space-y-4 text-gray-800 dark:text-gray-200">
				<p>
					Charles is a future-proofed internal application designed to assist social media leaders
					in managing and following up on media referrals that have not yet been successfully contacted.
				</p>
				<p>
					Charles does not act on his own. Instead, he works alongside missionaries by identifying
					uncontacted referrals and helping generate friendly, encouraging messages that a social
					media leader can review and send.
				</p>
			</div>
		</div>

		<!-- Right Column -->
		<div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-8">
			<h2 class="text-2xl font-semibold text-[#005175] dark:text-[#339acc] mb-6">How It Works</h2>
			<div class="space-y-6">
				<div class="flex items-start space-x-4">
					<div class="min-w-8 w-8 h-8 bg-[#005175] dark:bg-[#339acc] rounded-full flex items-center justify-center text-white font-bold">1</div>
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-1">Review Referrals</h3>
						<p class="text-gray-700 dark:text-gray-300">
							A social media leader opens Charles to review media referrals that still need contact.
						</p>
					</div>
				</div>

				<div class="flex items-start space-x-4">
					<div class="min-w-8 w-8 h-8 bg-[#005175] dark:bg-[#339acc] rounded-full flex items-center justify-center text-white font-bold">2</div>
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-1">Get AI Assistance</h3>
						<p class="text-gray-700 dark:text-gray-300">
							Charles helps draft warm, encouraging messages designed to motivate missionaries to reach out.
						</p>
					</div>
				</div>

				<div class="flex items-start space-x-4">
					<div class="min-w-8 w-8 h-8 bg-[#005175] dark:bg-[#339acc] rounded-full flex items-center justify-center text-white font-bold">3</div>
					<div>
						<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-1">Human Sends the Message</h3>
						<p class="text-gray-700 dark:text-gray-300">
							The social media leader reviews the message and sends it—keeping humans fully in control.
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Features Section -->
	<section class="mb-16">
		<h2 class="text-2xl font-semibold text-center text-[#005175] dark:text-[#339acc] mb-10">Why Charles Helps</h2>
		<div class="grid md:grid-cols-3 gap-8">
			<div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-6 text-center">
				<div class="w-16 h-16 bg-[#005175]/10 dark:bg-[#339acc]/10 text-[#005175] dark:text-[#339acc] rounded-full flex items-center justify-center mx-auto mb-4">
					<Bell size="30"/>
				</div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2">Clear Visibility</h3>
				<p class="text-gray-600 dark:text-gray-300 text-sm">
					Quickly see which media referrals still need follow-up—no guesswork.
				</p>
			</div>

			<div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-6 text-center">
				<div class="w-16 h-16 bg-[#005175]/10 dark:bg-[#339acc]/10 text-[#005175] dark:text-[#339acc] rounded-full flex items-center justify-center mx-auto mb-4">
					<Pilcrow size="30"/>
				</div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2">Human-Centered AI</h3>
				<p class="text-gray-600 dark:text-gray-300 text-sm">
					Charles assists with wording and encouragement, but missionaries make the final call.
				</p>
			</div>

			<div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-6 text-center">
				<div class="w-16 h-16 bg-[#005175]/10 dark:bg-[#339acc]/10 text-[#005175] dark:text-[#339acc] rounded-full flex items-center justify-center mx-auto mb-4">
					<Users size="30"/>
				</div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2">Built for Leaders</h3>
				<p class="text-gray-600 dark:text-gray-300 text-sm">
					Designed to support social media leaders without replacing personal judgment or stewardship.
				</p>
			</div>
		</div>
	</section>

	<!-- Getting Started -->
	<section class="text-center mb-16">
		<div class="bg-[#005175]/5 dark:bg-[#339acc]/5 border border-[#005175]/20 dark:border-[#339acc]/20 shadow-sm rounded-lg p-10 md:p-12 max-w-4xl mx-auto">
			<h2 class="text-2xl font-semibold text-[#005175] dark:text-[#339acc] mb-4">A Helping Hand, Not an Autopilot</h2>
			<p class="text-lg text-gray-700 dark:text-gray-300">
				Charles exists to support missionary efforts—not automate them.
				Every message is reviewed and sent by a real person, with Charles providing structure,
				clarity, and encouragement along the way.
			</p>
		</div>
	</section>

	<!-- Footer -->
	<footer class="text-center pt-8 border-t border-gray-300 dark:border-gray-700 mb-4">
		<p class="text-gray-500 dark:text-gray-400">
			© {currentYear} Charles — South Carolina Charleston Mission
		</p>
		<p class="text-sm text-gray-400 dark:text-gray-500 mt-2">
			Internal AI-assisted support for social media leaders
		</p>
	</footer>
</div>

<style>
	.container {
		min-height: 100vh;
	}
</style>
