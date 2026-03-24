<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import ImportWizard from '$lib/components/project/ImportWizard.svelte';
</script>

<svelte:head>
  <title>BES Book Formatter — {t('wizard.title')}</title>
</svelte:head>

<div data-testid="import-page" class="import-page">
  <div class="import-page__card">
    <h1 data-testid="import-page-title" class="import-page__title">{t('wizard.title')}</h1>
    <ImportWizard />
  </div>
</div>

<style>
  .import-page {
    height: 100%;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: var(--space-8) var(--space-4);
    overflow-y: auto;
  }

  .import-page__card {
    width: 100%;
    max-width: 600px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    padding: var(--space-8);
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .import-page__title {
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }
</style>
