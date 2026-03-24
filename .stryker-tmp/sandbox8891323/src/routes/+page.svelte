<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import ProjectCard from '$lib/components/project/ProjectCard.svelte';

  const projects = $derived($projectsStore.list);
  const loading = $derived($projectsStore.loading);
  const error = $derived($projectsStore.error);

  // Auto-open onboarding: redirect to /import on first launch with no projects
  $effect(() => {
    if (loading) return;
    if (projects.length === 0 && !error) {
      try {
        const firstLaunch = localStorage.getItem('bes_first_launch');
        if (!firstLaunch) {
          localStorage.setItem('bes_first_launch', 'done');
          goto('/import');
        }
      } catch {
        // localStorage unavailable — skip onboarding redirect
      }
    }
  });
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.dashboard')}</title>
</svelte:head>

<div data-testid="dashboard" class="dashboard">
  {#if loading}
    <div data-testid="dashboard-skeleton" class="dashboard__header">
      <h1 class="dashboard__title">{t('nav.dashboard')}</h1>
    </div>
    <div class="dashboard__projects dashboard__projects--skeleton" aria-label={t('common.loading')} aria-busy="true">
      {#each Array(6) as _}
        <div class="project-card-skeleton" aria-hidden="true">
          <div class="skeleton-pulse" style="height: 120px; border-radius: var(--radius-md);"></div>
          <div class="skeleton-pulse" style="height: 16px; width: 70%; margin-top: var(--space-2);"></div>
          <div class="skeleton-pulse" style="height: 12px; width: 45%; margin-top: var(--space-1);"></div>
        </div>
      {/each}
    </div>
  {:else if error}
    <div data-testid="dashboard-error" class="dashboard__error" role="alert">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <div>
        <p class="dashboard__error-title">{t('errors.generic')}</p>
        <p class="dashboard__error-detail">{error}</p>
      </div>
    </div>
  {:else if projects.length === 0}
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
      <a
        data-testid="dashboard-import-btn"
        href="/import"
        class="dashboard__import-btn"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <line x1="12" y1="5" x2="12" y2="19"/>
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        {t('emptyState.importCta')}
      </a>
    </div>

    <div data-testid="dashboard-projects" class="dashboard__projects">
      {#each projects as project (project.id)}
        <a
          data-testid="dashboard-project-link-{project.id}"
          href="/project/{project.id}"
          class="dashboard__project-link"
          aria-label={project.name}
        >
          <ProjectCard {project} />
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

  .dashboard__error {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    margin: var(--space-6);
    padding: var(--space-4);
    background: color-mix(in srgb, var(--color-error, #ef4444) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--color-error, #ef4444) 30%, transparent);
    border-radius: var(--radius-md);
    color: var(--color-error, #ef4444);
  }

  .dashboard__error-title {
    margin: 0;
    font-weight: 600;
    font-size: var(--text-sm);
  }

  .dashboard__error-detail {
    margin: var(--space-1) 0 0;
    font-size: var(--text-xs);
    opacity: 0.85;
  }

  .dashboard__header {
    padding: var(--space-6) var(--space-6) var(--space-4);
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .dashboard__title {
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  .dashboard__import-btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
    font-weight: 500;
    color: #fff;
    background: var(--color-primary);
    border-radius: var(--radius-md);
    text-decoration: none;
    transition: background var(--duration-fast);
    white-space: nowrap;
  }

  .dashboard__import-btn:hover {
    background: color-mix(in srgb, var(--color-primary) 85%, #000);
  }

  .dashboard__import-btn:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .dashboard__projects {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-4);
    padding: var(--space-6);
    align-content: start;
  }

  .dashboard__project-link {
    text-decoration: none;
    color: inherit;
    display: block;
    border-radius: var(--radius-lg);
    transition:
      box-shadow var(--duration-fast) ease,
      transform var(--duration-fast) ease;
  }

  .dashboard__project-link:hover {
    box-shadow: var(--shadow-md);
    transform: translateY(-1px);
  }

  .dashboard__project-link:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .dashboard__projects--skeleton {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-4);
    padding: var(--space-6);
  }

  .project-card-skeleton {
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .skeleton-pulse {
    background: var(--color-skeleton);
    border-radius: var(--radius-sm);
    animation: skeleton-pulse 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
