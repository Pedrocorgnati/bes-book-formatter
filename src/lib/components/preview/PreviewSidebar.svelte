<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n/engine';
  import { projectsStore, recentProjectsStore } from '$lib/stores/projectStore';
  import type { ApiResponse } from '$lib/types';

  interface Annotation {
    id: string;
    pageNumber: number;
    annotationType: string;
    color: string;
    content: string;
  }

  interface Props {
    projectId: string;
    currentPage: number;
    collapsed?: boolean;
    onNavigate: (page: number) => void;
    onToggleCollapse: () => void;
  }

  let {
    projectId,
    currentPage,
    collapsed = false,
    onNavigate,
    onToggleCollapse,
  }: Props = $props();

  type SidebarTab = 'chapters' | 'gallery' | 'projects';
  let activeTab = $state<SidebarTab>('chapters');
  let chaptersFilter = $state('');

  const project = $derived($projectsStore.current);
  const recentProjects = $derived($recentProjectsStore);
  const chapters = $derived(project?.besRootPath ? [] : []); // populated from parsed manuscript store if available

  // Annotations count per page for gallery tab
  let annotationsByPage = $state<Record<number, Annotation[]>>({});

  async function openProject(id: string) {
    try {
      await invoke('get_project', { projectId: id });
    } catch (e) {
      console.error('[PreviewSidebar] openProject error:', e);
    }
  }

  const filteredChapters = $derived(
    chaptersFilter
      ? (project as any)?.chapters?.filter((c: any) =>
          c.title.toLowerCase().includes(chaptersFilter.toLowerCase())
        ) ?? []
      : (project as any)?.chapters ?? []
  );
</script>

<aside
  class="preview-sidebar"
  class:collapsed
  aria-label={t('preview.sidebarLabel')}
>
  <!-- Collapse toggle -->
  <button
    class="sidebar-collapse-btn"
    onclick={onToggleCollapse}
    aria-label={collapsed ? t('preview.expandSidebar') : t('preview.collapseSidebar')}
    title={collapsed ? t('preview.expandSidebar') : t('preview.collapseSidebar')}
  >
    {collapsed ? '›' : '‹'}
  </button>

  {#if !collapsed}
    <!-- Tab header -->
    <div class="sidebar-tabs" role="tablist">
      <button
        role="tab"
        class="sidebar-tab"
        class:active={activeTab === 'chapters'}
        aria-selected={activeTab === 'chapters'}
        onclick={() => (activeTab = 'chapters')}
        title={t('preview.chaptersTab')}
      >
        📖
        <span class="tab-label">{t('preview.chapters')}</span>
      </button>
      <button
        role="tab"
        class="sidebar-tab"
        class:active={activeTab === 'gallery'}
        aria-selected={activeTab === 'gallery'}
        onclick={() => (activeTab = 'gallery')}
        title={t('preview.galleryTab')}
      >
        🖼
        <span class="tab-label">{t('preview.gallery')}</span>
      </button>
      <button
        role="tab"
        class="sidebar-tab"
        class:active={activeTab === 'projects'}
        aria-selected={activeTab === 'projects'}
        onclick={() => (activeTab = 'projects')}
        title={t('preview.projectsTab')}
      >
        📁
        <span class="tab-label">{t('preview.projects')}</span>
      </button>
    </div>

    <!-- Tab content -->
    <div class="sidebar-content" role="tabpanel">

      {#if activeTab === 'chapters'}
        <!-- Chapters panel -->
        <div class="panel panel-chapters">
          <div class="panel-search">
            <input
              type="search"
              bind:value={chaptersFilter}
              placeholder={t('preview.searchChapters')}
              aria-label={t('preview.searchChapters')}
              class="search-input"
            />
          </div>

          {#if filteredChapters.length === 0}
            <div class="panel-empty">
              {chaptersFilter ? t('preview.noChaptersFound') : t('preview.noChapters')}
            </div>
          {:else}
            <ul class="chapter-list" role="list">
              {#each filteredChapters as chapter, i}
                <li>
                  <button
                    class="chapter-item"
                    class:current={currentPage === (i + 1)}
                    onclick={() => onNavigate(i + 1)}
                    aria-label="{t('preview.goToChapter')}: {chapter.title}"
                    aria-current={currentPage === (i + 1) ? 'true' : undefined}
                  >
                    <span class="chapter-title">{chapter.title}</span>
                    <span class="chapter-status">
                      {#if chapter.wordCount > 500}✅
                      {:else if chapter.wordCount > 0}⚠️
                      {:else}❌{/if}
                    </span>
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>

      {:else if activeTab === 'gallery'}
        <!-- Illustration gallery panel -->
        <div class="panel panel-gallery">
          {#if !project}
            <div class="panel-empty">{t('preview.openProjectFirst')}</div>
          {:else}
            <div class="panel-empty">{t('preview.noIllustrations')}</div>
          {/if}
        </div>

      {:else}
        <!-- Recent projects panel -->
        <div class="panel panel-projects">
          {#if recentProjects.length === 0}
            <div class="panel-empty">{t('preview.noRecentProjects')}</div>
          {:else}
            <ul class="project-list" role="list">
              {#each recentProjects as proj}
                <li>
                  <button
                    class="project-item"
                    class:current={proj.id === projectId}
                    onclick={() => openProject(proj.id)}
                    aria-label="{t('preview.openProject')}: {proj.name}"
                    aria-current={proj.id === projectId ? 'true' : undefined}
                  >
                    <span class="project-name">{proj.name}</span>
                    {#if proj.id === projectId}
                      <span class="project-badge">{t('preview.current')}</span>
                    {/if}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/if}

    </div>
  {:else}
    <!-- Collapsed: icon-only tabs -->
    <div class="collapsed-tabs">
      <button
        class="collapsed-tab"
        class:active={activeTab === 'chapters'}
        onclick={() => { activeTab = 'chapters'; onToggleCollapse(); }}
        title={t('preview.chapters')}
      >📖</button>
      <button
        class="collapsed-tab"
        class:active={activeTab === 'gallery'}
        onclick={() => { activeTab = 'gallery'; onToggleCollapse(); }}
        title={t('preview.gallery')}
      >🖼</button>
      <button
        class="collapsed-tab"
        class:active={activeTab === 'projects'}
        onclick={() => { activeTab = 'projects'; onToggleCollapse(); }}
        title={t('preview.projects')}
      >📁</button>
    </div>
  {/if}
</aside>

<style>
  .preview-sidebar {
    width: 260px;
    min-width: 260px;
    display: flex;
    flex-direction: column;
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
    height: 100%;
    overflow: hidden;
    transition: width var(--duration-normal), min-width var(--duration-normal);
    position: relative;
    flex-shrink: 0;
  }

  .preview-sidebar.collapsed {
    width: 48px;
    min-width: 48px;
  }

  .sidebar-collapse-btn {
    position: absolute;
    top: var(--space-2);
    right: var(--space-1);
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

  .sidebar-tabs {
    display: flex;
    border-bottom: 1px solid var(--color-border);
    padding-top: var(--space-1);
    padding-right: 24px; /* space for collapse btn */
  }

  .sidebar-tab {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
    padding: var(--space-2) var(--space-1);
    font-size: 16px;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    color: var(--color-text-muted);
    transition: color var(--duration-fast), border-color var(--duration-fast);
  }

  .sidebar-tab:hover { color: var(--color-text); }

  .sidebar-tab.active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .tab-label {
    font-size: var(--text-xs);
    font-weight: 500;
  }

  .sidebar-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .panel {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .panel-search {
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--color-border);
  }

  .search-input {
    width: 100%;
    height: 28px;
    padding: 0 var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--text-sm);
    box-sizing: border-box;
  }

  .panel-empty {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-8) var(--space-4);
    color: var(--color-text-muted);
    font-size: var(--text-sm);
    text-align: center;
  }

  .chapter-list,
  .project-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .chapter-item,
  .project-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: var(--space-2) var(--space-3);
    text-align: left;
    background: none;
    border: none;
    border-bottom: 1px solid var(--color-border);
    cursor: pointer;
    color: var(--color-text);
    font-size: var(--text-sm);
    transition: background var(--duration-fast);
    gap: var(--space-2);
  }

  .chapter-item:hover,
  .project-item:hover {
    background: var(--color-surface-hover);
  }

  .chapter-item.current,
  .project-item.current {
    background: color-mix(in srgb, var(--color-primary) 8%, transparent);
    border-left: 3px solid var(--color-primary);
    padding-left: calc(var(--space-3) - 3px);
  }

  .chapter-title,
  .project-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chapter-status {
    font-size: 12px;
    flex-shrink: 0;
  }

  .project-badge {
    font-size: var(--text-xs);
    background: var(--color-primary);
    color: white;
    padding: 1px 4px;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }

  /* Collapsed state */
  .collapsed-tabs {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: var(--space-8);
    gap: var(--space-2);
  }

  .collapsed-tab {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: 1px solid transparent;
    border-radius: var(--radius-sm);
    cursor: pointer;
    font-size: 16px;
    color: var(--color-text-muted);
    transition: background var(--duration-fast);
  }

  .collapsed-tab:hover,
  .collapsed-tab.active {
    background: var(--color-surface-hover);
    color: var(--color-text);
  }
</style>
