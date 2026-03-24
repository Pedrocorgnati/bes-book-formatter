<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { t } from '$lib/i18n/engine';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import IllustrationGallery from '$lib/components/illustrations/IllustrationGallery.svelte';

  // Project ID comes from the URL parameter
  const projectId = $derived($page.params.id ?? '');
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.illustrations')}</title>
</svelte:head>

{#if !projectId}
  <EmptyState
    icon="image"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <div class="illustrations-page">
    <header class="page-header">
      <div>
        <h1 class="page-title">{t('nav.illustrations')}</h1>
        <p class="page-subtitle">{t('illustrations.pageSubtitle')}</p>
      </div>
    </header>

    <!-- IllustrationGallery owns the dropzone modal + alt-text modal internally -->
    <IllustrationGallery {projectId} />
  </div>
{/if}

<style>
  .illustrations-page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.5rem;
    height: 100%;
    overflow-y: auto;
  }

  .page-header {
    flex-shrink: 0;
  }

  .page-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text, #111827);
    margin: 0 0 0.25rem;
  }

  .page-subtitle {
    font-size: 0.875rem;
    color: var(--color-text-secondary, #6b7280);
    margin: 0;
  }
</style>
