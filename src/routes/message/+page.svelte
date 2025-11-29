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
        <Fetch />
      </div>
    </Tabs.Content>
    <Tabs.Content value="generate">
      <div class="card mt-5 mb-10 shadow-xl mx-10 bg-surface-200-800 p-6 space-y-2 text-center">
        <Generate />
      </div>
    </Tabs.Content>
  </Tabs>
