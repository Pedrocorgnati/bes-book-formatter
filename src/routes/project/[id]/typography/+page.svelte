<script lang="ts">
  import { goto } from '$app/navigation';
  import { ROUTES } from '$lib/constants/routes';
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { typographyStore, typographyLoadingStore } from '$lib/stores/typography';
  import { toast } from '$lib/stores/toastStore';
  import { ipcGetTypographyConfig } from '$lib/ipc/typography';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import TypographyPanel from '$lib/components/typography/TypographyPanel.svelte';
  import TypographyPreview from '$lib/components/typography/TypographyPreview.svelte';
  import FontUploader from '$lib/components/typography/FontUploader.svelte';
  import FontCatalog from '$lib/components/typography/FontCatalog.svelte';
  import PageConfigPanel from '$lib/components/typography/PageConfigPanel.svelte';
  import type { FontInfo } from '$lib/types/interfaces';

  let fontCatalog = $state<FontCatalog | undefined>(undefined);

  function handleFontAdded(font: FontInfo) {
    fontCatalog?.refresh();
  }

  const project = $derived($projectsStore.current);
  const projectId = $derived(project?.id ?? '');

  onMount(async () => {
    if (!project?.id) return;
    typographyLoadingStore.set(true);
    try {
      const config = await ipcGetTypographyConfig(project.id);
      typographyStore.set(config);
    } catch (err) {
      toast.error(t('typography.loadError'));
    } finally {
      typographyLoadingStore.set(false);
    }
  });
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.typography')}</title>
</svelte:head>

{#if !project}
  <EmptyState
    icon="type"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto(ROUTES.HOME)}
  />
{:else}
  <div class="typography-page">
    <header class="page-header">
      <h1 class="page-title">{t('nav.typography')}</h1>
      <p class="page-subtitle">{t('typography.pageSubtitle')}</p>
    </header>

    <div class="page-layout">
      <!-- Sidebar: Configurações -->
      <aside class="page-sidebar">
        <TypographyPanel {projectId} />
        <div class="sidebar-section">
          <h2 class="section-title">{t('typography.pageFormat')}</h2>
          <PageConfigPanel {projectId} />
        </div>
        <div class="sidebar-section">
          <h2 class="section-title">{t('typography.customFonts')}</h2>
          <FontUploader {projectId} onFontAdded={handleFontAdded} />
          <FontCatalog bind:this={fontCatalog} {projectId} />
        </div>
      </aside>

      <!-- Main: Preview -->
      <main class="page-main">
        <div class="preview-section">
          <h2 class="section-title">{t('typography.preview')}</h2>
          <TypographyPreview />
        </div>
      </main>
    </div>
  </div>
{/if}

<style>
  .typography-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: var(--space-6);
    padding: var(--space-6);
    overflow: hidden;
  }

  .page-header {
    flex-shrink: 0;
  }

  .page-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text, #111827);
    margin: 0 0 var(--space-1);
  }

  .page-subtitle {
    font-size: var(--text-sm);
    color: var(--color-text-secondary, #6b7280);
    margin: 0;
  }

  .page-layout {
    display: grid;
    grid-template-columns: 340px 1fr;
    gap: var(--space-6);
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .page-sidebar {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    overflow-y: auto;
    padding-right: var(--space-1);
  }

  .page-main {
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .sidebar-section,
  .preview-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .section-title {
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--color-text-secondary, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0;
  }

  @media (max-width: 900px) {
    .typography-page {
      overflow-y: auto;
    }

    .page-layout {
      grid-template-columns: 1fr;
      overflow: visible;
    }

    .page-sidebar,
    .page-main {
      overflow: visible;
    }
  }
</style>
