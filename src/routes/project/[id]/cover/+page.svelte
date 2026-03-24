<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { ROUTES } from '$lib/constants/routes';
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { toast } from '$lib/stores/toastStore';
  import { ipcGetCoverTemplates, ipcGetCoverConfig } from '$lib/ipc/cover';
  import type { CoverConfig, CoverTemplate } from '$lib/types/interfaces';
  import CoverEditor from '$lib/components/cover/CoverEditor.svelte';
  import CoverPreview from '$lib/components/cover/CoverPreview.svelte';
  import SpineCalculator from '$lib/components/cover/SpineCalculator.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
  const projectId = $derived($page.params.id ?? '');

  let coverConfig = $state<CoverConfig | null>(null);
  let templates = $state<CoverTemplate[]>([]);
  let loading = $state(true);
  let previewTrigger = $state(0);

  onMount(async () => {
    if (!projectId) return;
    try {
      const [loadedTemplates, loadedConfig] = await Promise.all([
        ipcGetCoverTemplates(),
        ipcGetCoverConfig(projectId),
      ]);
      templates = loadedTemplates;
      coverConfig = loadedConfig;
    } catch (e) {
      toast.error(t('errors.generic'));
    } finally {
      loading = false;
    }
  });

  function handleConfigSaved(saved: CoverConfig) {
    coverConfig = saved;
  }

  function handleSpineCalculated(mm: number) {
    if (coverConfig) {
      coverConfig = { ...coverConfig, spineWidthMm: mm };
    }
  }
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.cover')}</title>
</svelte:head>

{#if !project}
  <EmptyState
    icon="layout"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto(ROUTES.HOME)}
  />
{:else if loading}
  <div class="cover-page__loading" role="status" aria-label={t('common.loading')}>
    <span class="cover-page__spinner" aria-hidden="true"></span>
    <span>{t('common.loading')}</span>
  </div>
{:else}
  <div data-testid="cover-page" class="cover-page">

    <!-- Left panel: Editor + Spine Calculator -->
    <aside class="cover-page__left" data-testid="cover-left-panel">
      <div class="cover-page__editor-wrap">
        <CoverEditor
          {projectId}
          config={coverConfig}
          {templates}
          onSaved={handleConfigSaved}
          onPreviewRequest={() => { previewTrigger++; }}
        />
      </div>

      {#if coverConfig}
        <div class="cover-page__spine-wrap">
          <SpineCalculator
            {projectId}
            platform={coverConfig.platform}
            paperType={coverConfig.paperType}
            pageCount={coverConfig.pageCount}
            spineWidthMm={coverConfig.spineWidthMm}
            onSpineCalculated={handleSpineCalculated}
          />
        </div>
      {/if}
    </aside>

    <!-- Right panel: Preview -->
    <main class="cover-page__right" data-testid="cover-right-panel">
      <CoverPreview
        {projectId}
        config={coverConfig}
        {previewTrigger}
      />
    </main>

  </div>
{/if}

<style>
  .cover-page {
    display: grid;
    grid-template-columns: 320px 1fr;
    height: 100%;
    overflow: hidden;
  }

  .cover-page__left {
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--color-border);
    overflow: hidden;
  }

  .cover-page__editor-wrap {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .cover-page__spine-wrap {
    padding: var(--space-3);
    border-top: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .cover-page__right {
    overflow: hidden;
  }

  .cover-page__loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-3);
    height: 100%;
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }

  .cover-page__spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
