<script lang="ts">
  import { Send, LoaderCircle } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { afterUpdate } from "svelte";

  let messages: { id: number; sender: "user" | "bot"; text: string, loading?: boolean }[] = [
    { id: 0, sender: "bot", text: "Hello! How can I help you today?" }
  ];

  let userInput = "";
  let nextId = 1;
  let scroller: HTMLDivElement | null = null;

 async function sendMessage() {
  const text = userInput.trim();
  if (!text) return;

  // append user message
  messages = [...messages, { id: nextId++, sender: "user", text }];

  // clear the input immediately
  userInput = "";

  // create a placeholder bot message to stream into
  const botResponseId = nextId++;
  messages = [...messages, { id: botResponseId, sender: "bot", text: "", loading: true }];

  // listen for streamed chunks
  const unlistenChunk = await listen<string>(`chunk-${botResponseId}`, event => {
    const chunk = event.payload;
    const botMsg = messages.find(m => m.id === botResponseId);
    if (botMsg) {
      if (botMsg.loading) botMsg.loading = false;
      botMsg.text += chunk;
      messages = [...messages]; // trigger reactivity
    }
  });

  // listen for generation finish
  const unlistenFinish = await listen<void>(`finish-${botResponseId}`, () => {
    unlistenChunk();
    unlistenFinish();
  });

  // start backend generation
  invoke("generate", { prompt: text, model: "charlesJest:latest", id: botResponseId })
    .catch(err => console.error("Generate failed:", err));
  
}


  // keep scrolled to bottom when messages change
  afterUpdate(() => {
    if (scroller) scroller.scrollTop = scroller.scrollHeight;
  });
</script>

<div class="w-full h-screen flex flex-col">
  <div class="flex flex-col flex-grow m-4 shadow-xl rounded-2xl overflow-hidden bg-surface-50 dark:bg-surface-800">
    <div bind:this={scroller} class="flex flex-col flex-grow p-4 overflow-y-auto space-y-3 scrollbar-hide">
      {#each messages as msg (msg.id)}
                <div class="flex {msg.sender === 'user' ? 'justify-end' : 'justify-start'}">
            <div
              class="max-w-[75%] px-4 py-2 rounded-2xl text-sm shadow-md
              {msg.sender === 'user'
                ? 'bg-primary-500 text-white rounded-br-none'
                : 'bg-surface-200 dark:bg-surface-700 text-on-surface rounded-bl-none'} flex items-center gap-2">
              {#if msg.sender === 'bot' && msg.loading}
                <LoaderCircle class="w-4 h-4 animate-spin text-primary-500 dark:text-primary-300" />
              {/if}
              <span>{msg.text}</span>
            </div>
          </div>
      {/each}
    </div>

    <div class="flex items-center border-t border-surface-200 dark:border-surface-700 p-3 gap-2">
      <input
        type="text"
        placeholder="Type your message..."
        bind:value={userInput}
        on:keydown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            sendMessage();
          }
        }}
        class="flex-grow rounded-lg px-3 py-2 bg-surface-100 dark:bg-surface-900 text-on-surface outline-none border border-surface-300 dark:border-surface-600 focus:ring-2 focus:ring-primary-400"
      />
      <button
        on:click={sendMessage}
        disabled={userInput.trim() === ""}
        class="p-2 rounded-full bg-primary-500 text-white hover:bg-primary-600 shadow-md btn"
      >
        <Send class="w-5 h-5" />
      </button>
    </div>
  </div>
</div>
