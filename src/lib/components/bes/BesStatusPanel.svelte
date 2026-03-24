<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { ipcValidateBesWorkspace, ipcGetBesMetadata } from '$lib/ipc/bes';
  import type { BesWorkspaceInfo, BesMetadata } from '$lib/types/bes';

  interface Props {
    projectId: string;
    workspacePath: string;
    onValidated?: (info: BesWorkspaceInfo) => void;
  }

  let { projectId, workspacePath, onValidated }: Props = $props();

  type Status = 'idle' | 'loading' | 'valid' | 'warning' | 'error';

  let status = $state<Status>('idle');
  let workspaceInfo = $state<BesWorkspaceInfo | null>(null);
  let errorMessage = $state('');
  let metadata = $state<BesMetadata | null>(null);
  let metadataLoading = $state(false);

  const statusConfig: Record<Exclude<Status, 'idle'>, { icon: string; labelKey: string; cssClass: string }> = {
    loading: { icon: '⏳', labelKey: 'bes.statusPanel.checking', cssClass: 'bes-status--loading' },
    valid:   { icon: '✅', labelKey: 'bes.statusPanel.valid',    cssClass: 'bes-status--valid' },
    warning: { icon: '⚠️', labelKey: 'bes.statusPanel.warning', cssClass: 'bes-status--warning' },
    error:   { icon: '❌', labelKey: 'bes.statusPanel.error',    cssClass: 'bes-status--error' },
  };

  async function validateWorkspace() {
    if (!workspacePath) {
      status = 'error';
      errorMessage = t('bes.statusPanel.workspacePathMissing');
      return;
    }

    status = 'loading';
    errorMessage = '';

    try {
      const info = await ipcValidateBesWorkspace(workspacePath);

      workspaceInfo = info;

      if (info.isValid) {
        status = 'valid';
        // Fetch metadata when workspace is valid
        await fetchMetadata();
      } else if (info.missingFiles.length > 0 && info.detectedFiles.length > 0) {
        status = 'warning';
      } else {
        status = 'error';
      }

      onValidated?.(info);
    } catch (e) {
      status = 'error';
      errorMessage = String(e);
    }
  }

  async function fetchMetadata() {
    metadataLoading = true;
    try {
      metadata = await ipcGetBesMetadata(projectId, workspacePath);
    } catch {
      // Metadata fetch is best-effort — don't break the panel
      metadata = null;
    } finally {
      metadataLoading = false;
    }
  }

  // Validar automaticamente ao montar se workspacePath fornecido
  $effect(() => {
    if (workspacePath) {
      validateWorkspace();
    }
  });
</script>

<div
  data-testid="bes-status-panel"
  class="bes-status {status !== 'idle' ? statusConfig[status]?.cssClass ?? '' : ''}"
  role="status"
  aria-live="polite"
>
  {#if status === 'idle'}
    <span class="bes-status__icon" aria-hidden="true">📁</span>
    <span class="bes-status__label">{t('bes.statusPanel.idle')}</span>
    <button class="bes-status__btn" onclick={validateWorkspace}>
      {t('bes.statusPanel.recheck')}
    </button>
  {:else}
    <span class="bes-status__icon" aria-hidden="true">{statusConfig[status].icon}</span>
    <span class="bes-status__label">{t(statusConfig[status].labelKey)}</span>

    {#if status === 'error' && errorMessage}
      <p class="bes-status__error">{errorMessage}</p>
    {/if}

    {#if workspaceInfo && workspaceInfo.missingFiles.length > 0}
      <ul class="bes-status__missing" aria-label={t('bes.statusPanel.missingFiles')}>
        {#each workspaceInfo.missingFiles as file}
          <li>{file}</li>
        {/each}
      </ul>
    {/if}

    {#if status === 'valid' && metadataLoading}
      <div class="bes-status__meta-loading" role="status">{t('common.loading')}</div>
    {/if}

    {#if status === 'valid' && metadata}
      <dl data-testid="bes-metadata-display" class="bes-status__metadata">
        <dt>{t('bes.metadata.title')}</dt>
        <dd>{metadata.title}</dd>

        <dt>{t('bes.metadata.author')}</dt>
        <dd>{metadata.author}</dd>

        <dt>{t('bes.metadata.genre')}</dt>
        <dd>{metadata.genre}</dd>

        {#if metadata.language}
          <dt>{t('bes.metadata.language')}</dt>
          <dd>{metadata.language}</dd>
        {/if}

        {#if metadata.isbn}
          <dt>{t('bes.metadata.isbn')}</dt>
          <dd>{metadata.isbn}</dd>
        {/if}

        {#if metadata.publisher}
          <dt>{t('bes.metadata.publisher')}</dt>
          <dd>{metadata.publisher}</dd>
        {/if}
      </dl>
    {/if}

    {#if status !== 'loading'}
      <button
        class="bes-status__btn bes-status__btn--refresh"
        onclick={validateWorkspace}
        aria-label={t('bes.statusPanel.recheck')}
      >
        🔄 {t('bes.statusPanel.recheck')}
      </button>
    {/if}
  {/if}
</div>

<style>
  .bes-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    border: 1px solid var(--color-border);
    background: var(--color-surface);
    font-size: 0.875rem;
    flex-wrap: wrap;
  }

  .bes-status--loading { border-color: var(--color-info, #3b82f6); background: color-mix(in srgb, var(--color-info, #3b82f6) 8%, transparent); }
  .bes-status--valid   { border-color: var(--color-success, #22c55e); background: color-mix(in srgb, var(--color-success, #22c55e) 8%, transparent); }
  .bes-status--warning { border-color: var(--color-warning, #f59e0b); background: color-mix(in srgb, var(--color-warning, #f59e0b) 8%, transparent); }
  .bes-status--error   { border-color: var(--color-danger, #ef4444);  background: color-mix(in srgb, var(--color-danger, #ef4444) 8%, transparent); }

  .bes-status__icon  { font-size: 1.125rem; }
  .bes-status__label { font-weight: 500; flex: 1; }
  .bes-status__error { width: 100%; margin: 0.25rem 0 0; color: var(--color-danger, #ef4444); font-size: 0.8125rem; }

  .bes-status__missing {
    width: 100%;
    margin: 0.25rem 0 0;
    padding: 0 0 0 1rem;
    font-size: 0.8125rem;
    color: var(--color-warning, #f59e0b);
    list-style: disc;
  }

  .bes-status__metadata {
    width: 100%;
    margin: 0.5rem 0 0;
    padding: 0.75rem;
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 0.25rem 0.75rem;
    font-size: 0.8125rem;
    background: color-mix(in srgb, var(--color-success, #22c55e) 4%, transparent);
    border-radius: 0.375rem;
  }

  .bes-status__metadata dt {
    font-weight: 600;
    color: var(--color-text-secondary, #64748b);
  }

  .bes-status__metadata dd {
    margin: 0;
    color: var(--color-text, #1e293b);
  }

  .bes-status__meta-loading {
    width: 100%;
    font-size: 0.8125rem;
    color: var(--color-text-secondary, #64748b);
    margin-top: 0.25rem;
  }

  .bes-status__btn {
    padding: 0.25rem 0.625rem;
    border-radius: 0.375rem;
    border: 1px solid currentColor;
    background: transparent;
    cursor: pointer;
    font-size: 0.8125rem;
    opacity: 0.8;
    transition: opacity var(--duration-fast);
  }
  .bes-status__btn:hover { opacity: 1; }
  .bes-status__btn--refresh { margin-left: auto; }
</style>
