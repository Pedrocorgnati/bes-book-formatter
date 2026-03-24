<script lang="ts">
  import type { BookProject } from '$lib/types';
  import CompletenessBar from './CompletenessBar.svelte';

  interface Props {
    project: BookProject;
  }

  const { project }: Props = $props();

  const level = $derived(project.completenessLevel ?? null);
  const score = $derived(project.completenessScore ?? null);

  const genreBadge = $derived(project.genre ?? null);
</script>

<div data-testid="project-card-{project.id}" class="project-card">
  <div class="project-card__icon" aria-hidden="true">
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
      <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
      <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
    </svg>
  </div>

  <div class="project-card__body">
    <div class="project-card__header">
      <h2 data-testid="project-card-title" class="project-card__title">{project.name}</h2>
      {#if genreBadge}
        <span data-testid="project-card-genre" class="project-card__genre">{genreBadge}</span>
      {/if}
    </div>

    <p data-testid="project-card-meta" class="project-card__meta">{project.language}</p>

    {#if level != null}
      <div class="project-card__completeness">
        <CompletenessBar {level} {score} />
      </div>
    {/if}
  </div>
</div>

<style>
  .project-card {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    text-decoration: none;
    color: inherit;
    width: 100%;
  }

  .project-card__icon {
    flex-shrink: 0;
    color: var(--color-primary);
    margin-top: 2px;
  }

  .project-card__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .project-card__header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  .project-card__title {
    font-size: var(--text-base);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .project-card__genre {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-primary);
    background: color-mix(in srgb, var(--color-primary) 12%, transparent);
    padding: 1px var(--space-2);
    border-radius: var(--radius-full);
    white-space: nowrap;
  }

  .project-card__meta {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
  }

  .project-card__completeness {
    margin-top: var(--space-2);
  }
</style>
