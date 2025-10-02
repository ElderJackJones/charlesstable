<script lang="ts">
  import { Send } from "@lucide/svelte";

  let messages: { sender: "user" | "bot"; text: string }[] = [
    { sender: "bot", text: "Hello! How can I help you today?" }
  ];

  let userInput = "";

  function sendMessage() {
    if (userInput.trim() === "") return;
    messages = [...messages, { sender: "user", text: userInput }];
    // Placeholder bot reply
    setTimeout(() => {
      messages = [...messages, { sender: "bot", text: `You said: ${userInput}` }];
    }, 500);
    userInput = "";
  }
</script>

<div class="w-full h-screen flex flex-col">
  <!-- Chat container -->
  <div class="flex flex-col flex-grow m-4 shadow-xl rounded-2xl overflow-hidden bg-surface-50 dark:bg-surface-800">
    <div class="flex flex-col flex-grow p-4 overflow-y-auto space-y-3">
      {#each messages as msg}
        <div class="flex {msg.sender === 'user' ? 'justify-end' : 'justify-start'}">
          <div class="max-w-[75%] px-4 py-2 rounded-2xl text-sm shadow-md
            {msg.sender === 'user'
              ? 'bg-primary-500 text-white rounded-br-none'
              : 'bg-surface-200 dark:bg-surface-700 text-on-surface rounded-bl-none'}">
            {msg.text}
          </div>
        </div>
      {/each}
    </div>

    <!-- Input bar -->
    <div class="flex items-center border-t border-surface-200 dark:border-surface-700 p-3 gap-2">
      <input
        type="text"
        placeholder="Type your message..."
        bind:value={userInput}
        on:keydown={(e) => e.key === 'Enter' && sendMessage()}
        class="flex-grow rounded-lg px-3 py-2 bg-surface-100 dark:bg-surface-900 text-on-surface outline-none border border-surface-300 dark:border-surface-600 focus:ring-2 focus:ring-primary-400"
      />
      <button
        on:click={sendMessage}
        class="p-2 rounded-full bg-primary-500 text-white hover:bg-primary-600 shadow-md btn"
      >
        <Send class="w-5 h-5" />
      </button>
    </div>
  </div>
</div>

<style>
  /* Make chat scroll nice */
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>