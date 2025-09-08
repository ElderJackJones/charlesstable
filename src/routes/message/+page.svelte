<!-- src/lib/components/MessengerIntegration.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
	import { Check, Info } from '@lucide/svelte';
  
  let isMessengerOpen = false;
  let isLoading = false;
  let error: string | null = null;

  // Check if messenger window is already open on component mount
  onMount(async () => {
    try {
      isMessengerOpen = await invoke('is_messenger_window_open');
    } catch (err) {
      console.error('Error checking messenger window status:', err);
    }
  });

  async function openMessenger() {
    isLoading = true;
    error = null;
    
    try {
      await invoke('open_messenger_window');
      isMessengerOpen = true;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Error opening messenger:', err);
    } finally {
      isLoading = false;
    }
  }

  async function closeMessenger() {
    isLoading = true;
    error = null;
    
    try {
      await invoke('close_messenger_window');
      isMessengerOpen = false;
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      console.error('Error closing messenger:', err);
    } finally {
      isLoading = false;
    }
  }

  async function refreshStatus() {
    try {
      isMessengerOpen = await invoke('is_messenger_window_open');
    } catch (err) {
      console.error('Error refreshing status:', err);
    }
  }
</script>

<div class="card p-6 space-y-6 max-w-2xl mx-auto">
  <!-- Header -->
  <header class="flex justify-between items-center">
    <h2 class="h2">Facebook Messenger</h2>
    <div class="flex items-center gap-2">
      <span class="text-sm opacity-75">Status:</span>
      <span class="badge {isMessengerOpen ? 'variant-filled-success' : 'variant-filled-error'}">
        {isMessengerOpen ? 'Open' : 'Closed'}
      </span>
    </div>
  </header>

  <!-- Error Alert -->
  {#if error}
    <aside class="alert variant-filled-error">
      <div class="alert-message">
        <h3 class="h3">Error</h3>
        <p>{error}</p>
      </div>
    </aside>
  {/if}

  <!-- Controls -->
  <div class="btn-group variant-filled space-x-2">
    <button 
      class="btn variant-filled-primary"
      on:click={openMessenger} 
      disabled={isLoading || isMessengerOpen}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd" />
      </svg>
      <span>{isLoading ? 'Opening...' : 'Open Messenger'}</span>
    </button>

    <button 
      class="btn variant-filled-secondary"
      on:click={closeMessenger} 
      disabled={isLoading || !isMessengerOpen}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 000 2h4a1 1 0 100-2H8z" clip-rule="evenodd" />
      </svg>
      <span>{isLoading ? 'Closing...' : 'Close Messenger'}</span>
    </button>

    <button 
      class="btn variant-ghost-surface"
      on:click={refreshStatus}
      disabled={isLoading}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
        <path fill-rule="evenodd" d="M15.312 11.424a5.5 5.5 0 01-9.201 2.466l-.312-.311h2.433a.75.75 0 000-1.5H3.989a.75.75 0 00-.75.75v4.242a.75.75 0 001.5 0v-2.43l.31.31a7 7 0 0011.712-3.138.75.75 0 00-1.449-.39zm1.23-3.723a.75.75 0 00.219-.53V2.929a.75.75 0 00-1.5 0V5.36l-.31-.31A7 7 0 003.239 8.188a.75.75 0 101.448.389A5.5 5.5 0 0113.89 6.11l.311.31h-2.432a.75.75 0 000 1.5h4.243a.75.75 0 00.53-.219z" clip-rule="evenodd" />
      </svg>
      <span>Refresh</span>
    </button>
  </div>

  <!-- Information Card -->
  <div class="card variant-glass p-4 space-y-3">
    <div class="flex items-start gap-3">
      <Info size="20" class="text-primary-500 mt-0.5 flex-shrink-0" />
      <div class="space-y-2">
        <p class="font-semibold">Integration Details</p>
        <p class="text-sm opacity-75">
          This opens Facebook Messenger in a separate window. You'll need to log in with your Facebook credentials.
        </p>
        <ul class="list space-y-1 text-sm opacity-75">
          <li class="flex items-center gap-2">
            <Check size="16" class="text-success-500 flex-shrink-0" />
            <span>The window can be resized and moved independently</span>
          </li>
          <li class="flex items-center gap-2">
            <Check size="16" class="text-success-500 flex-shrink-0" />
            <span>Your session will be maintained between opens/closes</span>
          </li>
          <li class="flex items-center gap-2">
            <Check size="16" class="text-success-500 flex-shrink-0" />
            <span>Notifications will work within the Messenger window</span>
          </li>
        </ul>
      </div>
    </div>
  </div>
</div>