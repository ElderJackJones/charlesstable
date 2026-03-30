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

  {#if activeTab === 'fetch'}
    <div class="bg-yellow-50 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200 shadow-md rounded-lg border border-yellow-200 dark:border-yellow-800 p-6 mb-5 text-center">
      <h2 class="text-xl font-bold mb-2">Advanced Mode</h2>
      <p class="text-sm">This page is for power users and tinkerers. <br />
      Here you can see the exact prompt Charles would send to an AI model. If you ever want to run it through a different LLM—ChatGPT, Claude, Gemini, or something else—you can copy and paste it directly. <br />
      Most users won’t need this. But if you like transparency, control, or future-proofing your workflow, you’re in the right place.</p>
    </div>
  {/if}

  <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg border border-gray-300 dark:border-gray-700 p-6 mb-10 min-h-[500px]">
    {#if activeTab === 'fetch'}
      <Fetch />
    {:else}
      <Generate />
    {/if}
  </div>
</div>
