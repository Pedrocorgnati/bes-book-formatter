<script lang="ts">
  import { page } from '$app/stores';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { goto } from '$app/navigation';
  import BesStatusPanel from '$lib/components/bes/BesStatusPanel.svelte';
  import EditorialProgressBar from '$lib/components/bes/EditorialProgressBar.svelte';

  type SettingsTab = 'integration' | 'progress';

  const projectId = $derived($page.params.id ?? '');
  const currentProject = $derived($projectsStore.current);
  const workspacePath = $derived(currentProject?.besRootPath ?? '');
  const projectName = $derived(currentProject?.name ?? '');

  const activeTab = $derived<SettingsTab>(
    ($page.url.searchParams.get('tab') as SettingsTab) === 'progress'
      ? 'progress'
      : 'integration'
  );

  function switchTab(tab: SettingsTab) {
    const url = new URL($page.url);
    url.searchParams.set('tab', tab);
    goto(url.toString(), { replaceState: true, noScroll: true });
  }
</script>

<svelte:head>
  <title>{currentProject?.name ?? t('settings.title')} — {t('settings.title')} | BES Book Formatter</title>
</svelte:head>

<div data-testid="project-settings-page" class="settings-page">
  <header class="settings-header">
    <h2 class="settings-title">{t('settings.title')}</h2>
  </header>

  <nav data-testid="settings-tabs" class="settings-tabs" aria-label={t('a11y.projectNav')}>
    <button
      data-testid="settings-tab-integration"
      class="settings-tab"
      class:settings-tab--active={activeTab === 'integration'}
      aria-selected={activeTab === 'integration'}
      role="tab"
      onclick={() => switchTab('integration')}
    >
      {t('bes.settings.integrationTab')}
    </button>
    <button
      data-testid="settings-tab-progress"
      class="settings-tab"
      class:settings-tab--active={activeTab === 'progress'}
      aria-selected={activeTab === 'progress'}
      role="tab"
      onclick={() => switchTab('progress')}
    >
      {t('bes.settings.progressTab')}
    </button>
  </nav>

  <section data-testid="settings-content" class="settings-content">
    {#if activeTab === 'integration'}
      <div data-testid="settings-integration-panel">
        <BesStatusPanel
          {projectId}
          workspacePath={workspacePath ?? ''}
        />
      </div>
    {:else if activeTab === 'progress'}
      <div data-testid="settings-progress-panel">
        <EditorialProgressBar
          {projectId}
          workspacePath={workspacePath ?? ''}
          projectName={projectName ?? ''}
        />
      </div>
    {/if}
  </section>
</div>

<style>
  .settings-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-4, 1rem);
    padding: var(--space-6, 1.5rem);
    max-width: 48rem;
    margin: 0 auto;
    width: 100%;
  }

  .settings-header {
    display: flex;
    align-items: center;
    gap: var(--space-2, 0.5rem);
  }

  .settings-title {
    margin: 0;
    font-size: var(--text-lg, 1.125rem);
    font-weight: 600;
    color: var(--color-text, #1e293b);
  }

  .settings-tabs {
    display: flex;
    gap: 0;
    border-bottom: 1px solid var(--color-border, #e2e8f0);
  }

  .settings-tab {
    display: inline-flex;
    align-items: center;
    padding: var(--space-2, 0.5rem) var(--space-4, 1rem);
    font-size: var(--text-sm, 0.875rem);
    font-weight: 500;
    color: var(--color-text-secondary, #64748b);
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition:
      color var(--duration-fast, 0.15s) ease,
      border-color var(--duration-fast, 0.15s) ease;
    white-space: nowrap;
  }

  .settings-tab:hover {
    color: var(--color-text, #1e293b);
  }

  .settings-tab--active {
    color: var(--color-primary, #3b82f6);
    border-bottom-color: var(--color-primary, #3b82f6);
  }

  .settings-tab:focus-visible {
    outline: 2px solid var(--color-primary, #3b82f6);
    outline-offset: -2px;
  }

  .settings-content {
    padding-top: var(--space-2, 0.5rem);
  }
</style>
