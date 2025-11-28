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
      <div class="card mt-5 mx-10 max-h-[75vh] overflow-y-auto bg-surface-100-950 p-10 space-y-2 text-center">
        <Fetch />
      </div>
    </Tabs.Content>
    <Tabs.Content value="generate">
      <div class="card mt-5 mb-5 mx-10 max-h-[95vh] overflow-y-auto bg-surface-100-950 p-10 space-y-2 text-center">
        <Generate />
      </div>
    </Tabs.Content>
  </Tabs>
