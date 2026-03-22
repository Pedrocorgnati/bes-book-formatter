<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { initPreferences, preferencesStore } from '$lib/stores/preferencesStore';
  import { initLocale, t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { ipcInitDatabase, ipcGetProjects } from '$lib/ipc/projects';
  import AppShell from '$lib/components/layout/AppShell.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import RightPanel from '$lib/components/layout/RightPanel.svelte';
  import SidecarStatus from '$lib/components/startup/SidecarStatus.svelte';
  import ToastContainer from '$lib/components/ui/ToastContainer.svelte';
  import { dev } from '$app/environment';
  import '../app.css';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();

  let loading = $state(true);

  // Breadcrumb derivado da rota atual
  const breadcrumb = $derived(() => {
    const path = $page.url.pathname;
    if (path === '/settings') return t('nav.settings');
    if (path === '/import') return t('nav.import');
    if (path.startsWith('/project/')) {
      // RESOLVED: BookProject title should come from BookConfig loaded separately
      return t('nav.editor');
    }
    return t('nav.dashboard');
  });

  const showImportCta = $derived(
    !$page.url.pathname.startsWith('/import') &&
    !$page.url.pathname.startsWith('/settings')
  );

  // RESOLVED: Async initialization + sync cleanup setup for Svelte onMount constraints
  onMount(() => {
    // Async initialization - fire and forget to avoid returning Promise from callback
    (async () => {
      // Inicializar preferências e locale
      await initPreferences();
      initLocale();

      // Inicializar banco de dados e carregar projetos
      try {
        await ipcInitDatabase();
        const projects = await ipcGetProjects();
        projectsStore.setProjects(projects);
      } catch {
        // TODO: Implementar backend — banco vazio no stub
      }

      loading = false;
    })();

    // Atalhos de teclado globais
    function handleKeydown(e: KeyboardEvent) {
      const meta = e.metaKey || e.ctrlKey;
      if (meta && e.key === ',') {
        e.preventDefault();
        goto('/settings');
      }
      if (meta && e.key === 'i') {
        e.preventDefault();
        goto('/import');
      }
      // Note: Cmd+B to toggle sidebar is now handled by AppShell internally
    }

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

<!-- Skip navigation -->
<a href="#main-content" class="skip-nav">{t('a11y.skipNav')}</a>

<!-- RESOLVED: AppShell manages its own sidebar/panel state internally -->
<AppShell>
  {#snippet banner()}
    <SidecarStatus />
  {/snippet}

  {#snippet header()}
    <Header
      breadcrumb={breadcrumb()}
      showImportCta={showImportCta}
    />
  {/snippet}

  {#snippet sidebar()}
    <Sidebar />
  {/snippet}

  {#snippet main()}
    <main id="main-content" class="layout-main" tabindex="-1">
      {#if loading}
        <div class="layout-main__loading" aria-label={t('common.loading')}>
          <div class="spinner" aria-hidden="true"></div>
        </div>
      {:else}
        {@render children?.()}
      {/if}
    </main>
  {/snippet}

  {#snippet rightPanel()}
    <RightPanel />
  {/snippet}
</AppShell>

<!-- Toast notifications -->
<ToastContainer />

<!-- Dev-only: data-testid overlay -->
{#if dev}
  {#await import('$lib/components/dev/DataTestOverlay.svelte') then { default: DataTestOverlay }}
    <DataTestOverlay />
  {/await}
{/if}

<style>
  .layout-main {
    flex: 1;
    overflow-y: auto;
    outline: none;
  }

  .layout-main__loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 240px;
  }
</style>
