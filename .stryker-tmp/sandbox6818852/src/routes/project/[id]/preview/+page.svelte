<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { invoke } from '@tauri-apps/api/core';
  import type { ApiResponse } from '$lib/types';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import PreviewSidebar from '$lib/components/preview/PreviewSidebar.svelte';
  import PageSpreadViewer from '$lib/components/preview/PageSpreadViewer.svelte';
  import PreviewToolbar from '$lib/components/preview/PreviewToolbar.svelte';
  import PreviewRightPanel from '$lib/components/preview/PreviewRightPanel.svelte';
  import DistractionFreeMode from '$lib/components/preview/DistractionFreeMode.svelte';

  interface TypoIssue {
    issueType: string;
    pageNumber: number;
    lineText: string;
    lineYPercent: number;
    severity: string;
  }

  const projectId = $derived($page.params.id ?? '');
  const project = $derived($projectsStore.current);

  // Layout state
  let sidebarCollapsed = $state(false);
  let rightPanelCollapsed = $state(false);

  // Preview state
  let currentPage = $state(1);
  let zoom = $state(1.0);
  let spreadMode = $state(false);
  let showRuler = $state(false);
  let showAnnotations = $state(true);
  let showTypoHighlights = $state(false);
  let typoIssues = $state<TypoIssue[]>([]);
  let totalPages = $state(0);
  let renderMs = $state(0);

  function handleRendered(data: { totalPages: number; renderMs: number }) {
    totalPages = data.totalPages;
    renderMs = data.renderMs;
  }

  function handleTypoIssuesDetected(issues: TypoIssue[]) {
    typoIssues = issues;
    showTypoHighlights = issues.length > 0;
  }

  async function detectTypoIssuesFromToolbar() {
    if (!projectId) return;
    try {
      const res = await invoke<ApiResponse<TypoIssue[]>>('detect_orphans_widows', { projectId });
      if (res.data) {
        handleTypoIssuesDetected(res.data);
      }
    } catch (e) {
      console.error('[Preview] detectTypoIssues error:', e);
    }
  }

  // Global keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;

    // Cmd+B: toggle sidebar
    if (mod && e.key === 'b' && !e.shiftKey) {
      e.preventDefault();
      sidebarCollapsed = !sidebarCollapsed;
      return;
    }

    // Cmd+R: toggle right panel (but not Cmd+Shift+F / F11 which DistractionFreeMode handles)
    if (mod && e.key === 'r' && !e.shiftKey) {
      e.preventDefault();
      rightPanelCollapsed = !rightPanelCollapsed;
      return;
    }
  }
</script>

<svelte:head>
  <title>BES Book Formatter — {project?.name ?? t('nav.preview')}</title>
</svelte:head>

<svelte:window onkeydown={handleKeydown} />

<!-- Distraction-free mode overlay (keyboard listener + exit hint) -->
<DistractionFreeMode />

{#if !project}
  <EmptyState
    icon="eye"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <!-- 3-panel preview layout -->
  <div
    class="preview-layout"
    class:sidebar-collapsed={sidebarCollapsed}
    class:right-collapsed={rightPanelCollapsed}
    role="application"
    aria-label={t('preview.layoutLabel')}
  >
    <!-- Left: Sidebar -->
    <PreviewSidebar
      {projectId}
      {currentPage}
      collapsed={sidebarCollapsed}
      onNavigate={(page) => (currentPage = page)}
      onToggleCollapse={() => (sidebarCollapsed = !sidebarCollapsed)}
    />

    <!-- Center: Main preview area -->
    <main class="preview-main">
      <!-- Toolbar -->
      <PreviewToolbar
        {currentPage}
        {totalPages}
        {zoom}
        {spreadMode}
        {showRuler}
        {renderMs}
        {showAnnotations}
        {showTypoHighlights}
        typoIssueCount={typoIssues.length}
        onNavigate={(p) => (currentPage = p)}
        onZoomChange={(z) => (zoom = z)}
        onSpreadToggle={(s) => (spreadMode = s)}
        onRulerToggle={(r) => (showRuler = r)}
        onAnnotationsToggle={(a) => (showAnnotations = a)}
        onTypoHighlightsToggle={(h) => (showTypoHighlights = h)}
        onDetectTypoIssues={detectTypoIssuesFromToolbar}
      />

      <!-- Canvas: page viewer with overlays -->
      <div class="preview-canvas-container">
        <PageSpreadViewer
          {projectId}
          {currentPage}
          {zoom}
          {spreadMode}
          {showRuler}
          {showAnnotations}
          {typoIssues}
          {showTypoHighlights}
          onRendered={handleRendered}
          onNavigate={(p) => (currentPage = p)}
        />
      </div>
    </main>

    <!-- Right: Configuration panel -->
    <PreviewRightPanel
      {projectId}
      collapsed={rightPanelCollapsed}
      {currentPage}
      onToggleCollapse={() => (rightPanelCollapsed = !rightPanelCollapsed)}
      onNavigate={(p) => (currentPage = p)}
      onTypoIssuesDetected={handleTypoIssuesDetected}
    />
  </div>
{/if}

<style>
  .preview-layout {
    display: grid;
    grid-template-columns: 260px 1fr 280px;
    height: 100vh;
    overflow: hidden;
    transition: grid-template-columns var(--duration-normal);
  }

  .preview-layout.sidebar-collapsed {
    grid-template-columns: 48px 1fr 280px;
  }

  .preview-layout.right-collapsed {
    grid-template-columns: 260px 1fr 24px;
  }

  .preview-layout.sidebar-collapsed.right-collapsed {
    grid-template-columns: 48px 1fr 24px;
  }

  .preview-main {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 0;
  }

  .preview-canvas-container {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  /* Responsive: below 1280px — auto-collapse panels */
  @media (max-width: 1279px) {
    .preview-layout {
      grid-template-columns: 48px 1fr 0px;
    }

    .preview-layout.sidebar-collapsed {
      grid-template-columns: 48px 1fr 0px;
    }
  }

  /* Distraction-free: hide both panels */
  :global(body.distraction-free) .preview-layout {
    grid-template-columns: 0 1fr 0;
  }

  :global(body.distraction-free) .preview-main {
    grid-column: 1 / -1;
  }
</style>
