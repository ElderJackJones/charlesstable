<script lang="ts">
  import { ArrowLeftIcon, ArrowRightIcon } from '@lucide/svelte';
  import Authenticate from './Authenticate.svelte';
  
  // Source Data
  const steps = [
    { label: 'Authentication', content: Authenticate},
    { label: 'Step 2', content: Authenticate },
    { label: 'Step 3', content: Authenticate },
    { label: 'Step 4', content: Authenticate },
    { label: 'Step 5', content: Authenticate },
  ];

  // Reactive
  let currentStep = $state(0);
  const isFirstStep = $derived(currentStep === 0);
  const isLastStep = $derived(currentStep === steps.length - 1);

  /** Determine if on the current step. */
  function isCurrentStep(index: number) {
    return currentStep === index;
  }

  /** Jump to a particular step. */
  function setStep(index: number) {
    currentStep = index;
  }

  /** Progress to the previous step. */
  function prevStep() {
    currentStep--;
  }

  /** Progress to the next step. */
  function nextStep() {
    currentStep++;
  }
</script>

<div class="w-full p-8">
  <!-- Stepper -->
  <div class="space-y-8">
    <!-- Timeline -->
    <div class="relative">
      <!-- Numerals -->
      <div class="flex justify-between items-center gap-4">
        {#each steps as step, i (step)}
          <!-- Numeral Button -->
          <button
            class="btn-icon btn-icon-sm rounded-full {isCurrentStep(i) ? 'preset-filled-primary-500' : 'preset-filled-surface-200-800'}"
            onclick={() => setStep(i)}
            title="Go to {step.label}"
            aria-label="Go to {step.label}"
          >
            <span class="font-bold">{i + 1}</span>
          </button>
        {/each}
      </div>
      <!-- Line -->
      <hr class="hr !border-surface-200-800 absolute top-[50%] left-0 right-0 z-[-1]" />
    </div>
    <!-- Loop all steps -->
    {#each steps as step, i (step)}
      <!-- Filter to current step only -->
      {#if isCurrentStep(i)}
        {@const Step = steps[currentStep].content}

        <!-- Individual steps -->
        <div class="card max-h-[75vh] overflow-y-auto bg-surface-100-900 p-10 space-y-2 text-center">
            <Step />
        </div>
      {/if}
    {/each}
    <!-- Navigation -->
    <nav class="flex justify-between items-center gap-4">
      <!-- Back Button -->
      <button type="button" class="btn preset-tonal hover:preset-filled" onclick={prevStep} disabled={isFirstStep}>
        <ArrowLeftIcon size={18} />
        <span>Previous</span>
      </button>
      <!-- Next Button -->
      <button type="button" class="btn preset-tonal hover:preset-filled" onclick={nextStep} disabled={isLastStep}>
        <span>Next</span>
        <ArrowRightIcon size={18} />
      </button>
    </nav>
  </div>
</div>