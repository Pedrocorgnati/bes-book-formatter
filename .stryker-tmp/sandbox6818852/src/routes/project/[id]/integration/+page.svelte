<script lang="ts">
  import { page } from '$app/stores';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { toastStore } from '$lib/stores/toastStore';
  import BesStatusPanel from '$lib/components/bes/BesStatusPanel.svelte';
  import EditorialProgressBar from '$lib/components/bes/EditorialProgressBar.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const projectId = $derived($page.params.id ?? '');
  const currentProject = $derived($projectsStore.current);
  const workspacePath = $derived(currentProject?.besRootPath ?? '');
  const projectName = $derived(currentProject?.name ?? '');
  const hasWorkspace = $derived(!!workspacePath);
</script>

<svelte:head>
  <title>{currentProject?.name ?? t('nav.integration')} — {t('nav.integration')} | BES Book Formatter</title>
</svelte:head>

<div data-testid="integration-page" class="integration-page">
  <header class="integration-header">
    <h2 class="integration-title">{t('nav.integration')}</h2>
    <p class="integration-desc">{t('integration.description')}</p>
  </header>

  {#if !hasWorkspace}
    <EmptyState
      icon="package"
      title={t('integration.emptyTitle')}
      description={t('integration.emptyDesc')}
    />
  {:else}
    <div class="integration-panels">
      <section class="integration-section" aria-labelledby="bes-status-heading">
        <h3 id="bes-status-heading" class="section-heading">{t('bes.statusPanel.title')}</h3>
        <BesStatusPanel {projectId} workspacePath={workspacePath ?? ''} />
      </section>

      <section class="integration-section" aria-labelledby="editorial-progress-heading">
        <h3 id="editorial-progress-heading" class="section-heading">{t('bes.progress.title')}</h3>
        <EditorialProgressBar {projectId} workspacePath={workspacePath ?? ''} projectName={projectName ?? ''} />
      </section>
    </div>
  {/if}
</div>

<style>
  .integration-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    padding: var(--space-6);
    max-width: 64rem;
    margin: 0 auto;
    width: 100%;
  }

  .integration-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .integration-title {
    margin: 0;
    font-size: var(--text-xl);
    font-weight: 600;
    color: var(--color-text);
  }

  .integration-desc {
    margin: 0;
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .integration-panels {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .integration-section {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-6);
  }

  .section-heading {
    margin: 0 0 var(--space-4);
    font-size: var(--text-lg);
    font-weight: 600;
    color: var(--color-text);
  }
</style>
