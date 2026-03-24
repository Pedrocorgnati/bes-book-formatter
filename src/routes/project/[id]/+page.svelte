<script lang="ts">
  import { goto } from '$app/navigation';
  import { ROUTES } from '$lib/constants/routes';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
  // RESOLVED: BookProject.title comes from BookConfig, using .name as fallback
</script>

<svelte:head>
  <title>BES Book Formatter — {project?.name ?? t('nav.editor')}</title>
</svelte:head>

{#if !project}
  <EmptyState
    icon="folder"
    title={t('emptyState.noProjectSelected')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto(ROUTES.HOME)}
  />
{:else}
  <!-- Rock-2: Editor principal do projeto -->
  <!-- TODO: Implementar backend — conteúdo do editor (módulos: importação, tipografia, etc.) -->
  <div data-testid="editor-placeholder" class="editor-placeholder">
    <div class="editor-placeholder__info">
      <h1 data-testid="editor-project-title" class="editor-placeholder__title">{project.name}</h1>
      <p class="editor-placeholder__author">{project.language}</p>
    </div>
    <p class="editor-placeholder__note">
      {t('common.comingSoon')}
    </p>
  </div>
{/if}

<style>
  .editor-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-12) var(--space-8);
    gap: var(--space-4);
    text-align: center;
  }

  .editor-placeholder__info {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .editor-placeholder__title {
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  .editor-placeholder__author {
    font-size: var(--text-base);
    color: var(--color-text-secondary);
    margin: 0;
  }

  .editor-placeholder__note {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
    padding: var(--space-2) var(--space-4);
    background: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    font-family: var(--font-mono);
  }
</style>
