<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { invoke } from '@tauri-apps/api/core';
  import type { StoredGenerationResult, GenerationResult } from '$lib/types';
  import { toastStore } from '$lib/stores/toastStore';

  interface Props {
    projectId: string;
    history: StoredGenerationResult[];
    latestResult?: GenerationResult | null;
    loading?: boolean;
    error?: string | null;
    onRegenerate?: (format: string, platform: string) => void;
    onRetry?: () => void;
  }

  let { projectId, history, latestResult = null, loading = false, error = null, onRegenerate, onRetry }: Props = $props();

  function formatBytes(bytes: number | null): string {
    if (bytes === null) return '—';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
  }

  function formatDate(iso: string): string {
    try {
      return new Date(iso).toLocaleString();
    } catch {
      return iso;
    }
  }

  function statusBadgeClass(status: string): string {
    if (status === 'success') return 'badge--success';
    if (status === 'error') return 'badge--error';
    return 'badge--muted';
  }

  async function openFolder(outputPath: string | null) {
    if (!outputPath) return;
    try {
      // Open the folder in the system file manager
      const folder = outputPath.substring(0, outputPath.lastIndexOf('/'));
      await invoke('plugin:shell|open', { path: folder });
    } catch {
      toastStore.error(t('generation.openFolderError'));
    }
  }

  let epubcheckExpanded = $state<Record<string, boolean>>({});

  function toggleEpubcheck(id: string) {
    epubcheckExpanded = { ...epubcheckExpanded, [id]: !epubcheckExpanded[id] };
  }
</script>

<section data-testid="generation-results" class="gen-results" aria-label={t('generation.history')}>
  <h2 class="gen-results__title">{t('generation.history')}</h2>

  {#if error}
    <div class="gen-results__error" role="alert">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span class="gen-results__error-msg">{error}</span>
      {#if onRetry}
        <button class="btn btn--ghost btn--xs" onclick={onRetry} aria-label={t('common.retry')}>
          {t('common.retry')}
        </button>
      {/if}
    </div>
  {:else if loading}
    <div class="gen-results__loading" role="status">
      <div class="spinner" aria-hidden="true"></div>
      <span>{t('generation.historyLoading')}</span>
    </div>
  {:else if history.length === 0}
    <p class="gen-results__empty" role="status">{t('generation.historyEmpty')}</p>
  {:else}
    <div class="gen-results__table-wrap" role="region" aria-label="Tabela de histórico">
      <table class="gen-results__table">
        <thead>
          <tr>
            <th scope="col">{t('generation.table.format')}</th>
            <th scope="col">{t('generation.table.platform')}</th>
            <th scope="col">{t('generation.table.date')}</th>
            <th scope="col">{t('generation.table.size')}</th>
            <th scope="col">{t('generation.table.status')}</th>
            <th scope="col">{t('generation.table.actions')}</th>
          </tr>
        </thead>
        <tbody>
          {#each history as item (item.id)}
            <tr
              class="gen-results__row"
              class:gen-results__row--latest={latestResult && item.outputPath === latestResult.outputPath}
            >
              <td class="gen-results__format">
                <code>{item.format.toUpperCase()}</code>
              </td>
              <td>{item.platform}</td>
              <td class="gen-results__date">{formatDate(item.createdAt)}</td>
              <td>{formatBytes(item.fileSizeBytes)}</td>
              <td>
                <span class="badge {statusBadgeClass(item.status)}">
                  {t(`generation.status.${item.status}`) ?? item.status}
                </span>
              </td>
              <td class="gen-results__actions">
                {#if item.outputPath}
                  <button
                    class="btn btn--ghost btn--xs"
                    onclick={() => openFolder(item.outputPath)}
                    aria-label="Abrir pasta do arquivo gerado"
                  >
                    📁
                  </button>
                {/if}
                <button
                  class="btn btn--ghost btn--xs"
                  onclick={() => onRegenerate?.(item.format, item.platform)}
                  aria-label="Regenerar este formato"
                >
                  ↻
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</section>

<style>
  .gen-results {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .gen-results__title {
    font-size: var(--text-lg);
    font-weight: 600;
    margin: 0;
  }

  .gen-results__loading {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }

  .gen-results__error {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    background: var(--color-error-subtle, #fef2f2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-md);
    color: var(--color-error, #ef4444);
    font-size: var(--text-sm);
  }

  .gen-results__error-msg {
    flex: 1;
  }

  .gen-results__empty {
    color: var(--color-text-muted);
    font-size: var(--text-sm);
    text-align: center;
    padding: var(--space-8) 0;
  }

  .gen-results__table-wrap {
    overflow-x: auto;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .gen-results__table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--text-sm);
  }

  .gen-results__table th {
    padding: var(--space-2) var(--space-3);
    text-align: left;
    font-weight: 500;
    color: var(--color-text-muted);
    background: var(--color-surface-raised);
    border-bottom: 1px solid var(--color-border);
    white-space: nowrap;
  }

  .gen-results__row td {
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--color-border);
    vertical-align: middle;
  }

  .gen-results__row:last-child td {
    border-bottom: none;
  }

  .gen-results__row--latest {
    background: var(--color-accent-subtle, var(--color-surface-raised));
  }

  .gen-results__format code {
    font-family: var(--font-code);
    font-size: var(--text-xs);
    background: var(--color-border);
    padding: 2px var(--space-2);
    border-radius: var(--radius-sm);
  }

  .gen-results__date {
    white-space: nowrap;
    color: var(--color-text-muted);
    font-size: var(--text-xs);
  }

  .gen-results__actions {
    display: flex;
    gap: var(--space-1);
  }

  .badge {
    display: inline-flex;
    align-items: center;
    padding: 2px var(--space-2);
    border-radius: var(--radius-full);
    font-size: var(--text-xs);
    font-weight: 500;
  }

  .badge--success {
    background: var(--color-success-subtle);
    color: var(--color-success);
  }

  .badge--error {
    background: var(--color-error-subtle);
    color: var(--color-error);
  }

  .badge--muted {
    background: var(--color-border);
    color: var(--color-text-muted);
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
