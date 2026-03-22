<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const projects = $derived($projectsStore.list);
  // RESOLVED: BookProject.title/author come from BookConfig, using .name and .language as metadata
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.dashboard')}</title>
</svelte:head>

<div data-testid="dashboard" class="dashboard">
  {#if projects.length === 0}
    <EmptyState
      icon="book"
      title={t('emptyState.noProjects')}
      description={t('emptyState.noProjectsDesc')}
      ctaLabel={t('emptyState.importCta')}
      onCta={() => goto('/import')}
    />
  {:else}
    <div data-testid="dashboard-header" class="dashboard__header">
      <h1 data-testid="dashboard-title" class="dashboard__title">{t('nav.dashboard')}</h1>
    </div>

    <div data-testid="dashboard-projects" class="dashboard__projects">
      {#each projects as project (project.id)}
        <a
          data-testid="dashboard-project-card-{project.id}"
          href="/project/{project.id}"
          class="project-card"
          aria-label={project.name}
        >
          <div class="project-card__icon" aria-hidden="true">
            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
              <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
            </svg>
          </div>
          <div class="project-card__info">
            <h2 class="project-card__title">{project.name}</h2>
            <p class="project-card__meta">{project.language}</p>
          </div>
        </a>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dashboard {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .dashboard__header {
    padding: var(--space-6) var(--space-6) var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .dashboard__title {
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  .dashboard__projects {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-4);
    padding: var(--space-6);
  }

  .project-card {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-decoration: none;
    color: inherit;
    transition:
      background-color var(--duration-fast) ease,
      border-color var(--duration-fast) ease,
      box-shadow var(--duration-fast) ease;
  }

  .project-card:hover {
    background: var(--color-bg-secondary);
    border-color: var(--color-primary);
    box-shadow: var(--shadow-md);
  }

  .project-card:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .project-card__icon {
    flex-shrink: 0;
    color: var(--color-primary);
  }

  .project-card__info {
    min-width: 0;
  }

  .project-card__title {
    font-size: var(--text-base);
    font-weight: 600;
    color: var(--color-text);
    margin: 0 0 var(--space-1);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .project-card__meta {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
