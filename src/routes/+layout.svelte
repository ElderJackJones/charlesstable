<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
  	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { loadSettings } from '$lib/stores/settings';
  	// Icons
	import { House, MessageSquareText, FileSpreadsheet, Cog } from '@lucide/svelte';

  // State
    let value = $state('home');
	let { children } = $props();

  // Handlers
  const handleClick = (newValue: string) => {
	value = newValue
	goto(`/${newValue}`);
  };


	// Load settings and apply theme on app startup
	onMount(() => {
		if (browser) {
			// This will load settings from localStorage and apply the saved theme
			loadSettings();
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="card border-surface-100-900 grid h-dvh w-full grid-cols-[auto_1fr] border-[1px] overflow-hidden">
  <!-- Component -->
  <Navigation.Rail {value} onValueChange={(newValue) => (handleClick(newValue))}>
    {#snippet tiles()}
      <Navigation.Tile id="home" label="Home"><House /></Navigation.Tile>
      <Navigation.Tile id="message" label="Message"><MessageSquareText /></Navigation.Tile>
      <Navigation.Tile id="data" label="Data"><FileSpreadsheet /></Navigation.Tile>
      <Navigation.Tile id="settings" label="Settings"><Cog /></Navigation.Tile>
    {/snippet}
  </Navigation.Rail>
  <!-- Content -->
  <div class="flex overflow-y-auto">
    {@render children?.()}
  </div>
</div>


