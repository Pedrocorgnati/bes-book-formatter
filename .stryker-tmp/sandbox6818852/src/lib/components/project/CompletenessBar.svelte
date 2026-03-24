<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { ManuscriptCompleteness } from '$lib/types/enums';

  interface Props {
    level: ManuscriptCompleteness | string | null;
    score: number | null; // 0.0–1.0
  }

  const { level = null, score = null }: Props = $props();

  const percent = $derived(score != null ? Math.round(score * 100) : 0);

  const colorClass = $derived(() => {
    if (level === ManuscriptCompleteness.BLOCKING) return 'completeness-bar--blocking';
    if (level === ManuscriptCompleteness.WARNING) return 'completeness-bar--warning';
    if (level === ManuscriptCompleteness.NORMAL) return 'completeness-bar--normal';
    return 'completeness-bar--unknown';
  });

  const labelKey = $derived(() => {
    if (level === ManuscriptCompleteness.BLOCKING) return 'completeness.blocking';
    if (level === ManuscriptCompleteness.WARNING) return 'completeness.warning';
    if (level === ManuscriptCompleteness.NORMAL) return 'completeness.normal';
    return 'completeness.normal';
  });
</script>

{#if level != null}
  <div
    data-testid="completeness-bar"
    class="completeness-bar {colorClass()}"
    role="progressbar"
    aria-valuenow={percent}
    aria-valuemin={0}
    aria-valuemax={100}
    aria-label={t('completeness.score', { score: String(percent) })}
  >
    <div class="completeness-bar__track">
      <div
        class="completeness-bar__fill"
        style="width: {percent}%"
      ></div>
    </div>
    <span class="completeness-bar__label">
      {t(labelKey())} — {t('completeness.score', { score: String(percent) })}
    </span>
  </div>
{/if}

<style>
  .completeness-bar {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .completeness-bar__track {
    height: 4px;
    background: var(--color-border);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .completeness-bar__fill {
    height: 100%;
    border-radius: var(--radius-full);
    transition: width var(--duration-normal) ease;
  }

  .completeness-bar__label {
    font-size: var(--text-xs);
    font-weight: 500;
  }

  /* Blocking — red */
  .completeness-bar--blocking .completeness-bar__fill {
    background: var(--color-error, #ef4444);
  }
  .completeness-bar--blocking .completeness-bar__label {
    color: var(--color-error, #ef4444);
  }

  /* Warning — amber */
  .completeness-bar--warning .completeness-bar__fill {
    background: var(--color-warning, #f59e0b);
  }
  .completeness-bar--warning .completeness-bar__label {
    color: var(--color-warning, #f59e0b);
  }

  /* Normal — green */
  .completeness-bar--normal .completeness-bar__fill {
    background: var(--color-success, #22c55e);
  }
  .completeness-bar--normal .completeness-bar__label {
    color: var(--color-success, #22c55e);
  }

  /* Unknown / no data */
  .completeness-bar--unknown .completeness-bar__fill {
    background: var(--color-text-muted);
  }
  .completeness-bar--unknown .completeness-bar__label {
    color: var(--color-text-muted);
  }
</style>
