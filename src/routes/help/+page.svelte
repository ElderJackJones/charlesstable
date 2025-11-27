<script lang="ts">
    import { ChevronDown } from "@lucide/svelte";
    import { Accordion } from "@skeletonlabs/skeleton-svelte";
    import { slide } from "svelte/transition";
    import Export from "./Export.svelte";

    const items = [
        {
            id: '1',
            title: 'How to export data',
            component: Export
        }
    ]
</script>

<div class="flex flex-col px-8 w-full">
    <div class="card mt-5 h-15 preset-tonal-warning flex items-center justify-center">
        <p class="p-1">Restart the app before you continue. If that doesn't solve the problem, read on.</p>
    </div>

    <Accordion class="mt-5 overflow-hidden">
        {#each items as item, i (item)}
                {#if i !== 0}
                    <hr class="hr" />
                {/if}
                <Accordion.Item value={item.id}>
                    <h3>
                        <Accordion.ItemTrigger class="font-bold flex items-center justify-between gap-2">
                            {item.title}
                            <Accordion.ItemIndicator class="group">
                                <ChevronDown class="h-5 w-5 transition group-data-[state=open]:rotate-180" />
                            </Accordion.ItemIndicator>
                        </Accordion.ItemTrigger>
                    </h3>
                    <Accordion.ItemContent>
                        {#snippet element(attributes)}
                            {#if !attributes.hidden}
                                <div {...attributes} class="w-full" transition:slide={{ duration: 150 }}>
                                   <div class="w-full">
                                        <svelte:component this={item.component} />
                                   </div>
                                </div>
                            {/if}
                        {/snippet}
                    </Accordion.ItemContent>
                </Accordion.Item>
            {/each}
    </Accordion>
</div>