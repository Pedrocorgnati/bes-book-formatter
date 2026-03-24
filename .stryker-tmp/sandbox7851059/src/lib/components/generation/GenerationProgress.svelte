<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { ipcCancelGeneration } from '$lib/ipc/generation';

  interface ProgressEvent {
    percent: number;
    step: string;
    message: string;
  }

  interface Props {
    projectId: string;
    onCancel?: () => void;
  }

  let { projectId, onCancel }: Props = $props();

  let percent = $state(0);
  let currentStep = $state('');
  let logLines = $state<string[]>([]);
  let logExpanded = $state(false);
  let cancelling = $state(false);

  const STEPS = [
    { key: 'prepare', labelKey: 'generation.steps.preparation', threshold: 10 },
    { key: 'generate', labelKey: 'generation.steps.generation', threshold: 60 },
    { key: 'postprocess', labelKey: 'generation.steps.postProcessing', threshold: 90 },
    { key: 'finalize', labelKey: 'generation.steps.finalization', threshold: 100 },
  ];

  const activeStepIndex = $derived(
    STEPS.findIndex((s) => percent < s.threshold) === -1 ? STEPS.length - 1 : STEPS.findIndex((s) => percent < s.threshold)
  );

  let unlisten: UnlistenFn | null = null;

  onMount(async () => {
    unlisten = await listen<ProgressEvent>('generation:progress', (event) => {
      percent = event.payload.percent;
      currentStep = event.payload.step;
      logLines = [...logLines, event.payload.message];
    });
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function handleCancel() {
    if (cancelling) return;
    cancelling = true;
    try {
      await ipcCancelGeneration(projectId);
    } finally {
      onCancel?.();
    }
  }
</script>

<div data-testid="generation-progress" class="gen-progress" role="status" aria-label={t('generation.generating')}>
  <!-- Step indicators -->
  <div class="gen-progress__steps" aria-label="Etapas de geração">
    {#each STEPS as step, i (step.key)}
      <div
        class="gen-progress__step"
        class:gen-progress__step--done={i < activeStepIndex}
        class:gen-progress__step--active={i === activeStepIndex}
        class:gen-progress__step--pending={i > activeStepIndex}
      >
        <div class="gen-progress__step-dot" aria-hidden="true">
          {#if i < activeStepIndex}✓{:else}{i + 1}{/if}
        </div>
        <span class="gen-progress__step-label">{t(step.labelKey)}</span>
      </div>
    {/each}
  </div>

  <!-- Progress bar -->
  <div class="gen-progress__bar-wrap" role="progressbar" aria-valuenow={percent} aria-valuemin={0} aria-valuemax={100}>
    <div class="gen-progress__bar" style="width: {percent}%"></div>
  </div>
  <div class="gen-progress__percent">{percent}%</div>

  <!-- Current step message -->
  {#if currentStep}
    <p class="gen-progress__step-msg">{currentStep}</p>
  {/if}

  <!-- Log terminal -->
  {#if logLines.length > 0}
    <div class="gen-progress__log-wrap">
      <button
        class="gen-progress__log-toggle"
        onclick={() => (logExpanded = !logExpanded)}
        aria-expanded={logExpanded}
      >
        {t('generation.logLabel')} {logExpanded ? '▲' : '▼'}
      </button>
      {#if logExpanded}
        <pre class="gen-progress__log" role="log" aria-label="Log de geração">{logLines.join('\n')}</pre>
      {/if}
    </div>
  {/if}

  <!-- Cancel button -->
  <div class="gen-progress__actions">
    <button
      class="btn btn--ghost btn--sm"
      disabled={cancelling}
      aria-disabled={cancelling}
      onclick={handleCancel}
    >
      {cancelling ? t('generation.cancelling') : t('generation.btnCancel')}
    </button>
  </div>
</div>

<style>
  .gen-progress {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .gen-progress__steps {
    display: flex;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .gen-progress__step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-1);
    flex: 1;
    font-size: var(--text-xs);
  }

  .gen-progress__step-dot {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--text-xs);
    font-weight: 600;
    border: 2px solid var(--color-border);
    background: var(--color-surface);
    color: var(--color-text-muted);
  }

  .gen-progress__step--done .gen-progress__step-dot {
    background: var(--color-success);
    border-color: var(--color-success);
    color: white;
  }

  .gen-progress__step--active .gen-progress__step-dot {
    background: var(--color-accent);
    border-color: var(--color-accent);
    color: white;
  }

  .gen-progress__step-label {
    color: var(--color-text-muted);
    text-align: center;
  }

  .gen-progress__step--active .gen-progress__step-label {
    color: var(--color-accent);
    font-weight: 500;
  }

  .gen-progress__bar-wrap {
    height: 8px;
    background: var(--color-border);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .gen-progress__bar {
    height: 100%;
    background: var(--color-accent);
    border-radius: var(--radius-full);
    transition: width 0.3s ease;
  }

  .gen-progress__percent {
    text-align: right;
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-accent);
  }

  .gen-progress__step-msg {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
  }

  .gen-progress__log-toggle {
    background: none;
    border: none;
    cursor: pointer;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    padding: 0;
  }

  .gen-progress__log {
    background: var(--color-surface-raised);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    padding: var(--space-3);
    font-size: var(--text-xs);
    font-family: var(--font-code);
    max-height: 200px;
    overflow-y: auto;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .gen-progress__actions {
    display: flex;
    justify-content: flex-end;
  }
</style>
