<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';

  interface Props {
    header?: Snippet;
    sidebar?: Snippet;
    main?: Snippet;
    rightPanel?: Snippet;
    loading?: boolean;
    banner?: Snippet;
  }

  let {
    header,
    sidebar,
    main,
    rightPanel,
    loading = false,
    banner
  }: Props = $props();

  // --- Estado dos painéis ---
  let sidebarCollapsed = $state(false);
  let rightPanelCollapsed = $state(false);
  let windowWidth = $state(1440);

  // Auto-colapso em janelas estreitas
  $effect(() => {
    if (windowWidth < 960) {
      sidebarCollapsed = true;
      rightPanelCollapsed = true;
    } else if (windowWidth < 1280) {
      rightPanelCollapsed = true;
    }
  });

  function toggleSidebar() {
    sidebarCollapsed = !sidebarCollapsed;
  }

  function toggleRightPanel() {
    rightPanelCollapsed = !rightPanelCollapsed;
  }

  // Atalhos de teclado (Cmd/Ctrl+B)
  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === 'b' && !e.shiftKey) {
      e.preventDefault();
      toggleSidebar();
    }
    if (mod && e.key === ',') {
      e.preventDefault();
      // Navegar para /settings — evento customizado
      window.dispatchEvent(new CustomEvent('bes:navigate', { detail: '/settings' }));
    }
  }

  function handleResize() {
    windowWidth = window.innerWidth;
  }

  onMount(() => {
    windowWidth = window.innerWidth;
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<div data-testid="app-shell" class="app-shell" class:sidebar-collapsed={sidebarCollapsed} class:panel-collapsed={rightPanelCollapsed}>
  <!-- Skip navigation -->
  <a href="#main-content" class="skip-nav">Ir para o conteúdo</a>

  <!-- Header fixo no topo -->
  <header data-testid="header" class="app-shell__header">
    <!-- Botão collapse sidebar -->
    <button
      data-testid="header-sidebar-toggle-button"
      class="collapse-btn"
      onclick={toggleSidebar}
      aria-label={sidebarCollapsed ? 'Expandir menu' : 'Recolher menu'}
      aria-expanded={!sidebarCollapsed}
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        {#if sidebarCollapsed}
          <path d="M9 18l6-6-6-6"/>
        {:else}
          <path d="M15 18l-6-6 6-6"/>
        {/if}
      </svg>
    </button>

    {#if header}
      {@render header()}
    {/if}

    <!-- Botão collapse painel direito -->
    <button
      data-testid="header-right-panel-toggle-button"
      class="collapse-btn collapse-btn--right"
      onclick={toggleRightPanel}
      aria-label={rightPanelCollapsed ? 'Expandir painel' : 'Recolher painel'}
      aria-expanded={!rightPanelCollapsed}
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        {#if rightPanelCollapsed}
          <path d="M15 18l-6-6 6-6"/>
        {:else}
          <path d="M9 18l6-6-6-6"/>
        {/if}
      </svg>
    </button>
  </header>

  <!-- Banner (sidecar status, warnings) -->
  {#if banner}
    <div data-testid="banner" class="app-shell__banner">
      {@render banner()}
    </div>
  {/if}

  <!-- Corpo: sidebar + main + right panel -->
  <div class="app-shell__body">
    <!-- Sidebar esquerda -->
    <nav
      data-testid="sidebar"
      class="app-shell__sidebar"
      aria-label="Navegação principal"
      aria-hidden={sidebarCollapsed}
    >
      {#if loading}
        <div class="skeleton-container" aria-busy="true">
          <div class="skeleton-bar"></div>
          <div class="skeleton-bar" style="width: 75%"></div>
          <div class="skeleton-bar" style="width: 60%"></div>
          <div class="skeleton-bar" style="width: 80%"></div>
        </div>
      {:else if sidebar}
        {@render sidebar()}
      {/if}
    </nav>

    <!-- Área principal -->
    <main data-testid="main-content" class="app-shell__main" id="main-content" aria-label="Conteúdo principal">
      {#if loading}
        <div class="skeleton-main" aria-busy="true">
          <div class="skeleton-block"></div>
        </div>
      {:else if main}
        {@render main()}
      {/if}
    </main>

    <!-- Painel direito -->
    <aside
      data-testid="right-panel"
      class="app-shell__right-panel"
      aria-label="Painel de propriedades"
      aria-hidden={rightPanelCollapsed}
    >
      {#if rightPanel}
        {@render rightPanel()}
      {/if}
    </aside>
  </div>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-rows: var(--header-height) auto 1fr;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: var(--color-bg);
  }

  .app-shell__header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    height: var(--header-height);
    padding: 0 var(--space-3);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    position: relative;
    z-index: 10;
    transition: background-color var(--duration-fast) ease;
  }

  .app-shell__banner {
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface);
  }

  .app-shell__body {
    display: grid;
    grid-template-columns: var(--sidebar-width) 1fr var(--right-panel-width);
    overflow: hidden;
    height: 100%;
    transition: grid-template-columns var(--duration-normal) ease;
  }

  .app-shell.sidebar-collapsed .app-shell__body {
    grid-template-columns: 0px 1fr var(--right-panel-width);
  }

  .app-shell.panel-collapsed .app-shell__body {
    grid-template-columns: var(--sidebar-width) 1fr 0px;
  }

  .app-shell.sidebar-collapsed.panel-collapsed .app-shell__body {
    grid-template-columns: 0px 1fr 0px;
  }

  .app-shell__sidebar {
    background: var(--color-sidebar-bg);
    border-right: 1px solid var(--color-border);
    overflow-y: auto;
    overflow-x: hidden;
    transition: background-color var(--duration-fast) ease;
  }

  .app-shell__main {
    background: var(--color-bg);
    overflow-y: auto;
    overflow-x: hidden;
    min-width: 0;
    transition: background-color var(--duration-fast) ease;
  }

  .app-shell__right-panel {
    background: var(--color-sidebar-bg);
    border-left: 1px solid var(--color-border);
    overflow-y: auto;
    overflow-x: hidden;
    transition: background-color var(--duration-fast) ease;
  }

  /* Collapse btn */
  .collapse-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color var(--duration-fast) ease, color var(--duration-fast) ease;
  }

  .collapse-btn:hover {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }

  .collapse-btn--right {
    margin-left: auto;
  }

  /* Skeleton loading */
  .skeleton-container {
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .skeleton-bar {
    height: 14px;
    border-radius: var(--radius-sm);
    background: var(--color-skeleton);
    animation: skeleton-pulse 1.5s ease-in-out infinite;
  }

  .skeleton-main {
    padding: var(--space-6);
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .skeleton-block {
    flex: 1;
    border-radius: var(--radius-lg);
    background: var(--color-skeleton);
    animation: skeleton-pulse 1.5s ease-in-out infinite;
    max-height: 300px;
  }
</style>
