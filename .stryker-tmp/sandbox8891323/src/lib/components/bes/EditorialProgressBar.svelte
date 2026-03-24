<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { ipcSyncEditorialProgress } from '$lib/ipc/bes';
  import type { EditorialProgress, PhaseStatus, EditorialStatusType } from '$lib/types/bes';

  interface Props {
    projectId: string;
    workspacePath: string;
    projectName: string;
    /** Chamado após sincronização bem-sucedida */
    onSynced?: (progress: EditorialProgress) => void;
  }

  let { projectId, workspacePath, projectName, onSynced }: Props = $props();

  let progress = $state<EditorialProgress | null>(null);
  let loading = $state(false);
  let syncing = $state(false);
  let error = $state('');

  const statusConfig: Record<EditorialStatusType, { icon: string; label: string; css: string }> = {
    done:        { icon: '✅', label: 'Concluído',    css: 'phase--done' },
    in_progress: { icon: '🔄', label: 'Em andamento', css: 'phase--progress' },
    pending:     { icon: '⬜', label: 'Pendente',     css: 'phase--pending' },
    blocked:     { icon: '🔴', label: 'Bloqueado',    css: 'phase--blocked' },
    skipped:     { icon: '⏭️', label: 'Pulado',       css: 'phase--skipped' },
  };

  function phaseConfig(phase: PhaseStatus) {
    return statusConfig[phase.status] ?? statusConfig.pending;
  }

  async function loadProgress() {
    if (!workspacePath) return;
    loading = true;
    error = '';
    try {
      progress = await ipcSyncEditorialProgress(projectId, workspacePath, projectName);
      onSynced?.(progress);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function syncF10() {
    syncing = true;
    error = '';
    try {
      // Trigger best-effort — não bloqueia se falhar
      progress = await ipcSyncEditorialProgress(projectId, workspacePath, projectName);
      onSynced?.(progress);
    } catch (e) {
      error = String(e);
    } finally {
      syncing = false;
    }
  }

  $effect(() => {
    if (workspacePath && projectId) {
      loadProgress();
    }
  });
</script>

<div data-testid="editorial-progress-bar" class="epb" role="region" aria-label={t('bes.progress.title')}>
  <div class="epb__header">
    <h3 class="epb__title">{t('bes.progress.title')}</h3>
    <button
      class="epb__sync-btn"
      onclick={syncF10}
      disabled={syncing || loading}
      aria-label={t('bes.progress.sync')}
    >
      {#if syncing}
        <span aria-hidden="true">⏳</span> {t('bes.progress.syncing')}
      {:else}
        <span aria-hidden="true">🔄</span> {t('bes.progress.sync')}
      {/if}
    </button>
  </div>

  {#if loading}
    <div class="epb__loading" role="status" aria-live="polite">
      <span aria-hidden="true">⏳</span> {t('bes.progress.loading')}
    </div>
  {:else if error}
    <div class="epb__error" role="alert">
      <span aria-hidden="true">❌</span> {error}
    </div>
  {:else if progress}
    <div class="epb__grid" role="list" aria-label="Fases do pipeline">
      {#each progress.phases as phase (phase.phaseId)}
        {@const cfg = phaseConfig(phase)}
        <div
          class="phase {cfg.css}"
          role="listitem"
          title="{phase.phaseName}{phase.notes ? ` — ${phase.notes}` : ''}"
          aria-label="{phase.phaseId}: {phase.phaseName} — {cfg.label}"
        >
          <span class="phase__id" aria-hidden="true">{phase.phaseId}</span>
          <span class="phase__icon" aria-hidden="true">{cfg.icon}</span>
          <span class="phase__name">{phase.phaseName}</span>
          {#if phase.date}
            <span class="phase__date" aria-label="Data: {phase.date}">{phase.date}</span>
          {/if}
        </div>
      {/each}
    </div>

    <p class="epb__updated">
      {t('bes.progress.updated')}: {new Date(progress.lastUpdated).toLocaleString('pt-BR')}
    </p>
  {:else}
    <p class="epb__empty">{t('bes.progress.empty')}</p>
  {/if}
</div>

<style>
  .epb {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1rem;
    border: 1px solid var(--border, #e2e8f0);
    border-radius: 0.5rem;
    background: var(--surface, #fff);
  }

  .epb__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .epb__title {
    margin: 0;
    font-size: 0.9375rem;
    font-weight: 600;
  }

  .epb__sync-btn {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.625rem;
    border-radius: 0.375rem;
    border: 1px solid var(--border, #e2e8f0);
    background: var(--surface, #fff);
    font-size: 0.8125rem;
    cursor: pointer;
    transition: background 0.15s;
  }
  .epb__sync-btn:hover:not(:disabled) { background: var(--surface-hover, #f8fafc); }
  .epb__sync-btn:disabled { opacity: 0.5; cursor: not-allowed; }

  .epb__grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.5rem;
  }

  .phase {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.125rem;
    padding: 0.5rem 0.25rem;
    border-radius: 0.375rem;
    border: 1px solid var(--border, #e2e8f0);
    text-align: center;
    font-size: 0.75rem;
    transition: transform 0.1s;
    cursor: default;
  }
  .phase:hover { transform: translateY(-1px); }

  .phase--done     { border-color: var(--color-success, #22c55e); background: color-mix(in srgb, var(--color-success, #22c55e) 10%, transparent); }
  .phase--progress { border-color: var(--color-info, #3b82f6);    background: color-mix(in srgb, var(--color-info, #3b82f6) 10%, transparent); }
  .phase--pending  { border-color: var(--border, #e2e8f0); }
  .phase--blocked  { border-color: var(--color-danger, #ef4444);  background: color-mix(in srgb, var(--color-danger, #ef4444) 10%, transparent); }
  .phase--skipped  { border-color: var(--border, #e2e8f0); opacity: 0.5; }

  .phase__id   { font-weight: 700; font-size: 0.6875rem; letter-spacing: 0.025em; color: var(--text-muted, #64748b); }
  .phase__icon { font-size: 1rem; }
  .phase__name { font-weight: 500; line-height: 1.2; }
  .phase__date { font-size: 0.625rem; color: var(--text-muted, #64748b); }

  .epb__loading, .epb__empty {
    text-align: center;
    color: var(--text-muted, #64748b);
    font-size: 0.875rem;
    padding: 1rem 0;
  }

  .epb__error {
    color: var(--color-danger, #ef4444);
    font-size: 0.875rem;
    padding: 0.5rem;
    border-radius: 0.25rem;
    background: color-mix(in srgb, var(--color-danger, #ef4444) 8%, transparent);
  }

  .epb__updated {
    margin: 0;
    font-size: 0.75rem;
    color: var(--text-muted, #64748b);
    text-align: right;
  }

  @media (max-width: 768px) {
    .epb__grid { grid-template-columns: repeat(3, 1fr); }
  }
  @media (max-width: 480px) {
    .epb__grid { grid-template-columns: repeat(2, 1fr); }
  }
</style>
