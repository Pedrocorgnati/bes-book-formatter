<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore, recentProjectsStore } from '$lib/stores/projectStore';
  import { ROUTES, PROJECT_ROUTES } from '$lib/constants/routes';

  const currentProject = $derived($projectsStore.current);
  const recentProjects = $derived($recentProjectsStore);
  const loading = $derived($projectsStore.loading);
  const currentPath = $derived($page.url.pathname);

  function isActive(path: string): boolean {
    return currentPath === path || currentPath.startsWith(path + '/');
  }

  function handleImport() {
    // TODO: Rock-1 implementa lógica real de importação
    goto(ROUTES.IMPORT);
  }
</script>

<div data-testid="sidebar-content" class="sidebar">
  <!-- Seção: Projetos Recentes -->
  <section data-testid="sidebar-recent-projects" class="sidebar__section">
    <h2 class="sidebar__section-title">{t('nav.recentProjects')}</h2>

    {#if loading}
      <div class="sidebar__skeleton" aria-busy="true" aria-label={t('common.loading')}>
        <div class="skeleton-item"></div>
        <div class="skeleton-item" style="width: 80%"></div>
        <div class="skeleton-item" style="width: 65%"></div>
      </div>
    {:else if recentProjects.length === 0}
      <p class="sidebar__empty-msg">{t('emptyState.noProjectSelected')}</p>
    {:else}
      <ul data-testid="sidebar-project-list" class="sidebar__project-list" aria-label={t('nav.recentProjects')}>
        {#each recentProjects as proj (proj.id)}
          <li>
            <button
              data-testid="sidebar-project-item-{proj.id}"
              class="sidebar__project-item"
              class:sidebar__item--active={currentProject?.id === proj.id}
              onclick={() => goto(PROJECT_ROUTES.ROOT(proj.id))}
              aria-current={currentProject?.id === proj.id ? 'page' : undefined}
            >
              <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
              </svg>
              <span class="sidebar__project-name">{proj.name}</span>
            </button>
          </li>
        {/each}
      </ul>
    {/if}

    <!-- Botão Novo Projeto -->
    <button data-testid="sidebar-new-project-button" class="sidebar__new-btn" onclick={handleImport} aria-label={t('emptyState.importCta')}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
        <line x1="12" y1="5" x2="12" y2="19"/>
        <line x1="5" y1="12" x2="19" y2="12"/>
      </svg>
      {t('nav.newProject')}
    </button>
  </section>

  <!-- Divisor -->
  <hr class="sidebar__divider" />

  <!-- Seção: Sub-rotas do projeto ativo (desabilitadas se sem projeto) -->
  <section class="sidebar__section">
    <nav aria-label={t('a11y.projectNav')}>
      <ul data-testid="sidebar-nav-list" class="sidebar__nav-list">
        <!-- Tipografia — Rock-2 -->
        <li>
          <button
            data-testid="sidebar-nav-item-typography"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/typography`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.TYPOGRAPHY(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.typography')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <polyline points="4 7 4 4 20 4 20 7"/>
              <line x1="9" y1="20" x2="15" y2="20"/>
              <line x1="12" y1="4" x2="12" y2="20"/>
            </svg>
            <span>{t('nav.typography')}</span>
            {#if !currentProject}
              <span class="badge-soon">{t('nav.openProjectFirst')}</span>
            {/if}
          </button>
        </li>

        <!-- Ilustrações — Rock-2 -->
        <li>
          <button
            data-testid="sidebar-nav-item-illustrations"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/illustrations`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.ILLUSTRATIONS(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.illustrations')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <circle cx="8.5" cy="8.5" r="1.5"/>
              <polyline points="21 15 16 10 5 21"/>
            </svg>
            <span>{t('nav.illustrations')}</span>
          </button>
        </li>

        <!-- Gerar — Rock-3 -->
        <li>
          <button
            data-testid="sidebar-nav-item-output"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/output`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.OUTPUT(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.generate')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            <span>{t('nav.generate')}</span>
          </button>
        </li>

        <!-- Preview — Rock-4 -->
        <li>
          <button
            data-testid="sidebar-nav-item-preview"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/preview`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.PREVIEW(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.preview')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
              <circle cx="12" cy="12" r="3"/>
            </svg>
            <span>{t('nav.preview')}</span>
          </button>
        </li>

        <!-- Capa — Rock-6 -->
        <li>
          <button
            data-testid="sidebar-nav-item-cover"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/cover`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.COVER(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.cover')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <line x1="9" y1="3" x2="9" y2="21"/>
            </svg>
            <span>{t('nav.cover')}</span>
          </button>
        </li>

        <!-- Integração BES — Rock-5 -->
        <li>
          <button
            data-testid="sidebar-nav-item-integration"
            class="sidebar__nav-item"
            class:sidebar__item--active={currentProject && isActive(`/project/${currentProject.id}/integration`)}
            class:sidebar__item--disabled={!currentProject}
            onclick={() => currentProject && goto(PROJECT_ROUTES.INTEGRATION(currentProject.id))}
            disabled={!currentProject || undefined}
            aria-disabled={!currentProject}
            title={!currentProject ? t('nav.openProjectFirst') : t('nav.integration')}
          >
            <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
            </svg>
            <span>{t('nav.integration')}</span>
          </button>
        </li>
      </ul>
    </nav>
  </section>

  <!-- Divisor -->
  <hr class="sidebar__divider" />

  <!-- Marketplace (desabilitado no Skeleton) -->
  <section class="sidebar__section">
    <ul class="sidebar__nav-list">
      <li>
        <button
          data-testid="sidebar-nav-item-marketplace"
          class="sidebar__nav-item sidebar__item--disabled"
          disabled
          aria-disabled="true"
          title={t('nav.marketplaceCombined')}
        >
          <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"/>
            <line x1="3" y1="6" x2="21" y2="6"/>
            <path d="M16 10a4 4 0 0 1-8 0"/>
          </svg>
          <span>{t('nav.marketplace')}</span>
          <span class="badge-soon" aria-label={t('nav.marketplaceSoon')}>{t('nav.marketplaceSoon')}</span>
        </button>
      </li>
    </ul>
  </section>

  <!-- Footer: Settings -->
  <div data-testid="sidebar-footer" class="sidebar__footer">
    <button
      data-testid="sidebar-nav-item-settings"
      class="sidebar__nav-item"
      class:sidebar__item--active={isActive('/settings')}
      onclick={() => goto(ROUTES.SETTINGS)}
      aria-label={t('nav.goToSettings')}
    >
      <svg class="sidebar__icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
      <span>{t('nav.settings')}</span>
    </button>
  </div>
</div>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: var(--space-3) 0;
    overflow-y: auto;
  }

  .sidebar__section {
    padding: 0 var(--space-3);
    margin-bottom: var(--space-2);
  }

  .sidebar__section-title {
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: var(--space-2) var(--space-1);
    margin-bottom: var(--space-1);
  }

  .sidebar__empty-msg {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    padding: var(--space-2) var(--space-1);
    font-style: italic;
  }

  .sidebar__divider {
    border: none;
    border-top: 1px solid var(--color-border);
    margin: var(--space-2) var(--space-3);
  }

  .sidebar__project-list,
  .sidebar__nav-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  /* Item base */
  .sidebar__nav-item,
  .sidebar__project-item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    padding: var(--space-2) var(--space-2);
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-size: var(--text-sm);
    color: var(--color-text);
    text-align: left;
    cursor: pointer;
    transition:
      background-color var(--duration-fast) ease,
      color var(--duration-fast) ease;
    position: relative;
  }

  .sidebar__nav-item:hover,
  .sidebar__project-item:hover {
    background: var(--color-bg);
    color: var(--color-text);
  }

  /* Item ativo */
  .sidebar__item--active {
    background: var(--color-bg);
    color: var(--color-primary);
    border-left: 2px solid var(--color-primary);
    padding-left: calc(var(--space-2) - 2px);
    font-weight: 500;
  }

  /* Item desabilitado — cobre aria-disabled visual e disabled nativo */
  .sidebar__item--disabled,
  .sidebar__nav-item:disabled,
  .sidebar__project-item:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .sidebar__icon {
    flex-shrink: 0;
    color: inherit;
  }

  .sidebar__project-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
    min-width: 0;
  }

  .badge-soon {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    background: var(--color-bg-secondary);
    padding: 1px var(--space-1);
    border-radius: var(--radius-sm);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .sidebar__new-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    margin-top: var(--space-2);
    padding: var(--space-2) var(--space-2);
    background: transparent;
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-md);
    font-family: var(--font-sans);
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-primary);
    cursor: pointer;
    transition: background-color var(--duration-fast) ease;
  }

  .sidebar__new-btn:hover {
    background: var(--color-accent);
  }

  .sidebar__footer {
    margin-top: auto;
    padding: var(--space-2) var(--space-3);
    border-top: 1px solid var(--color-border);
  }

  /* Skeleton */
  .sidebar__skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-1);
  }

  .skeleton-item {
    height: 28px;
    border-radius: var(--radius-md);
    background: var(--color-skeleton);
    animation: skeleton-pulse 1.5s ease-in-out infinite;
  }
</style>
