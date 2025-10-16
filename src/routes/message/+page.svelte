<script lang="ts">
  import { Send } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { afterUpdate } from "svelte";

  let messages: { 
    id: number; 
    sender: "user" | "bot"; 
    text: string; 
    loading?: boolean;
    options?: { label: string; value: string }[];
  }[] = [
    { id: 0, sender: "bot", text: "Hello! How can I help you today?" }
  ];

  let userInput = "";
  let nextId = 1;
  let scroller: HTMLDivElement | null = null;

  async function sendMessage() {
    const text = userInput.trim();
    if (!text) return;

    messages = [...messages, { id: nextId++, sender: "user", text }];
    userInput = "";

    const botResponseId = nextId++;
    messages = [...messages, { id: botResponseId, sender: "bot", text: "", loading: true }];

    const unlistenChunk = await listen<string>(`chunk-${botResponseId}`, event => {
      const chunk = event.payload;
      const botMsg = messages.find(m => m.id === botResponseId);
      if (botMsg) {
        botMsg.loading = false;
        botMsg.text += chunk;
        messages = [...messages];
      }
    });

    const unlistenFinish = await listen<void>(`finish-${botResponseId}`, () => {
      unlistenChunk();
      unlistenFinish();
    });

    invoke("generate", { prompt: text, model: "charlesJest:latest", id: botResponseId })
      .catch(err => console.error("Generate failed:", err));
  }

  afterUpdate(() => {
    if (scroller) scroller.scrollTop = scroller.scrollHeight;
  });
</script>

<div class="w-full h-screen flex flex-col">
  <div class="flex flex-col flex-grow m-4 shadow-xl rounded-2xl overflow-hidden bg-surface-50 dark:bg-surface-800">
    <div bind:this={scroller} class="flex flex-col flex-grow p-4 overflow-y-auto space-y-3 scrollbar-hide">
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
