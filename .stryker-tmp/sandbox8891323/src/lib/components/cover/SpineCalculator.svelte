<script lang="ts">
  import { ipcCalculateSpineWidth } from '$lib/ipc/cover';
  import { toast } from '$lib/stores/toastStore';
  import { t } from '$lib/i18n/engine';

  interface Props {
    projectId: string;
    platform: 'amazon-kdp' | 'ingram' | 'generic';
    paperType: 'white' | 'cream';
    pageCount: number;
    spineWidthMm: number;
    onSpineCalculated?: (mm: number) => void;
  }

  let {
    projectId,
    platform,
    paperType,
    pageCount,
    spineWidthMm,
    onSpineCalculated,
  }: Props = $props();

  let loading = $state(false);

  const platformLabelKeys: Record<string, string> = {
    'amazon-kdp': 'cover.spine.platformKdp',
    ingram: 'cover.spine.platformIngram',
    generic: 'cover.spine.platformGeneric',
  };

  const paperLabelKeys: Record<string, string> = {
    white: 'cover.spine.paperWhite',
    cream: 'cover.spine.paperCream',
  };

  async function recalculate() {
    loading = true;
    try {
      const { result, warnings } = await ipcCalculateSpineWidth(projectId, platform, paperType);
      onSpineCalculated?.(result.spineWidthMm);
      if (warnings.length > 0) {
        toast.warning(warnings[0]);
      }
    } catch (e) {
      toast.error(String(e));
    } finally {
      loading = false;
    }
  }
</script>

<div data-testid="spine-calculator" class="spine-calc">
  <div class="spine-calc__header">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
      <rect x="3" y="3" width="18" height="18" rx="2"/>
      <line x1="9" y1="3" x2="9" y2="21"/>
      <line x1="15" y1="3" x2="15" y2="21"/>
    </svg>
    <span class="spine-calc__title">{t('cover.spine.title')}</span>
  </div>

  <div class="spine-calc__values">
    <div class="spine-calc__stat">
      <span class="spine-calc__label">{t('cover.spine.pages')}</span>
      <span class="spine-calc__value" data-testid="page-count">{pageCount > 0 ? pageCount : '—'}</span>
    </div>
    <div class="spine-calc__stat">
      <span class="spine-calc__label">{t('cover.spine.platform')}</span>
      <span class="spine-calc__value">{t(platformLabelKeys[platform] ?? 'cover.spine.platformGeneric')}</span>
    </div>
    <div class="spine-calc__stat">
      <span class="spine-calc__label">{t('cover.spine.paper')}</span>
      <span class="spine-calc__value">{t(paperLabelKeys[paperType] ?? 'cover.spine.paperWhite')}</span>
    </div>
    <div class="spine-calc__stat spine-calc__stat--highlight">
      <span class="spine-calc__label">{t('cover.spine.width')}</span>
      <span class="spine-calc__value" data-testid="spine-width">
        {spineWidthMm > 0 ? `${spineWidthMm.toFixed(2)} mm` : '—'}
      </span>
    </div>
  </div>

  {#if pageCount === 0}
    <p class="spine-calc__hint">
      {t('cover.spine.hint')}
    </p>
  {/if}

  <button
    data-testid="btn-recalculate-spine"
    class="spine-calc__btn"
    onclick={recalculate}
    disabled={loading}
    aria-busy={loading}
  >
    {#if loading}
      <span class="spinner" aria-hidden="true"></span>
      {t('cover.spine.calculating')}
    {:else}
      {t('cover.spine.recalculate')}
    {/if}
  </button>
</div>

<style>
  .spine-calc {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .spine-calc__header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-text);
  }

  .spine-calc__title {
    font-weight: 600;
  }

  .spine-calc__values {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-2);
  }

  .spine-calc__stat {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .spine-calc__stat--highlight .spine-calc__value {
    color: var(--color-primary);
    font-size: var(--text-lg);
    font-weight: 700;
  }

  .spine-calc__label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .spine-calc__value {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text);
  }

  .spine-calc__hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    background: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
    padding: var(--space-2);
    margin: 0;
  }

  .spine-calc__btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    color: var(--color-text);
    cursor: pointer;
    transition: background-color var(--duration-fast) ease;
  }

  .spine-calc__btn:hover:not(:disabled) {
    background: var(--color-border);
  }

  .spine-calc__btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    width: 12px;
    height: 12px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
