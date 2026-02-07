<script lang="ts">
  import Fetch from './Fetch.svelte';
  import Generate from './Generate.svelte'
  import { Tabs } from '@skeletonlabs/skeleton-svelte';
</script>


  <Tabs defaultValue="fetch" class="mt-2">
    <Tabs.List>
      <Tabs.Trigger class="ml-2" value="fetch">Fetch</Tabs.Trigger>
      <Tabs.Trigger value="generate">Generate</Tabs.Trigger>
      <Tabs.Indicator />
    </Tabs.List>
    <Tabs.Content value="fetch">
        <div class="card mt-5 shadow-xl mx-10 bg-surface-200-800 p-6 space-y-2 text-center">
            <h5 class="h5">Advanced Mode</h5>
            <p>This page is for power users and tinkerers. <br />
                Here you can see the exact prompt Charles would send to an AI model. If you ever want to run it through a different LLM—ChatGPT, Claude, Gemini, or something else—you can copy and paste it directly. <br />
                Most users won’t need this. But if you like transparency, control, or future-proofing your workflow, you’re in the right place.</p>
        </div>
      <div class="card mt-5 mb-5 shadow-xl mx-10 bg-surface-200-800 p-6 space-y-2 text-center">
        <Fetch />
      </div>
    </Tabs.Content>
    <Tabs.Content value="generate">
      <div class="card mt-5 mb-10 shadow-xl mx-10 bg-surface-200-800 p-6 space-y-2 text-center">
        <Generate />
      </div>
    </Tabs.Content>
  </Tabs>
