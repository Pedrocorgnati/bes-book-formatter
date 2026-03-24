<script lang="ts">
  import type { Snippet } from 'svelte';
  import { t } from '$lib/i18n/engine';

  interface Props {
    collapsed?: boolean;
    onToggle?: () => void;
    children?: Snippet;
  }

  let { collapsed = false, onToggle, children }: Props = $props();
</script>

<aside
  data-testid="right-panel-aside"
  class="right-panel"
  class:right-panel--collapsed={collapsed}
  aria-label={t('a11y.propertiesPanel')}
>
  <!-- Toggle button -->
  <button
    data-testid="right-panel-toggle-button"
    class="right-panel__toggle"
    onclick={onToggle}
    aria-expanded={!collapsed}
    aria-label={collapsed ? t('a11y.expandPanel') : t('a11y.collapsePanel')}
  >
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      aria-hidden="true"
    >
      {#if collapsed}
        <polyline points="15 18 9 12 15 6" />
      {:else}
        <polyline points="9 18 15 12 9 6" />
      {/if}
    </svg>
  </button>

  {#if !collapsed}
    <div data-testid="right-panel-content" class="right-panel__content">
      {#if children}
        {@render children()}
      {:else}
        <div class="right-panel__empty">
          <p class="right-panel__empty-text">{t('emptyState.noProjectSelected')}</p>
        </div>
      {/if}
    </div>
  {/if}
</aside>

<style>
  .right-panel {
    display: flex;
    flex-direction: column;
    border-left: 1px solid var(--color-border);
    background: var(--color-surface);
    overflow: hidden;
    position: relative;
    transition: width var(--duration-normal) ease;
  }

  .right-panel__toggle {
    position: absolute;
    left: -12px;
    top: 50%;
    transform: translateY(-50%);
    width: 24px;
    height: 24px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--color-text-muted);
    z-index: 10;
    transition:
      background-color var(--duration-fast) ease,
      color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .right-panel__toggle:hover {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }

  .right-panel__toggle:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .right-panel__content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-4);
  }

  .right-panel__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-6);
    text-align: center;
  }

  .right-panel__empty-text {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    line-height: 1.5;
  }
</style>
