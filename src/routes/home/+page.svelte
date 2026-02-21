<script>
	import { Rabbit, Pilcrow, Telescope, Bell, Users } from "@lucide/svelte";
	import { Avatar } from "@skeletonlabs/skeleton-svelte";

	let currentYear = new Date().getFullYear();
</script>

<div class="container mx-auto px-4 py-8 max-w-6xl">
	<!-- Header Section -->
	<header class="text-center mb-12">
		<div class="flex justify-center items-center mb-6">
			<img class="w-20 h-20" src="/icon.png" alt="Charles App Icon">
		</div>
		<h1 class="h1 mb-4">Meet Charles</h1>
		<p class="text-xl text-surface-600-300-token mb-6 max-w-2xl mx-auto">
			An internal AI-assisted tool that helps social media leaders follow up on media referrals
			in the South Carolina Charleston Mission
		</p>
	</header>

	<!-- Main Content Grid -->
	<div class="grid lg:grid-cols-2 gap-12 mb-16">
		<!-- Left Column -->
		<div class="card p-8">
			<h2 class="h2 mb-6">What is Charles?</h2>
			<div class="space-y-4 text-surface-700-200-token">
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
		<div class="card p-8">
			<h2 class="h2 mb-6">How It Works</h2>
			<div class="space-y-6">
				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">1</div>
					<div>
						<h3 class="h4 mb-2">Review Referrals</h3>
						<p class="text-surface-700-200-token">
							A social media leader opens Charles to review media referrals that still need contact.
						</p>
					</div>
				</div>

				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">2</div>
					<div>
						<h3 class="h4 mb-2">Get AI Assistance</h3>
						<p class="text-surface-700-200-token">
							Charles helps draft warm, encouraging messages designed to motivate missionaries to reach out.
						</p>
					</div>
				</div>

				<div class="flex items-start space-x-4">
					<div class="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white font-bold">3</div>
					<div>
						<h3 class="h4 mb-2">Human Sends the Message</h3>
						<p class="text-surface-700-200-token">
							The social media leader reviews the message and sends it—keeping humans fully in control.
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Features Section -->
	<section class="mb-16">
		<h2 class="h2 text-center mb-10">Why Charles Helps</h2>
		<div class="grid md:grid-cols-3 gap-8">
			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-secondary-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Bell size="30"/>
				</div>
				<h3 class="h4 mb-3">Clear Visibility</h3>
				<p class="text-surface-600-300-token">
					Quickly see which media referrals still need follow-up—no guesswork.
				</p>
			</div>

			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-tertiary-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Pilcrow size="30"/>
				</div>
				<h3 class="h4 mb-3">Human-Centered AI</h3>
				<p class="text-surface-600-300-token">
					Charles assists with wording and encouragement, but missionaries make the final call.
				</p>
			</div>

			<div class="card p-6 text-center">
				<div class="w-16 h-16 bg-success-500 rounded-full flex items-center justify-center mx-auto mb-4">
					<Users size="30"/>
				</div>
				<h3 class="h4 mb-3">Built for Leaders</h3>
				<p class="text-surface-600-300-token">
					Designed to support social media leaders without replacing personal judgment or stewardship.
				</p>
			</div>
		</div>
	</section>

	<!-- Getting Started -->
	<section class="text-center mb-16">
		<div class="card p-12">
			<h2 class="h2 mb-6">A Helping Hand, Not an Autopilot</h2>
			<p class="text-xl text-surface-600-300-token max-w-2xl mx-auto">
				Charles exists to support missionary efforts—not automate them.
				Every message is reviewed and sent by a real person, with Charles providing structure,
				clarity, and encouragement along the way.
			</p>
		</div>
	</section>

	<!-- Footer -->
	<footer class="text-center pt-8 border-t border-surface-300-600-token mb-4">
		<p class="text-surface-500-400-token">
			© {currentYear} Charles — South Carolina Charleston Mission
		</p>
		<p class="text-sm text-surface-400-500-token mt-2">
			Internal AI-assisted support for social media leaders
		</p>
	</footer>
</div>

<style>
	.container {
		min-height: 100vh;
	}
</style>
