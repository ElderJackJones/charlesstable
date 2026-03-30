<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	// Icons
	import { House, MessageSquareText, KeySquare } from '@lucide/svelte';

	// State - derive navigation value from current route
	let value = $state('home');
	let { children } = $props();

	const links = [
		{ id: 'home', label: 'Home', icon: House, href: '/' },
		{ id: 'message', label: 'Message', icon: MessageSquareText, href: '/message' },
		{ id: 'advanced', label: 'Advanced', icon: KeySquare, href: '/advanced'}
	];

	// Update navigation value when route changes
	$effect(() => {
		if ($page.route.id) {
			const routePath = $page.route.id.replace('/[', '').replace(']', '');
			// Extract the route segment (home, message, advanced)
			const segment = routePath.split('/')[1] || 'home';
			value = segment;
		}
	});

	// Handlers
	const handleClick = (newValue: string, event: Event) => {
		event.preventDefault();
		value = newValue;
		goto(`/${newValue === 'home' ? '' : newValue}`);
	};
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex h-dvh w-full overflow-hidden bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-200">
	<!-- Sidebar Navigation -->
	<nav class="w-20 lg:w-24 shrink-0 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col items-center justify-center py-6 gap-2">
		{#each links as link (link.id)}
			{@const Icon = link.icon}
			{@const isActive = value === link.id}
			<a
				href={link.href}
				onclick={(e) => handleClick(link.id, e)}
				class="group flex flex-col items-center justify-center gap-1 w-16 h-16 lg:w-20 lg:h-20 rounded-lg transition-colors {isActive ? 'text-[#005175] dark:text-[#339acc] bg-[#005175]/10 dark:bg-[#339acc]/10 font-medium' : 'text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700'}"
			>
				<Icon class="w-6 h-6 lg:w-7 lg:h-7" />
				<span class="text-[10px] lg:text-xs">{link.label}</span>
			</a>
		{/each}
	</nav>

	<!-- Content -->
	<main class="flex-1 overflow-y-auto">
		{@render children?.()}
	</main>
</div>