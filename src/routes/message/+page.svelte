<!-- src/lib/components/MessengerIntegration.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  
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

<div class="messenger-integration">
  <div class="header">
    <h2>Facebook Messenger</h2>
    <div class="status">
      Status: <span class="status-indicator" class:open={isMessengerOpen} class:closed={!isMessengerOpen}>
        {isMessengerOpen ? 'Open' : 'Closed'}
      </span>
    </div>
  </div>

  {#if error}
    <div class="error">
      Error: {error}
    </div>
  {/if}

  <div class="controls">
    <button 
      on:click={openMessenger} 
      disabled={isLoading || isMessengerOpen}
      class="btn btn-primary"
    >
      {#if isLoading}
        Opening...
      {:else}
        Open Messenger
      {/if}
    </button>

    <button 
      on:click={closeMessenger} 
      disabled={isLoading || !isMessengerOpen}
      class="btn btn-secondary"
    >
      {#if isLoading}
        Closing...
      {:else}
        Close Messenger
      {/if}
    </button>

    <button 
      on:click={refreshStatus}
      disabled={isLoading}
      class="btn btn-tertiary"
    >
      Refresh Status
    </button>
  </div>

  <div class="info">
    <p>
      <strong>Note:</strong> This will open Facebook Messenger in a separate window. 
      You'll need to log in with your Facebook credentials.
    </p>
    <ul>
      <li>The window can be resized and moved independently</li>
      <li>Your session will be maintained between opens/closes</li>
      <li>Notifications will work within the Messenger window</li>
    </ul>
  </div>
</div>

<style>
  .messenger-integration {
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #e9ecef;
    max-width: 600px;
    margin: 0 auto;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .header h2 {
    margin: 0;
    color: #212529;
  }

  .status {
    font-size: 0.9rem;
    color: #6c757d;
  }

  .status-indicator {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: bold;
    text-transform: uppercase;
    font-size: 0.8rem;
  }

  .status-indicator.open {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }

  .status-indicator.closed {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
  }

  .error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
    padding: 0.75rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background-color 0.2s, opacity 0.2s;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #007bff;
    color: white;
  }

  .btn-primary:not(:disabled):hover {
    background: #0056b3;
  }

  .btn-secondary {
    background: #6c757d;
    color: white;
  }

  .btn-secondary:not(:disabled):hover {
    background: #545b62;
  }

  .btn-tertiary {
    background: #e9ecef;
    color: #495057;
    border: 1px solid #ced4da;
  }

  .btn-tertiary:not(:disabled):hover {
    background: #f8f9fa;
  }

  .info {
    background: white;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid #dee2e6;
  }

  .info p {
    margin: 0 0 0.5rem 0;
    color: #495057;
  }

  .info ul {
    margin: 0.5rem 0 0 0;
    color: #6c757d;
    font-size: 0.9rem;
  }

  .info li {
    margin-bottom: 0.25rem;
  }
</style>