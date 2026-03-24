<script lang="ts">
  import type { CoverTemplate } from '$lib/types/interfaces';

  interface Props {
    templates: CoverTemplate[];
    selectedId: string;
    onSelect: (template: CoverTemplate) => void;
  }

  let { templates, selectedId, onSelect }: Props = $props();

  const genres = $derived([
    'all',
    ...Array.from(new Set(templates.map(t => t.genre))),
  ]);
  let activeGenre = $state('all');

  const filtered = $derived(
    activeGenre === 'all' ? templates : templates.filter(t => t.genre === activeGenre)
  );

  const genreLabel: Record<string, string> = {
    all: 'Todos',
    fiction: 'Ficção',
    'non-fiction': 'Não-Ficção',
    technical: 'Técnico',
    children: 'Infantil',
    poetry: 'Poesia',
    academic: 'Acadêmico',
  };
</script>

<div data-testid="cover-template-gallery" class="gallery">
  <!-- Genre filter chips -->
  <div class="gallery__filters" role="tablist" aria-label="Filtrar por gênero">
    {#each genres as genre}
      <button
        role="tab"
        data-testid="genre-filter-{genre}"
        class="gallery__chip"
        class:gallery__chip--active={activeGenre === genre}
        aria-selected={activeGenre === genre}
        onclick={() => (activeGenre = genre)}
      >
        {genreLabel[genre] ?? genre}
      </button>
    {/each}
  </div>

  <!-- Template grid -->
  <div class="gallery__grid" role="tabpanel">
    {#if templates.length === 0}
      {#each Array(6) as _}
        <div class="gallery__skeleton" aria-hidden="true">
          <div class="gallery__skeleton-preview"></div>
          <div class="gallery__skeleton-text"></div>
        </div>
      {/each}
    {/if}
    {#each filtered as tpl (tpl.id)}
      <button
        data-testid="template-card-{tpl.id}"
        class="gallery__card"
        class:gallery__card--selected={tpl.id === selectedId}
        onclick={() => onSelect(tpl)}
        aria-pressed={tpl.id === selectedId}
        title={tpl.description}
      >
        <!-- Mini cover preview using template colors -->
        <div
          class="gallery__preview"
          style="background: {tpl.primaryColor};"
          aria-hidden="true"
        >
          <div class="gallery__preview-spine" style="background: color-mix(in srgb, {tpl.primaryColor} 80%, black);"></div>
          <div class="gallery__preview-text">
            <span class="gallery__preview-title" style="color: {tpl.secondaryColor};">T</span>
          </div>
        </div>

        <div class="gallery__card-info">
          <span class="gallery__card-name">{tpl.name}</span>
          <span class="gallery__card-genre">{genreLabel[tpl.genre] ?? tpl.genre}</span>
        </div>

        {#if tpl.id === selectedId}
          <div class="gallery__card-check" aria-hidden="true">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
          </div>
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .gallery {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .gallery__filters {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .gallery__chip {
    padding: var(--space-1) var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: 999px;
    background: transparent;
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all var(--duration-fast) ease;
  }

  .gallery__chip:hover {
    border-color: var(--color-primary);
    color: var(--color-primary);
  }

  .gallery__chip--active {
    background: var(--color-primary);
    border-color: var(--color-primary);
    color: white;
  }

  .gallery__grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-2);
  }

  .gallery__card {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-1);
    border: 2px solid transparent;
    border-radius: var(--radius-md);
    background: var(--color-surface);
    cursor: pointer;
    transition: border-color var(--duration-fast) ease;
    text-align: left;
  }

  .gallery__card:hover {
    border-color: var(--color-border);
  }

  .gallery__card--selected {
    border-color: var(--color-primary);
  }

  .gallery__preview {
    position: relative;
    aspect-ratio: 6/9;
    border-radius: var(--radius-sm);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }

  .gallery__preview-spine {
    width: 12%;
    height: 100%;
    flex-shrink: 0;
  }

  .gallery__preview-text {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .gallery__preview-title {
    font-size: 2rem;
    font-weight: 700;
    opacity: 0.9;
  }

  .gallery__card-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    padding: 0 var(--space-1);
  }

  .gallery__card-name {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .gallery__card-genre {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .gallery__skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-md);
  }

  .gallery__skeleton-preview {
    aspect-ratio: 6/9;
    border-radius: var(--radius-sm);
    background: var(--color-border);
    animation: pulse 1.5s ease-in-out infinite;
  }

  .gallery__skeleton-text {
    height: 14px;
    width: 70%;
    border-radius: var(--radius-sm);
    background: var(--color-border);
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }

  .gallery__card-check {
    position: absolute;
    top: var(--space-1);
    right: var(--space-1);
    width: 18px;
    height: 18px;
    border-radius: 50%;
    background: var(--color-primary);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
