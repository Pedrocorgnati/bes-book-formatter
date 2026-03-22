<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n/engine';
  import TypographyPanel from '$lib/components/typography/TypographyPanel.svelte';
  import type { ApiResponse } from '$lib/types';

  interface Annotation {
    id: string;
    projectId: string;
    pageNumber: number;
    xPercent: number;
    yPercent: number;
    annotationType: string;
    color: string;
    content: string;
    createdAt: string;
  }

  interface TypoIssue {
    issueType: string;
    pageNumber: number;
    lineText: string;
    lineYPercent: number;
    severity: string;
  }

  interface Props {
    projectId: string;
    collapsed?: boolean;
    currentPage?: number;
    onToggleCollapse: () => void;
    onNavigate?: (page: number) => void;
    onTypoIssuesDetected?: (issues: TypoIssue[]) => void;
  }

  let {
    projectId,
    collapsed = false,
    currentPage = 1,
    onToggleCollapse,
    onNavigate,
    onTypoIssuesDetected,
  }: Props = $props();

  // Annotations summary state
  let annotations = $state<Annotation[]>([]);
  let annotationsExpanded = $state(false);
  let annotationsLoading = $state(false);

  // Typo issues state
  let typoIssues = $state<TypoIssue[]>([]);
  let typoDetecting = $state(false);
  let typoDetected = $state(false);

  async function loadAnnotations() {
    if (!projectId) return;
    annotationsLoading = true;
    try {
      const res = await invoke<ApiResponse<Annotation[]>>('get_annotations', {
        projectId,
        pageNumber: 0, // 0 = all pages
      });
      if (res.data) annotations = res.data;
    } catch (e) {
      console.error('[PreviewRightPanel] loadAnnotations error:', e);
    } finally {
      annotationsLoading = false;
    }
  }

  async function detectTypoIssues() {
    if (!projectId) return;
    typoDetecting = true;
    try {
      const res = await invoke<ApiResponse<TypoIssue[]>>('detect_orphans_widows', { projectId });
      if (res.data) {
        typoIssues = res.data;
        typoDetected = true;
        onTypoIssuesDetected?.(res.data);
      }
    } catch (e) {
      console.error('[PreviewRightPanel] detectTypoIssues error:', e);
    } finally {
      typoDetecting = false;
    }
  }

  // Load annotations when panel mounts/projectId changes
  $effect(() => {
    if (projectId && !collapsed) loadAnnotations();
  });

  const commentCount = $derived(annotations.filter((a) => a.annotationType === 'comment').length);
  const highlightCount = $derived(annotations.filter((a) => a.annotationType === 'highlight').length);
  const flagCount = $derived(annotations.filter((a) => a.annotationType === 'flag').length);

  const visibleAnnotations = $derived(
    annotationsExpanded ? annotations : annotations.slice(0, 5)
  );

  const orphanCount = $derived(typoIssues.filter((i) => i.issueType === 'orphan').length);
  const widowCount = $derived(typoIssues.filter((i) => i.issueType === 'widow').length);

  function annotationIcon(type: string) {
    if (type === 'comment') return '💬';
    if (type === 'flag') return '🚩';
    return '🖍';
  }
</script>

<aside
  class="preview-right-panel"
  class:collapsed
  aria-label={t('preview.rightPanelLabel')}
>
  <!-- Collapse toggle -->
  <button
    class="panel-collapse-btn"
    onclick={onToggleCollapse}
    aria-label={collapsed ? t('preview.expandRightPanel') : t('preview.collapseRightPanel')}
    title={collapsed ? t('preview.expandRightPanel') : t('preview.collapseRightPanel')}
  >
    {collapsed ? '◀' : '▶'}
  </button>

  {#if !collapsed}
    <!-- Section: Typography -->
    <div class="panel-section panel-section--typography">
      <div class="section-header">
        <h3 class="section-title">{t('nav.typography')}</h3>
      </div>
      <div class="typography-panel-wrapper">
        <TypographyPanel {projectId} />
      </div>
    </div>

    <!-- Divider -->
    <hr class="panel-divider" />

    <!-- Section: Annotations Summary -->
    <div class="panel-section panel-section--annotations">
      <div class="section-header">
        <h3 class="section-title">{t('preview.annotations')}</h3>
        <button
          class="section-refresh-btn"
          onclick={loadAnnotations}
          aria-label={t('preview.refreshAnnotations')}
          title={t('preview.refreshAnnotations')}
          disabled={annotationsLoading}
        >
          {annotationsLoading ? '…' : '↻'}
        </button>
      </div>

      {#if annotations.length === 0 && !annotationsLoading}
        <p class="section-empty">{t('preview.noAnnotations')}</p>
      {:else}
        <div class="annotation-counts">
          {#if commentCount > 0}
            <span class="count-badge count-badge--comment">💬 {commentCount}</span>
          {/if}
          {#if highlightCount > 0}
            <span class="count-badge count-badge--highlight">🖍 {highlightCount}</span>
          {/if}
          {#if flagCount > 0}
            <span class="count-badge count-badge--flag">🚩 {flagCount}</span>
          {/if}
        </div>

        <ul class="annotation-list" role="list">
          {#each visibleAnnotations as ann (ann.id)}
            <li>
              <button
                class="annotation-list-item"
                onclick={() => onNavigate?.(ann.pageNumber)}
                aria-label="{t('preview.goToPage')} {ann.pageNumber}: {ann.content}"
              >
                <span class="ann-icon">{annotationIcon(ann.annotationType)}</span>
                <span class="ann-text">{ann.content || ann.annotationType}</span>
                <span class="ann-page">p.{ann.pageNumber}</span>
              </button>
            </li>
          {/each}
        </ul>

        {#if annotations.length > 5}
          <button
            class="section-expand-btn"
            onclick={() => (annotationsExpanded = !annotationsExpanded)}
          >
            {annotationsExpanded
              ? t('preview.showLess')
              : t('preview.showAll', { count: String(annotations.length) })}
          </button>
        {/if}
      {/if}
    </div>

    <!-- Divider -->
    <hr class="panel-divider" />

    <!-- Section: Orphan/Widow Summary -->
    <div class="panel-section panel-section--typo">
      <div class="section-header">
        <h3 class="section-title">{t('preview.typoQuality')}</h3>
      </div>

      {#if !typoDetected}
        <p class="section-empty">{t('preview.typoNotChecked')}</p>
      {:else if typoIssues.length === 0}
        <p class="section-ok">✅ {t('preview.noTypoIssues')}</p>
      {:else}
        <div class="typo-counts">
          {#if orphanCount > 0}
            <span class="count-badge count-badge--orphan">⚠ {orphanCount} {t('preview.orphans')}</span>
          {/if}
          {#if widowCount > 0}
            <span class="count-badge count-badge--widow">⚠ {widowCount} {t('preview.widows')}</span>
          {/if}
        </div>

        <ul class="typo-list" role="list">
          {#each typoIssues.slice(0, 5) as issue, i}
            <li>
              <button
                class="typo-list-item"
                onclick={() => onNavigate?.(issue.pageNumber)}
                aria-label="{issue.issueType} {t('preview.onPage')} {issue.pageNumber}"
              >
                <span class="typo-type typo-type--{issue.issueType}">
                  {issue.issueType === 'orphan' ? t('preview.orphan') : t('preview.widow')}
                </span>
                <span class="typo-text">{issue.lineText}</span>
                <span class="typo-page">p.{issue.pageNumber}</span>
              </button>
            </li>
          {/each}
          {#if typoIssues.length > 5}
            <li class="typo-more">+{typoIssues.length - 5} {t('preview.moreIssues')}</li>
          {/if}
        </ul>
      {/if}

      <button
        class="btn-check-typo"
        onclick={detectTypoIssues}
        disabled={typoDetecting}
        aria-label={t('preview.checkOrphansWidows')}
      >
        {typoDetecting ? t('preview.detecting') : t('preview.checkOrphansWidows')}
      </button>
    </div>
  {/if}
</aside>

<style>
  .preview-right-panel {
    width: 280px;
    min-width: 280px;
    display: flex;
    flex-direction: column;
    background: var(--color-surface);
    border-left: 1px solid var(--color-border);
    height: 100%;
    overflow: hidden;
    transition: width var(--duration-normal), min-width var(--duration-normal);
    position: relative;
    flex-shrink: 0;
  }

  .preview-right-panel.collapsed {
    width: 24px;
    min-width: 24px;
  }

  .panel-collapse-btn {
    position: absolute;
    top: var(--space-2);
    left: var(--space-1);
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-surface-hover);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: var(--text-sm);
    cursor: pointer;
    color: var(--color-text-muted);
    z-index: 1;
    line-height: 1;
  }

  .panel-section {
    padding: var(--space-3);
    overflow-y: auto;
  }

  .panel-section--typography {
    flex-shrink: 0;
    max-height: 45%;
    overflow-y: auto;
  }

  .panel-section--annotations,
  .panel-section--typo {
    flex: 1;
    overflow-y: auto;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-2);
    padding-top: var(--space-6); /* space for collapse btn */
  }

  .panel-section--annotations .section-header,
  .panel-section--typo .section-header {
    padding-top: 0;
  }

  .section-title {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .section-refresh-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-text-muted);
    font-size: var(--text-base);
    padding: 2px 4px;
    border-radius: var(--radius-sm);
    line-height: 1;
  }

  .section-refresh-btn:hover {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }

  .section-empty {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-align: center;
    padding: var(--space-3) 0;
    margin: 0;
  }

  .section-ok {
    font-size: var(--text-xs);
    color: var(--color-success, #16a34a);
    margin: 0 0 var(--space-2) 0;
  }

  .panel-divider {
    border: none;
    border-top: 1px solid var(--color-border);
    margin: 0;
  }

  /* Annotation counts */
  .annotation-counts,
  .typo-counts {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-bottom: var(--space-2);
  }

  .count-badge {
    font-size: var(--text-xs);
    padding: 1px 6px;
    border-radius: var(--radius-full);
    background: var(--color-surface-hover);
    color: var(--color-text-muted);
  }

  .count-badge--orphan,
  .count-badge--widow {
    background: rgba(245, 158, 11, 0.12);
    color: var(--color-warning, #b45309);
  }

  /* Annotation list */
  .annotation-list,
  .typo-list {
    list-style: none;
    margin: 0 0 var(--space-2) 0;
    padding: 0;
  }

  .annotation-list-item,
  .typo-list-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    width: 100%;
    padding: var(--space-1) var(--space-2);
    background: none;
    border: none;
    border-radius: var(--radius-sm);
    cursor: pointer;
    text-align: left;
    color: var(--color-text);
    font-size: var(--text-xs);
    transition: background var(--duration-fast);
  }

  .annotation-list-item:hover,
  .typo-list-item:hover {
    background: var(--color-surface-hover);
  }

  .ann-icon,
  .typo-type {
    flex-shrink: 0;
    font-size: 12px;
  }

  .typo-type--orphan {
    color: var(--color-error, #ef4444);
  }

  .typo-type--widow {
    color: var(--color-warning, #f59e0b);
  }

  .ann-text,
  .typo-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--color-text-muted);
  }

  .ann-page,
  .typo-page {
    flex-shrink: 0;
    font-size: 10px;
    color: var(--color-text-muted);
    opacity: 0.7;
  }

  .typo-more {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    padding: var(--space-1) var(--space-2);
  }

  .section-expand-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: var(--text-xs);
    color: var(--color-primary);
    padding: 0;
    text-decoration: underline;
  }

  .btn-check-typo {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    margin-top: var(--space-2);
    background: var(--color-surface-hover);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    cursor: pointer;
    color: var(--color-text);
    transition: background var(--duration-fast);
  }

  .btn-check-typo:hover:not(:disabled) {
    background: color-mix(in srgb, var(--color-primary) 8%, transparent);
    border-color: var(--color-primary);
    color: var(--color-primary);
  }

  .btn-check-typo:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .typography-panel-wrapper {
    overflow-y: auto;
  }
</style>
