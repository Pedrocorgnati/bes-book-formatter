<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    icon?: 'book' | 'folder' | 'image' | 'package' | 'eye' | 'layout' | 'type' | 'warning' | 'store';
    title: string;
    description?: string;
    ctaLabel?: string;
    ctaHref?: string;
    onCta?: () => void;
    size?: 'sm' | 'md' | 'lg';
  }

  let {
    icon = 'book',
    title,
    description,
    ctaLabel,
    ctaHref,
    onCta,
    size = 'md'
  }: Props = $props();
</script>

<div data-testid="empty-state" class="empty-state empty-state--{size}" role="status">
  <!-- Ícone SVG por tipo -->
  <div class="empty-state__icon" aria-hidden="true">
    {#if icon === 'book'}
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
      </svg>
    {:else if icon === 'folder'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
      </svg>
    {:else if icon === 'image'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
        <circle cx="8.5" cy="8.5" r="1.5"/>
        <polyline points="21 15 16 10 5 21"/>
      </svg>
    {:else if icon === 'package'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <line x1="16.5" y1="9.4" x2="7.5" y2="4.21"/>
        <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
        <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
        <line x1="12" y1="22.08" x2="12" y2="12"/>
      </svg>
    {:else if icon === 'eye'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
        <circle cx="12" cy="12" r="3"/>
      </svg>
    {:else if icon === 'layout'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
        <line x1="9" y1="3" x2="9" y2="21"/>
      </svg>
    {:else if icon === 'type'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <polyline points="4 7 4 4 20 4 20 7"/>
        <line x1="9" y1="20" x2="15" y2="20"/>
        <line x1="12" y1="4" x2="12" y2="20"/>
      </svg>
    {:else if icon === 'warning'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
        <line x1="12" y1="9" x2="12" y2="13"/>
        <line x1="12" y1="17" x2="12.01" y2="17"/>
      </svg>
    {:else if icon === 'store'}
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/>
        <line x1="3" y1="6" x2="21" y2="6"/>
        <path d="M16 10a4 4 0 0 1-8 0"/>
      </svg>
    {/if}
  </div>

  <h2 class="empty-state__title">{title}</h2>

  {#if description}
    <p class="empty-state__desc">{description}</p>
  {/if}

  {#if ctaLabel}
    {#if ctaHref}
      <a data-testid="empty-state-cta" href={ctaHref} class="btn btn--primary">{ctaLabel}</a>
    {:else if onCta}
      <button data-testid="empty-state-cta" class="btn btn--primary" onclick={onCta}>{ctaLabel}</button>
    {/if}
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--space-12) var(--space-8);
    gap: var(--space-4);
    height: 100%;
    min-height: 240px;
  }

  .empty-state--sm {
    padding: var(--space-6) var(--space-4);
    min-height: 160px;
    gap: var(--space-2);
  }

  .empty-state--lg {
    padding: var(--space-16) var(--space-12);
    min-height: 360px;
    gap: var(--space-6);
  }

  .empty-state__icon {
    color: var(--color-text-muted);
    opacity: 0.6;
  }

  .empty-state--sm .empty-state__icon :global(svg) {
    width: 32px;
    height: 32px;
  }

  .empty-state__title {
    font-size: var(--text-xl);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .empty-state--sm .empty-state__title {
    font-size: var(--text-base);
  }

  .empty-state__desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    max-width: 320px;
    line-height: 1.6;
    margin: 0;
  }
</style>
