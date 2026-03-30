<script lang="ts">
  import Fetch from './Fetch.svelte';
  import Generate from './Generate.svelte'
  let activeTab = $state('fetch');
</script>

<div class="w-full max-w-4xl mx-auto px-4 mt-2">
  <div class="flex border-b border-gray-200 dark:border-gray-700 mb-6">
    <button
      class="py-2 px-4 font-semibold mr-2 border-b-2 transition-colors {activeTab === 'fetch' ? 'border-[#005175] dark:border-[#339acc] text-[#005175] dark:text-[#339acc]' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
      onclick={() => activeTab = 'fetch'}
    >
      Fetch
    </button>
    <button
      class="py-2 px-4 font-semibold border-b-2 transition-colors {activeTab === 'generate' ? 'border-[#005175] dark:border-[#339acc] text-[#005175] dark:text-[#339acc]' : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
      onclick={() => activeTab = 'generate'}
    >
      Generate
    </button>
  </div>

  <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-6 mb-10 min-h-[500px]">
    {#if activeTab === 'fetch'}
      <Fetch />
    {:else}
      <Generate />
    {/if}
  </div>
</div>
