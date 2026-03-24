<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { toast } from '$lib/stores/toastStore';
  import { ipcListFonts, ipcDeleteCustomFont } from '$lib/ipc/typography';
  import { typographyStore, typographyLoadingStore } from '$lib/stores/typography';
  import { ipcSetTypographyConfig } from '$lib/ipc/typography';
  import type { FontInfo } from '$lib/types/interfaces';

  interface Props {
    projectId: string;
    onFontSelected?: (fontName: string) => void;
  }

  let { projectId, onFontSelected }: Props = $props();

  let fonts = $state<FontInfo[]>([]);
  let loading = $state(true);
  let loadError = $state<string | null>(null);
  let confirmDelete = $state<string | null>(null);

  const config = $derived($typographyStore);

  const bundledFonts = $derived(fonts.filter(f => f.isBundled));
  const customFonts = $derived(fonts.filter(f => !f.isBundled));
  const sampleText = 'A tipografia define a experiência de leitura.';

  async function loadFonts() {
    if (!projectId) return;
    loading = true;
    loadError = null;
    try {
      fonts = await ipcListFonts(projectId);
    } catch (err) {
      loadError = t('typography.fontsLoadError');
    } finally {
      loading = false;
    }
  }

  async function selectFont(fontName: string) {
    if (!projectId || !config) return;
    typographyLoadingStore.set(true);
    try {
      const updated = await ipcSetTypographyConfig(projectId, {
        fontBody: fontName,
        fontHeading: fontName,
      } as never);
      if (updated) typographyStore.set(updated);
      onFontSelected?.(fontName);
    } catch {
      toast.error(t('typography.saveError'));
    } finally {
      typographyLoadingStore.set(false);
    }
  }

  async function deleteFont(fontName: string) {
    if (!projectId) return;
    loading = true;
    confirmDelete = null;
    try {
      await ipcDeleteCustomFont(projectId, fontName);
      toast.success(`${t('typography.fontDeleted')}: "${fontName}"`);
      // If deleted font was in use, reset to default
      if (config?.fontBody === fontName) {
        await selectFont('EB Garamond');
      }
      await loadFonts();
    } catch (err) {
      toast.error(t('typography.fontDeleteError'));
    } finally {
      loading = false;
    }
  }

  onMount(loadFonts);

  export function refresh() {
    loadFonts();
  }
</script>

<div class="font-catalog" data-testid="font-catalog">
  {#if loadError}
    <div class="catalog-error" role="alert">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span>{loadError}</span>
      <button class="catalog-error__retry" onclick={loadFonts}>{t('common.retry')}</button>
    </div>
  {/if}

  {#if loading}
    <div class="catalog-skeleton" role="status" aria-label={t('common.loading')}>
      <div class="skeleton-block"></div>
      <div class="skeleton-block skeleton-block--short"></div>
      <div class="skeleton-block"></div>
    </div>
  {:else}
    <!-- Fontes incluídas -->
    <section aria-labelledby="bundled-fonts-title">
      <h4 id="bundled-fonts-title" class="catalog-section-title">{t('typography.bundledFonts')}</h4>
      {#if bundledFonts.length === 0}
        <p class="catalog-empty">{t('typography.noFontsFound')}</p>
      {:else}
        <ul class="font-list" role="list">
          {#each bundledFonts as font (font.name)}
            <li class="font-item" class:font-item--active={config?.fontBody === font.name}>
              <button
                class="font-select-btn"
                onclick={() => selectFont(font.name)}
                aria-pressed={config?.fontBody === font.name}
                aria-label={t('typography.ariaSelectFont').replace('{name}', font.name)}
              >
                <span class="font-name">{font.name}</span>
                <span class="font-sample" style="font-family: '{font.name}', serif">
                  {sampleText.slice(0, 30)}
                </span>
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <!-- Suas fontes -->
    <section aria-labelledby="custom-fonts-title">
      <h4 id="custom-fonts-title" class="catalog-section-title">{t('typography.customFontsSection')}</h4>
      {#if customFonts.length === 0}
        <p class="catalog-empty">{t('typography.noCustomFonts')}</p>
      {:else}
        <ul class="font-list" role="list">
          {#each customFonts as font (font.name)}
            <li class="font-item" class:font-item--active={config?.fontBody === font.name}>
              <button
                class="font-select-btn"
                onclick={() => selectFont(font.name)}
                aria-pressed={config?.fontBody === font.name}
                aria-label={t('typography.ariaSelectFont').replace('{name}', font.name)}
              >
                <span class="font-name">{font.name}</span>
                <span class="font-sample" style="font-family: '{font.name}', serif">
                  {sampleText.slice(0, 30)}
                </span>
              </button>
              <button
                class="font-delete-btn"
                onclick={() => (confirmDelete = font.name)}
                aria-label={t('typography.ariaRemoveFont').replace('{name}', font.name)}
                title={t('typography.removeFont')}
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14" aria-hidden="true">
                  <polyline points="3 6 5 6 21 6" />
                  <path d="M19 6l-1 14H6L5 6" />
                  <path d="M10 11v6M14 11v6" />
                  <path d="M9 6V4h6v2" />
                </svg>
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  {/if}

  <!-- Confirm delete dialog -->
  {#if confirmDelete}
    <div class="confirm-overlay" role="dialog" aria-modal="true" aria-labelledby="confirm-title">
      <div class="confirm-card">
        <p id="confirm-title" class="confirm-text">
          {t('typography.confirmDeleteFont')} "<strong>{confirmDelete}</strong>"?
        </p>
        <div class="confirm-actions">
          <button class="btn btn--ghost" onclick={() => (confirmDelete = null)}>
            {t('common.cancel')}
          </button>
          <button class="btn btn--danger" onclick={() => deleteFont(confirmDelete!)}>
            {t('common.delete')}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .font-catalog {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    position: relative;
  }

  .catalog-section-title {
    font-size: 0.6875rem;
    font-weight: 700;
    color: var(--color-text-secondary, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin: 0 0 var(--space-2);
  }

  .catalog-empty {
    font-size: 0.8125rem;
    color: var(--color-text-secondary, #6b7280);
    margin: 0;
    font-style: italic;
  }

  .catalog-error {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) 0.625rem;
    background: var(--color-error-subtle, #fef2f2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-sm, 6px);
    color: var(--color-error, #ef4444);
    font-size: var(--text-xs);
    margin-bottom: var(--space-2);
  }

  .catalog-error span { flex: 1; }

  .catalog-error__retry {
    padding: 0.125rem var(--space-2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: 4px;
    background: transparent;
    color: var(--color-error, #ef4444);
    font-size: 0.6875rem;
    cursor: pointer;
    white-space: nowrap;
  }

  .catalog-error__retry:hover {
    background: var(--color-error, #ef4444);
    color: #fff;
  }

  .catalog-skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-2) 0;
  }

  .skeleton-block {
    height: 2.5rem;
    border-radius: var(--radius-sm, 6px);
    background: var(--color-border-light, #f3f4f6);
    animation: skeleton-pulse 1.2s ease-in-out infinite;
    width: 100%;
  }

  .skeleton-block--short { width: 70%; }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }

  .font-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .font-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    border-radius: var(--radius-sm, 6px);
    border: 1px solid transparent;
    transition: border-color var(--duration-fast), background var(--duration-fast);
  }

  .font-item:hover {
    background: var(--color-surface-secondary, #f9fafb);
    border-color: var(--color-border, #e5e7eb);
  }

  .font-item--active {
    background: rgba(59, 130, 246, 0.06);
    border-color: var(--color-primary, #3b82f6);
  }

  .font-select-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: var(--space-2) 0.625rem;
    background: none;
    border: none;
    cursor: pointer;
    text-align: left;
    min-width: 0;
  }

  .font-name {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #111827);
  }

  .font-sample {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #6b7280);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .font-delete-btn {
    padding: 0.375rem;
    border-radius: 4px;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-text-secondary, #6b7280);
    opacity: 0;
    transition: opacity var(--duration-fast), color var(--duration-fast);
  }

  .font-item:hover .font-delete-btn {
    opacity: 1;
  }

  .font-delete-btn:hover {
    color: var(--color-error, #ef4444);
  }

  /* Confirm dialog */
  .confirm-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0,0,0,0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md, 8px);
    z-index: 10;
  }

  .confirm-card {
    background: var(--color-surface, #fff);
    border-radius: var(--radius-md, 8px);
    padding: 1.25rem;
    box-shadow: 0 8px 24px rgba(0,0,0,0.12);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 280px;
    width: 90%;
  }

  .confirm-text {
    font-size: var(--text-sm);
    color: var(--color-text, #111827);
    margin: 0;
    text-align: center;
  }

  .confirm-actions {
    display: flex;
    gap: var(--space-2);
    justify-content: center;
  }

  .btn {
    padding: 0.4375rem var(--space-4);
    border-radius: var(--radius-sm, 6px);
    font-size: var(--text-sm);
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: background var(--duration-fast);
  }

  .btn--ghost {
    background: var(--color-surface-secondary, #f3f4f6);
    color: var(--color-text, #374151);
  }
  .btn--ghost:hover { background: var(--color-border, #e5e7eb); }

  .btn--danger {
    background: var(--color-error, #ef4444);
    color: #fff;
  }
  .btn--danger:hover { background: #dc2626; }
</style>
