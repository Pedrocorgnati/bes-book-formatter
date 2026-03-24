<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import type { PreflightResult, ChecklistItem } from '$lib/types';

  interface Props {
    result: PreflightResult | null;
    loading?: boolean;
    onGenerate?: () => void;
    onRecheck?: () => void;
  }

  let { result, loading = false, onGenerate, onRecheck }: Props = $props();

  const canGenerate = $derived(result !== null && result.passed);

  let expandedBlockers = $state(false);
  let expandedWarnings = $state(false);
</script>

<div data-testid="preflight-checklist" class="preflight">
  {#if loading}
    <div class="preflight__loading" role="status" aria-label="Verificando pré-condições">
      <div class="spinner" aria-hidden="true"></div>
      <span>{t('generation.btnRunPreflight')}…</span>
    </div>
  {:else if result === null}
    <div class="preflight__empty">
      <button class="btn btn--secondary" onclick={onRecheck}>
        {t('generation.btnRunPreflight')}
      </button>
    </div>
  {:else}
    <div class="preflight__header">
      {#if result.passed}
        <span class="badge badge--success" aria-label={t('generation.preflightPassed')}>
          ✅ {t('generation.preflightPassed')}
        </span>
      {:else}
        <span class="badge badge--error" aria-label={t('generation.preflightFailed')}>
          ❌ {t('generation.preflightFailed')}
        </span>
      {/if}
    </div>

    {#if result.blockers.length > 0}
      <section class="preflight__section preflight__section--error">
        <button
          class="preflight__section-toggle"
          aria-expanded={expandedBlockers}
          onclick={() => (expandedBlockers = !expandedBlockers)}
        >
          <span class="preflight__section-icon">❌</span>
          <span class="preflight__section-title">{t('generation.blockers')} ({result.blockers.length})</span>
          <span class="preflight__chevron">{expandedBlockers ? '▲' : '▼'}</span>
        </button>
        {#if expandedBlockers}
          <ul class="preflight__items" role="list">
            {#each result.blockers as item (item.id)}
              <li class="preflight__item preflight__item--error">
                <span class="preflight__item-id">{item.id}</span>
                <span class="preflight__item-msg">{item.message}</span>
                {#if item.files && item.files.length > 0}
                  <ul class="preflight__files" aria-label="Arquivos afetados">
                    {#each item.files as f}
                      <li class="preflight__file">{f}</li>
                    {/each}
                  </ul>
                {/if}
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {/if}

    {#if result.warnings.length > 0}
      <section class="preflight__section preflight__section--warn">
        <button
          class="preflight__section-toggle"
          aria-expanded={expandedWarnings}
          onclick={() => (expandedWarnings = !expandedWarnings)}
        >
          <span class="preflight__section-icon">⚠️</span>
          <span class="preflight__section-title">{t('generation.warnings')} ({result.warnings.length})</span>
          <span class="preflight__chevron">{expandedWarnings ? '▲' : '▼'}</span>
        </button>
        {#if expandedWarnings}
          <ul class="preflight__items" role="list">
            {#each result.warnings as item (item.id)}
              <li class="preflight__item preflight__item--warn">
                <span class="preflight__item-id">{item.id}</span>
                <span class="preflight__item-msg">{item.message}</span>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {/if}

    <div class="preflight__actions">
      <button class="btn btn--ghost btn--sm" onclick={onRecheck}>
        ↻ {t('generation.recheck')}
      </button>
      <button
        class="btn btn--primary"
        disabled={!canGenerate}
        aria-disabled={!canGenerate}
        onclick={onGenerate}
      >
        {t('generation.btnGenerate')}
      </button>
    </div>
  {/if}
</div>

<style>
  .preflight {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .preflight__loading {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    color: var(--color-text-muted);
    font-size: var(--text-sm);
  }

  .preflight__empty {
    display: flex;
    justify-content: flex-start;
  }

  .preflight__header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .badge {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-full);
    font-size: var(--text-sm);
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

  .preflight__section {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .preflight__section--error {
    border-color: var(--color-error);
  }

  .preflight__section--warn {
    border-color: var(--color-warning);
  }

  .preflight__section-toggle {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    padding: var(--space-2) var(--space-3);
    background: none;
    border: none;
    cursor: pointer;
    font-size: var(--text-sm);
    font-weight: 500;
    text-align: left;
  }

  .preflight__section-title {
    flex: 1;
  }

  .preflight__items {
    list-style: none;
    margin: 0;
    padding: var(--space-2) var(--space-3) var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    border-top: 1px solid var(--color-border);
  }

  .preflight__item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    font-size: var(--text-sm);
  }

  .preflight__item-id {
    font-family: var(--font-code);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .preflight__files {
    list-style: none;
    padding-left: var(--space-4);
    margin: 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .preflight__actions {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--space-3);
    margin-top: var(--space-2);
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
