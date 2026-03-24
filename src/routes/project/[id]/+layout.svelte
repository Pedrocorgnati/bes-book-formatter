<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import type { Snippet } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { toastStore } from '$lib/stores/toastStore';
  import { onMount } from 'svelte';
  import { ipcGetProject } from '$lib/ipc/projects';
  import { ROUTES, PROJECT_ROUTES } from '$lib/constants/routes';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();

  const projectId = $derived($page.params.id);
  const currentProject = $derived($projectsStore.current);
  const currentPath = $derived($page.url.pathname);

  const tabs = $derived([
    {
      href: PROJECT_ROUTES.ROOT(projectId),
      label: t('nav.editor'),
      exact: true,
    },
    {
      href: PROJECT_ROUTES.TYPOGRAPHY(projectId),
      label: t('nav.typography'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.ILLUSTRATIONS(projectId),
      label: t('nav.illustrations'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.OUTPUT(projectId),
      label: t('nav.output'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.PREVIEW(projectId),
      label: t('nav.preview'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.COVER(projectId),
      label: t('nav.cover'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.INTEGRATION(projectId),
      label: t('nav.integration'),
      exact: false,
    },
    {
      href: PROJECT_ROUTES.SETTINGS(projectId),
      label: t('nav.settings'),
      exact: false,
    },
  ]);

  function isActive(tab: { href: string; exact: boolean }) {
    if (tab.exact) return currentPath === tab.href;
    return currentPath.startsWith(tab.href);
  }

  onMount(async () => {
    // Tentar carregar projeto pelo id
    if (!projectId) {
      goto(ROUTES.HOME);
      return;
    }
    try {
      const project = await ipcGetProject(projectId);
      if (project) {
        projectsStore.setCurrent(project);
      } else {
        // Projeto não encontrado — redirecionar ao dashboard
        goto(ROUTES.HOME);
      }
    } catch (e) {
      toastStore.error(t('errors.projectNotFound'));
      goto(ROUTES.HOME);
    }
  });
</script>

<div data-testid="project-layout" class="project-layout">
  <!-- Tab bar de navegação do projeto -->
  <nav data-testid="project-tabs" class="project-tabs" aria-label={t('a11y.projectNav')}>
    {#each tabs as tab}
      <a
        data-testid="project-tab-{tab.href.split('/').pop()}"
        href={tab.href}
        class="project-tab"
        class:project-tab--active={isActive(tab)}
        aria-current={isActive(tab) ? 'page' : undefined}
      >
        {tab.label}
      </a>
    {/each}
  </nav>

  <!-- Conteúdo da sub-rota -->
  <div data-testid="project-content" class="project-content">
    {@render children?.()}
  </div>
</div>

<style>
  .project-layout {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .project-tabs {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-surface);
    padding: 0 var(--space-4);
    overflow-x: auto;
    flex-shrink: 0;
  }

  .project-tab {
    display: inline-flex;
    align-items: center;
    padding: var(--space-3) var(--space-4);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
    text-decoration: none;
    border-bottom: 2px solid transparent;
    transition:
      color var(--duration-fast) ease,
      border-color var(--duration-fast) ease;
    white-space: nowrap;
  }

  .project-tab:hover {
    color: var(--color-text);
  }

  .project-tab--active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .project-tab:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: -2px;
  }

  .project-content {
    flex: 1;
    overflow-y: auto;
  }
</style>
