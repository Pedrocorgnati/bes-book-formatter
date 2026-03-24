<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import type { ApiResponse } from '$lib/types';
  import AnnotationLayer from './AnnotationLayer.svelte';
  import OrphanWidowHighlight from './OrphanWidowHighlight.svelte';

  interface PageImage {
    pageNumber: number;
    imageBase64: string;
    widthPx: number;
    heightPx: number;
  }

  interface TypoIssue {
    issueType: string;
    pageNumber: number;
    lineText: string;
    lineYPercent: number;
    severity: string;
  }

  interface PreviewPageResponse {
    pages: PageImage[];
    totalPages: number;
    renderMs: number;
  }

  interface Props {
    projectId: string;
    currentPage: number;
    zoom: number;
    spreadMode: boolean;
    showRuler: boolean;
    showAnnotations?: boolean;
    typoIssues?: TypoIssue[];
    showTypoHighlights?: boolean;
    onRendered?: (data: { totalPages: number; renderMs: number }) => void;
    onNavigate?: (page: number) => void;
  }

  let {
    projectId,
    currentPage = 1,
    zoom = 1.0,
    spreadMode = false,
    showRuler = false,
    showAnnotations = false,
    typoIssues = [],
    showTypoHighlights = false,
    onRendered,
    onNavigate,
  }: Props = $props();

  // State
  let pages = $state<PageImage[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let renderMs = $state(0);
  let totalPages = $state(0);

  // In-memory page cache: "page_zoom_spread" → PageImage
  const pageCache = new Map<string, PageImage>();
  const MAX_CACHE = 10;

  function cacheKey(page: number, z: number, spread: boolean) {
    return `${page}_${z}_${spread}`;
  }

  function evictCache() {
    if (pageCache.size > MAX_CACHE) {
      const first = pageCache.keys().next().value;
      if (first) pageCache.delete(first);
    }
  }

  async function renderPage(page: number, z: number, spread: boolean) {
    if (!projectId) return;

    // Check in-memory cache
    const key = cacheKey(page, z, spread);
    if (pageCache.has(key)) {
      const cached = pageCache.get(key)!;
      pages = [cached];
      renderMs = 0;
      onRendered?.({ totalPages, renderMs: 0 });
      // Prefetch adjacent pages in background
      prefetchAdjacent(page);
      return;
    }

    loading = true;
    error = null;

    try {
      const res = await invoke<ApiResponse<PreviewPageResponse>>('render_preview_page', {
        projectId,
        page,
        zoom: z,
        spread,
      });

      if (res.error) {
        error = res.error;
        return;
      }

      const data = res.data!;
      pages = data.pages;
      totalPages = data.totalPages;
      renderMs = data.renderMs;

      // Cache pages
      for (const p of data.pages) {
        pageCache.set(cacheKey(p.pageNumber, z, spread), p);
        evictCache();
      }

      onRendered?.({ totalPages: data.totalPages, renderMs: data.renderMs });

      // Warn on slow render
      if (data.renderMs > 1000) {
        console.warn(`[Preview] Slow render: ${data.renderMs}ms`);
      }

      // Prefetch adjacent pages
      prefetchAdjacent(page);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function prefetchAdjacent(page: number) {
    if (page > 1) renderPageBackground(page - 1);
    if (page < totalPages) renderPageBackground(page + 1);
  }

  async function renderPageBackground(page: number) {
    const key = cacheKey(page, zoom, spreadMode);
    if (pageCache.has(key)) return;
    try {
      const res = await invoke<ApiResponse<PreviewPageResponse>>('render_preview_page', {
        projectId,
        page,
        zoom,
        spread: spreadMode,
      });
      if (res.data) {
        for (const p of res.data.pages) {
          pageCache.set(cacheKey(p.pageNumber, zoom, spreadMode), p);
          evictCache();
        }
      }
    } catch {
      // Silent — prefetch failures are non-critical
    }
  }

  // Keyboard navigation
  function handleKeydown(e: KeyboardEvent) {
    if (!onNavigate) return;
    if (e.key === 'ArrowLeft' && currentPage > 1) {
      onNavigate(currentPage - 1);
    } else if (e.key === 'ArrowRight' && currentPage < totalPages) {
      onNavigate(currentPage + 1);
    }
  }

  // Debounce timer for live preview events
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let unlistenConfig: (() => void) | null = null;
  let unlistenAst: (() => void) | null = null;

  function invalidateLocalCache() {
    pageCache.clear();
  }

  onMount(async () => {
    // Initial render
    await renderPage(currentPage, zoom, spreadMode);

    // Live preview: listen for config-changed events (typography updates)
    unlistenConfig = await listen<{ projectId: string }>('preview:config-changed', (event) => {
      if (event.payload.projectId !== projectId) return;
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        invalidateLocalCache();
        renderPage(currentPage, zoom, spreadMode);
      }, 300);
    });

    // Live preview: listen for ast-changed events (manuscript save)
    unlistenAst = await listen<{ projectId: string }>('preview:ast-changed', (event) => {
      if (event.payload.projectId !== projectId) return;
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        invalidateLocalCache();
        renderPage(1, zoom, false);
        onNavigate?.(1);
      }, 300);
    });
  });

  onDestroy(() => {
    if (debounceTimer) clearTimeout(debounceTimer);
    unlistenConfig?.();
    unlistenAst?.();
  });

  // Re-render when props change
  $effect(() => {
    renderPage(currentPage, zoom, spreadMode);
  });

  function retry() {
    error = null;
    renderPage(currentPage, zoom, spreadMode);
  }

  // Compute display width based on zoom
  const displayWidth = $derived(
    pages.length > 0
      ? Math.round(pages[0].widthPx * (zoom === 0.0 ? 1.0 : zoom))
      : 595
  );
  const displayHeight = $derived(
    pages.length > 0
      ? Math.round(pages[0].heightPx * (zoom === 0.0 ? 1.0 : zoom))
      : 842
  );
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="page-spread-viewer"
  class:spread={spreadMode}
  role="region"
  aria-label={t('preview.viewerLabel')}
  aria-live="polite"
>
  {#if loading}
    <!-- Loading skeleton -->
    <div class="pages-container" aria-hidden="true">
      <div
        class="page-skeleton skeleton-pulse"
        style="width:{displayWidth}px; height:{displayHeight}px;"
      ></div>
      {#if spreadMode}
        <div
          class="page-skeleton skeleton-pulse"
          style="width:{displayWidth}px; height:{displayHeight}px;"
        ></div>
      {/if}
    </div>
    <span class="sr-only">{t('preview.loading')}</span>

  {:else if error}
    <!-- Error state -->
    <div class="preview-error" role="alert">
      <p class="preview-error__msg">{error}</p>
      <button class="btn btn--secondary" onclick={retry}>
        {t('common.retry')}
      </button>
    </div>

  {:else if pages.length === 0}
    <!-- Empty state (no project loaded yet) -->
    <div class="preview-empty" role="status">
      <p>{t('preview.noProject')}</p>
    </div>

  {:else}
    <!-- Rendered pages -->
    <div class="pages-container" class:spread-layout={spreadMode}>
      {#each pages as page}
        <div class="page-wrapper" style="position: relative; width:{displayWidth}px; height:{displayHeight}px;">
          <img
            src="data:image/png;base64,{page.imageBase64}"
            alt={t('preview.pageAlt', { n: String(page.pageNumber) })}
            width={displayWidth}
            height={displayHeight}
            class="page-image"
            style="width:{displayWidth}px; height:{displayHeight}px;"
            aria-label={t('preview.pageLabel', { n: String(page.pageNumber), total: String(totalPages) })}
            draggable="false"
          />

          {#if showAnnotations && projectId}
            <AnnotationLayer
              {projectId}
              pageNumber={page.pageNumber}
              pageWidthPx={displayWidth}
              pageHeightPx={displayHeight}
              visible={showAnnotations}
            />
          {/if}
          {#if showTypoHighlights && typoIssues.length > 0}
            <OrphanWidowHighlight
              issues={typoIssues}
              pageNumber={page.pageNumber}
              pageWidthPx={displayWidth}
              pageHeightPx={displayHeight}
              visible={showTypoHighlights}
            />
          {/if}

          {#if showRuler}
            <div class="ruler ruler-h" aria-hidden="true">
              {#each Array.from({ length: 20 }, (_, i) => i) as mark}
                <span class="ruler-mark" style="left:{mark * 5}%">
                  {#if mark % 2 === 0}<span class="ruler-label">{mark * 10}mm</span>{/if}
                </span>
              {/each}
            </div>
            <div class="ruler ruler-v" aria-hidden="true">
              {#each Array.from({ length: 20 }, (_, i) => i) as mark}
                <span class="ruler-mark-v" style="top:{mark * 5}%">
                  {#if mark % 2 === 0}<span class="ruler-label-v">{mark * 10}mm</span>{/if}
                </span>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .page-spread-viewer {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    overflow: auto;
    padding: var(--space-6);
    background: var(--color-canvas-bg, #e8e4dc);
    box-sizing: border-box;
  }

  .pages-container {
    display: flex;
    gap: 0;
    align-items: flex-start;
  }

  .spread-layout {
    gap: var(--space-3);
  }

  .page-wrapper {
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.18), 0 1px 4px rgba(0, 0, 0, 0.1);
    border-radius: 1px;
    overflow: hidden;
    flex-shrink: 0;
  }

  .page-image {
    display: block;
    border-radius: 1px;
  }

  .page-skeleton {
    border-radius: 1px;
    background: var(--color-surface);
    flex-shrink: 0;
  }

  .skeleton-pulse {
    animation: skeleton-pulse 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }

  .preview-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-8);
    text-align: center;
  }

  .preview-error__msg {
    color: var(--color-error);
    font-size: var(--text-sm);
    max-width: 400px;
  }

  .preview-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }

  /* Ruler */
  .ruler {
    position: absolute;
    pointer-events: none;
  }

  .ruler-h {
    top: 0;
    left: 0;
    width: 100%;
    height: 16px;
    background: rgba(255, 255, 255, 0.7);
    border-bottom: 1px solid rgba(0, 0, 0, 0.15);
  }

  .ruler-v {
    top: 0;
    left: 0;
    width: 16px;
    height: 100%;
    background: rgba(255, 255, 255, 0.7);
    border-right: 1px solid rgba(0, 0, 0, 0.15);
  }

  .ruler-mark {
    position: absolute;
    top: 4px;
    width: 1px;
    height: 8px;
    background: rgba(0, 0, 0, 0.3);
    transform: translateX(-50%);
  }

  .ruler-label {
    position: absolute;
    top: 0;
    left: 2px;
    font-size: 8px;
    color: rgba(0, 0, 0, 0.5);
    white-space: nowrap;
  }

  .ruler-mark-v {
    position: absolute;
    left: 4px;
    width: 8px;
    height: 1px;
    background: rgba(0, 0, 0, 0.3);
    transform: translateY(-50%);
  }

  .ruler-label-v {
    position: absolute;
    top: 2px;
    left: 0;
    font-size: 8px;
    color: rgba(0, 0, 0, 0.5);
    white-space: nowrap;
    transform: rotate(-90deg) translateX(-100%);
    transform-origin: top left;
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
