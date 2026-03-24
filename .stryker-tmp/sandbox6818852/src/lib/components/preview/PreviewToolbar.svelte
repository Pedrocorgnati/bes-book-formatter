<script lang="ts">
  import { t } from '$lib/i18n/engine';

  interface Props {
    currentPage: number;
    totalPages: number;
    zoom: number;
    spreadMode: boolean;
    showRuler: boolean;
    renderMs: number;
    showAnnotations?: boolean;
    showTypoHighlights?: boolean;
    typoIssueCount?: number;
    onNavigate: (page: number) => void;
    onZoomChange: (zoom: number) => void;
    onSpreadToggle: (enabled: boolean) => void;
    onRulerToggle: (enabled: boolean) => void;
    onAnnotationsToggle?: (enabled: boolean) => void;
    onTypoHighlightsToggle?: (enabled: boolean) => void;
    onDetectTypoIssues?: () => void;
  }

  let {
    currentPage,
    totalPages,
    zoom,
    spreadMode,
    showRuler,
    renderMs,
    showAnnotations = true,
    showTypoHighlights = false,
    typoIssueCount = 0,
    onNavigate,
    onZoomChange,
    onSpreadToggle,
    onRulerToggle,
    onAnnotationsToggle,
    onTypoHighlightsToggle,
    onDetectTypoIssues,
  }: Props = $props();

  const ZOOM_OPTIONS = [
    { value: 0.5, label: '50%' },
    { value: 0.75, label: '75%' },
    { value: 1.0, label: '100%' },
    { value: 1.25, label: '125%' },
    { value: 1.5, label: '150%' },
    { value: 0.0, label: t('preview.fitPage') },
  ];

  let pageInputValue = $state('');

  $effect(() => {
    pageInputValue = String(currentPage);
  });

  function handlePageInputBlur() {
    const parsed = parseInt(pageInputValue, 10);
    if (!isNaN(parsed)) {
      const clamped = Math.max(1, Math.min(parsed, totalPages));
      onNavigate(clamped);
    } else {
      pageInputValue = String(currentPage);
    }
  }

  function handlePageInputKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      (e.currentTarget as HTMLInputElement).blur();
    }
    if (e.key === 'Escape') {
      pageInputValue = String(currentPage);
      (e.currentTarget as HTMLInputElement).blur();
    }
  }

  const timingClass = $derived(
    renderMs === 0
      ? 'badge--info'
      : renderMs < 300
      ? 'badge--success'
      : renderMs < 500
      ? 'badge--warning'
      : 'badge--error'
  );

  const timingLabel = $derived(
    renderMs === 0 ? '⚡ cache' : `${renderMs}ms`
  );
</script>

<div class="preview-toolbar" role="toolbar" aria-label={t('preview.toolbarLabel')}>
  <!-- Navigation group -->
  <div class="toolbar-group toolbar-nav">
    <button
      class="btn-icon"
      onclick={() => onNavigate(currentPage - 1)}
      disabled={currentPage <= 1}
      aria-label={t('preview.prevPage')}
      title={t('preview.prevPage')}
    >
      ←
    </button>

    <div class="page-input-wrap">
      <input
        type="number"
        class="page-input"
        bind:value={pageInputValue}
        onblur={handlePageInputBlur}
        onkeydown={handlePageInputKeydown}
        min="1"
        max={totalPages}
        aria-label={t('preview.pageInputLabel')}
      />
      <span class="page-total" aria-label={t('preview.ofPages', { total: String(totalPages) })}>
        / {totalPages}
      </span>
    </div>

    <button
      class="btn-icon"
      onclick={() => onNavigate(currentPage + 1)}
      disabled={currentPage >= totalPages}
      aria-label={t('preview.nextPage')}
      title={t('preview.nextPage')}
    >
      →
    </button>
  </div>

  <div class="toolbar-separator" aria-hidden="true"></div>

  <!-- Zoom group -->
  <div class="toolbar-group">
    <label class="sr-only" for="zoom-select">{t('preview.zoom')}</label>
    <select
      id="zoom-select"
      class="zoom-select"
      value={zoom}
      onchange={(e) => onZoomChange(parseFloat((e.currentTarget as HTMLSelectElement).value))}
      aria-label={t('preview.zoom')}
    >
      {#each ZOOM_OPTIONS as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  </div>

  <div class="toolbar-separator" aria-hidden="true"></div>

  <!-- View options group -->
  <div class="toolbar-group toolbar-extras">
    <label class="toggle-label">
      <input
        type="checkbox"
        checked={spreadMode}
        onchange={(e) => onSpreadToggle((e.currentTarget as HTMLInputElement).checked)}
        aria-label={t('preview.spreadMode')}
      />
      <span>{t('preview.spread')}</span>
    </label>

    <label class="toggle-label">
      <input
        type="checkbox"
        checked={showRuler}
        onchange={(e) => onRulerToggle((e.currentTarget as HTMLInputElement).checked)}
        aria-label={t('preview.ruler')}
      />
      <span>{t('preview.ruler')}</span>
    </label>

    {#if onAnnotationsToggle}
      <label class="toggle-label">
        <input
          type="checkbox"
          checked={showAnnotations}
          onchange={(e) => onAnnotationsToggle?.((e.currentTarget as HTMLInputElement).checked)}
          aria-label={t('preview.annotations')}
        />
        <span>{t('preview.annotations')}</span>
      </label>
    {/if}

    {#if onDetectTypoIssues}
      <button
        class="btn-sm {typoIssueCount > 0 ? 'btn-sm--warning' : ''}"
        onclick={onDetectTypoIssues}
        aria-label={t('preview.checkOrphans')}
        title={t('preview.checkOrphans')}
      >
        {typoIssueCount > 0 ? `⚠ ${typoIssueCount}` : t('preview.checkOrphans')}
      </button>
    {/if}
  </div>

  <!-- Timing badge -->
  <div class="toolbar-timing" aria-live="polite" aria-atomic="true">
    <span class="badge {timingClass}" title={t('preview.renderTime')}>
      {timingLabel}
    </span>
  </div>
</div>

<style>
  .preview-toolbar {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-4);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    min-height: 44px;
    flex-shrink: 0;
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .toolbar-separator {
    width: 1px;
    height: 20px;
    background: var(--color-border);
    margin: 0 var(--space-1);
  }

  .toolbar-nav {
    gap: var(--space-1);
  }

  .toolbar-timing {
    margin-left: auto;
  }

  .toolbar-extras {
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    cursor: pointer;
    font-size: var(--text-base);
    color: var(--color-text);
    transition: background var(--duration-fast);
  }

  .btn-icon:hover:not(:disabled) {
    background: var(--color-surface-hover);
  }

  .btn-icon:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .page-input-wrap {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .page-input {
    width: 48px;
    height: 28px;
    text-align: center;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--text-sm);
    padding: 0 var(--space-1);
  }

  .page-input::-webkit-inner-spin-button,
  .page-input::-webkit-outer-spin-button {
    opacity: 0;
  }

  .page-total {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    white-space: nowrap;
  }

  .zoom-select {
    height: 28px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--text-sm);
    padding: 0 var(--space-2);
    cursor: pointer;
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    cursor: pointer;
    user-select: none;
  }

  .toggle-label input {
    cursor: pointer;
  }

  .btn-sm {
    height: 26px;
    padding: 0 var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--text-xs);
    cursor: pointer;
    white-space: nowrap;
  }

  .btn-sm--warning {
    border-color: var(--color-warning);
    color: var(--color-warning);
    background: color-mix(in srgb, var(--color-warning) 10%, transparent);
  }

  .badge {
    display: inline-flex;
    align-items: center;
    height: 22px;
    padding: 0 var(--space-2);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: 500;
    white-space: nowrap;
  }

  .badge--info {
    background: color-mix(in srgb, var(--color-info) 15%, transparent);
    color: var(--color-info);
  }

  .badge--success {
    background: color-mix(in srgb, var(--color-success) 15%, transparent);
    color: var(--color-success);
  }

  .badge--warning {
    background: color-mix(in srgb, var(--color-warning) 15%, transparent);
    color: var(--color-warning);
  }

  .badge--error {
    background: color-mix(in srgb, var(--color-error) 15%, transparent);
    color: var(--color-error);
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
