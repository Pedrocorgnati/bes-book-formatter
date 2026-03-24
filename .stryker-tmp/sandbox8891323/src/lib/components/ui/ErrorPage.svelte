<script lang="ts">
  import { t } from '$lib/i18n/engine';

  interface Props {
    code?: number;
    title?: string;
    message?: string;
    showBackButton?: boolean;
    backHref?: string;
    backLabel?: string;
    showReloadButton?: boolean;
  }

  let {
    code,
    title,
    message,
    showBackButton = true,
    backHref = '/',
    backLabel,
    showReloadButton = false
  }: Props = $props();

  const displayTitle = $derived(title ?? (code === 404 ? t('errors.notFound') : t('errors.generic')));
  const displayMessage = $derived(message ?? (code === 404 ? t('errors.notFoundDesc') : t('errors.genericDesc')));
  const displayBackLabel = $derived(backLabel ?? t('nav.backToDashboard'));

  function handleReload() {
    if (typeof window !== 'undefined') {
      window.location.reload();
    }
  }
</script>

<div data-testid="error-page" class="error-page" role="alert">
  <div class="error-page__icon" aria-hidden="true">
    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
      <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
      <line x1="12" y1="9" x2="12" y2="13"/>
      <line x1="12" y1="17" x2="12.01" y2="17"/>
    </svg>
  </div>

  <!-- svelte-ignore a11y_autofocus -->
  <h1 class="error-page__title" tabindex="-1" autofocus>
    {displayTitle}
  </h1>

  {#if code}
    <p class="error-page__code">{t('errors.code', { code: String(code) })}</p>
  {/if}

  <p class="error-page__desc">
    {displayMessage}
  </p>

  <div class="error-page__actions">
    {#if showBackButton}
      <a data-testid="error-back-link" href={backHref} class="btn btn--primary">
        {displayBackLabel}
      </a>
    {/if}

    {#if showReloadButton}
      <button data-testid="error-reload-btn" class="btn btn--secondary" onclick={handleReload}>
        {t('common.reload')}
      </button>
    {/if}
  </div>
</div>

<style>
  .error-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--space-16) var(--space-8);
    gap: var(--space-4);
    min-height: 60vh;
  }

  .error-page__icon {
    color: var(--color-text-muted);
    opacity: 0.6;
    margin-bottom: var(--space-2);
  }

  .error-page__title {
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
    outline: none;
  }

  .error-page__code {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
    font-family: var(--font-mono);
  }

  .error-page__desc {
    font-size: var(--text-base);
    color: var(--color-text-secondary);
    max-width: 400px;
    line-height: 1.6;
    margin: 0;
  }

  .error-page__actions {
    display: flex;
    gap: var(--space-3);
    margin-top: var(--space-2);
  }
</style>
