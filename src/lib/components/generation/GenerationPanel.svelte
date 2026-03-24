<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { generationStore, GenerationStatus } from '$lib/stores/generationStore';
  import { OutputFormat } from '$lib/types/enums';
  import { toastStore } from '$lib/stores/toastStore';
  import {
    ipcRunPreflight,
    ipcGenerateEpub,
    ipcGeneratePdfPrint,
    ipcGeneratePdfEbook,
    ipcGenerateDocx,
    ipcGetGenerationResults,
  } from '$lib/ipc/generation';
  import FormatSelector from './FormatSelector.svelte';
  import PreGenerationChecklist from './PreGenerationChecklist.svelte';
  import GenerationProgress from './GenerationProgress.svelte';
  import GenerationResults from './GenerationResults.svelte';
  import type { FormatSelection } from '$lib/types';

  interface Props {
    projectId: string;
  }

  let { projectId }: Props = $props();

  const genState = $derived.by(() => $generationStore);

  let currentSelection = $state<FormatSelection | null>(null);
  let showNewGeneration = $state(false);
  let preflightLoading = $state(false);

  async function handleSelectionChange(selection: FormatSelection) {
    currentSelection = selection;
    generationStore.reset();
  }

  async function runPreflight() {
    if (!currentSelection) return;
    preflightLoading = true;
    generationStore.startPreflight(currentSelection.formats[0] ?? OutputFormat.EPUB3, currentSelection.platform);
    try {
      const result = await ipcRunPreflight(projectId, currentSelection.formats[0]);
      generationStore.setPreflight(result);
    } catch (e) {
      generationStore.setError(String(e));
      toastStore.error(`${t('generation.preflightError')}: ${e}`);
    } finally {
      preflightLoading = false;
    }
  }

  async function startGeneration() {
    if (!currentSelection || !genState.preflight?.passed) return;
    generationStore.startGeneration();

    const { formats, platform } = currentSelection;
    let lastResult = null;
    let anyError = false;

    for (const format of formats) {
      try {
        let result;
        if (format === OutputFormat.EPUB3) {
          result = await ipcGenerateEpub(projectId, platform);
        } else if (format === OutputFormat.PDF_PRINT) {
          result = await ipcGeneratePdfPrint(projectId, platform);
        } else if (format === OutputFormat.PDF_EBOOK) {
          result = await ipcGeneratePdfEbook(projectId, platform);
        } else if (format === OutputFormat.DOCX) {
          result = await ipcGenerateDocx(projectId, platform);
        } else {
          // Formatos simples delegados ao DocxService/SimpleExportService — skip se não mapeado
          continue;
        }
        lastResult = result;
        if (!result.success) anyError = true;
      } catch (e) {
        anyError = true;
        toastStore.error(`${t('generation.generateFormatError')} ${format}: ${e}`);
      }
    }

    if (lastResult) {
      generationStore.setResult(lastResult);
      if (!anyError) {
        toastStore.success(t('generation.done'));
      }
    } else if (anyError) {
      generationStore.setError(t('generation.partialError'));
    }

    // Refresh history
    try {
      const history = await ipcGetGenerationResults(projectId);
      generationStore.setHistory(history);
    } catch (err) {
      console.error('[GenerationPanel] history refresh error:', err instanceof Error ? err.message : String(err));
      toastStore.error(t('generation.historyLoadError'));
    } finally {
      showNewGeneration = false;
    }
  }

  function handleRegenerate(format: string, platform: string) {
    showNewGeneration = true;
    // Could pre-select the format here
  }

  function handleCancel() {
    generationStore.reset();
  }

  // Load history on mount
  $effect(() => {
    generationStore.setHistoryLoading(true);
    ipcGetGenerationResults(projectId)
      .then((h) => generationStore.setHistory(h))
      .catch((err) => {
        generationStore.setHistoryLoading(false);
        toastStore.error(t('generation.historyLoadError'));
      });
  });
</script>

<div data-testid="generation-panel" class="gen-panel">
  <!-- Header -->
  <div class="gen-panel__header">
    <h1 class="gen-panel__title">{t('generation.title')}</h1>
    {#if genState.status === GenerationStatus.IDLE || genState.status === GenerationStatus.DONE || genState.status === GenerationStatus.ERROR}
      {#if !showNewGeneration}
        <button class="btn btn--primary btn--sm" onclick={() => (showNewGeneration = true)}>
          {t('generation.newGeneration')}
        </button>
      {:else}
        <button class="btn btn--ghost btn--sm" onclick={() => { showNewGeneration = false; generationStore.reset(); }}>
          {t('generation.cancelFlow')}
        </button>
      {/if}
    {/if}
  </div>

  <!-- Generation flow -->
  {#if genState.status === GenerationStatus.GENERATING}
    <div class="gen-panel__section">
      <GenerationProgress {projectId} onCancel={handleCancel} />
    </div>
  {:else if showNewGeneration}
    <div class="gen-panel__section">
      <h2 class="gen-panel__section-title">{t('generation.selectFormat')}</h2>
      <FormatSelector onChange={handleSelectionChange} />
    </div>

    {#if currentSelection}
      <div class="gen-panel__section">
        <h2 class="gen-panel__section-title">{t('generation.preflight')}</h2>
        <PreGenerationChecklist
          result={genState.preflight}
          loading={preflightLoading}
          onGenerate={startGeneration}
          onRecheck={runPreflight}
        />
      </div>

      {#if genState.preflight === null}
        <div class="gen-panel__preflight-cta">
          <button class="btn btn--secondary" onclick={runPreflight} disabled={preflightLoading}>
            {t('generation.btnRunPreflight')}
          </button>
        </div>
      {/if}
    {/if}
  {/if}

  <!-- Error banner -->
  {#if genState.status === GenerationStatus.ERROR && genState.error}
    <div class="gen-panel__error" role="alert">
      ❌ {genState.error}
    </div>
  {/if}

  <!-- History -->
  <div class="gen-panel__section">
    <GenerationResults
      {projectId}
      history={genState.history}
      latestResult={genState.lastResult}
      loading={genState.historyLoading}
      onRegenerate={handleRegenerate}
    />
  </div>
</div>

<style>
  .gen-panel {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-6);
    max-width: 1000px;
    margin: 0 auto;
  }

  .gen-panel__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-4);
  }

  .gen-panel__title {
    font-size: var(--text-2xl);
    font-weight: 700;
    margin: 0;
  }

  .gen-panel__section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
  }

  .gen-panel__section-title {
    font-size: var(--text-base);
    font-weight: 600;
    margin: 0;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-size: var(--text-xs);
  }

  .gen-panel__preflight-cta {
    display: flex;
    justify-content: flex-start;
    padding-left: var(--space-5);
  }

  .gen-panel__error {
    padding: var(--space-3) var(--space-4);
    background: var(--color-error-subtle);
    color: var(--color-error);
    border: 1px solid var(--color-error);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
  }
</style>
