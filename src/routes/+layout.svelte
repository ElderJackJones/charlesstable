<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
  	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
  	// Icons
	import { House, MessageSquareText, FileSpreadsheet, CircleQuestionMark } from '@lucide/svelte';

  // State - derive navigation value from current route
    let value = $state('home');
	let { children } = $props();

	const links = [
		{ id: 'home', label: 'Home', icon: House, href: '/' },
		{ id: 'message', label: 'Message', icon: MessageSquareText, href: '/message' },
		{ id: 'data', label: 'Data', icon: FileSpreadsheet, href: '/data' },
		{ id: 'help', label: 'Help', icon: CircleQuestionMark, href: '/help'}
	];

	  let anchorRail = 'btn hover:preset-tonal aspect-square w-full max-w-[84px] flex flex-col items-center gap-0.5';

	// Update navigation value when route changes
	$effect(() => {
		if ($page.route.id) {
			const routePath = $page.route.id.replace('/[', '').replace(']', '');
			// Extract the route segment (home, data, message, settings)
			const segment = routePath.split('/')[1] || 'home';
			value = segment;
		}
	});

  // Handlers
  const handleClick = (newValue: string) => {
	value = newValue
	goto(`/${newValue}`);
  };
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="card border-surface-100-900 grid h-dvh w-full grid-cols-[auto_1fr] border-[1px] overflow-hidden">
  <!-- Component -->
  <Navigation layout="rail">
    <Navigation.Menu>
	  {#each links as link (link)}
          {@const Icon = link.icon}
          <a href={link.href} class={anchorRail}>
            <Icon class="size-5" />
            <span class="text-xs">{link.label}</span>
          </a>
        {/each}
	</Navigation.Menu>
  </Navigation>
  <!-- Content -->
  <div class="flex overflow-y-auto">
    {@render children?.()}
  </div>
</div>


