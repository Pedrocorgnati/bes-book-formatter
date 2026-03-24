<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { toast } from '$lib/stores/toastStore';
  import { illustrationStore, illustrationsLoadingStore } from '$lib/stores/illustrationStore';
  import { ipcListIllustrations, ipcUpdateIllustrationAltText } from '$lib/ipc/illustrations';
  import IllustrationCard from './IllustrationCard.svelte';
  import IllustrationDropzone from './IllustrationDropzone.svelte';
  import { IllustrationState } from '$lib/types/enums';
  import { ILLUSTRATION_FILTERS, type IllustrationFilter } from '$lib/constants/ui-tabs';
  import { ALT_TEXT_MIN_LENGTH } from '$lib/constants/timing';
  import type { Illustration } from '$lib/types/interfaces';

  interface Props {
    projectId: string;
  }

  let { projectId }: Props = $props();

  let activeFilter = $state<IllustrationFilter>(ILLUSTRATION_FILTERS.ALL);

  // Alt-text modal state
  let altTextModalIllus = $state<Illustration | null>(null);
  let altTextValue = $state('');
  let altTextSaving = $state(false);

  // Dropzone modal state (per-illustration)
  let dropzoneIllus = $state<Illustration | null>(null);

  const illustrations = $derived($illustrationStore);
  const loading = $derived($illustrationsLoadingStore);
  let loadError = $state<string | null>(null);

  const counts = $derived({
    all:      illustrations.length,
    pending:  illustrations.filter((i) => i.state === IllustrationState.PENDING).length,
    imported: illustrations.filter((i) => i.state === IllustrationState.IMPORTED).length,
    linked:   illustrations.filter((i) => i.state === IllustrationState.LINKED).length,
    error:    illustrations.filter((i) => i.state === IllustrationState.ERROR).length,
  });

  const filteredIllustrations = $derived(
    activeFilter === ILLUSTRATION_FILTERS.ALL
      ? illustrations
      : illustrations.filter((i) => i.state === activeFilter)
  );

  const filters: { key: IllustrationFilter; label: string }[] = [
    { key: ILLUSTRATION_FILTERS.ALL,      label: t('illustrations.filterAll') },
    { key: ILLUSTRATION_FILTERS.PENDING,  label: t('illustrations.filterPending') },
    { key: ILLUSTRATION_FILTERS.IMPORTED, label: t('illustrations.filterImported') },
    { key: ILLUSTRATION_FILTERS.LINKED,   label: t('illustrations.filterLinked') },
    { key: ILLUSTRATION_FILTERS.ERROR,    label: t('illustrations.filterError') },
  ];

  // Load illustrations on mount
  onMount(async () => {
    await loadIllustrations();
  });

  async function loadIllustrations() {
    illustrationsLoadingStore.set(true);
    loadError = null;
    try {
      const list = await ipcListIllustrations(projectId);
      illustrationStore.set(list);
    } catch (err) {
      loadError = t('illustrations.loadError');
      console.error('[IllustrationGallery] load error:', err instanceof Error ? err.message : String(err));
    } finally {
      illustrationsLoadingStore.set(false);
    }
  }

  // ── Alt-text modal ─────────────────────────────────────────────────────────

  function openAltTextModal(id: string) {
    const illus = illustrations.find((i) => i.id === id) ?? null;
    if (!illus) return;
    altTextModalIllus = illus;
    altTextValue = illus.altText ?? '';
  }

  function closeAltTextModal() {
    altTextModalIllus = null;
    altTextValue = '';
  }

  async function saveAltText() {
    if (!altTextModalIllus) return;
    if (altTextValue.trim().length < 10) return;
    altTextSaving = true;
    try {
      const updated = await ipcUpdateIllustrationAltText(altTextModalIllus.id, altTextValue.trim());
      if (updated) {
        illustrationStore.update((list) =>
          list.map((i) => (i.id === updated.id ? updated : i))
        );
        toast.success(t('illustrations.altTextSaved'));
      }
      closeAltTextModal();
    } catch (err) {
      toast.error(t('illustrations.altTextSaveError'));
      console.error('[IllustrationGallery] alt-text save error:', err instanceof Error ? err.message : String(err));
    } finally {
      altTextSaving = false;
    }
  }

  function skipAltText() {
    toast.error(t('illustrations.altTextSkipWarning'));
    closeAltTextModal();
  }

  // ── Dropzone modal ─────────────────────────────────────────────────────────

  function openDropzoneModal(id: string) {
    const illus = illustrations.find((i) => i.id === id) ?? null;
    dropzoneIllus = illus;
  }

  function closeDropzoneModal() {
    dropzoneIllus = null;
  }

  function onIllustrationImported(illustration: Illustration) {
    illustrationStore.update((list) =>
      list.map((i) => (i.id === illustration.id ? illustration : i))
    );
    closeDropzoneModal();
    // Automatically open alt-text modal after import
    altTextModalIllus = illustration;
    altTextValue = '';
  }

  // ── Reprocess ─────────────────────────────────────────────────────────────

  function onReprocess(id: string) {
    openDropzoneModal(id);
  }

  // ── Keyboard trap for modals ───────────────────────────────────────────────
  const altTextSaveEnabled = $derived(altTextValue.trim().length >= ALT_TEXT_MIN_LENGTH);
</script>

<!-- ── Alt-text modal ────────────────────────────────────────────────── -->
{#if altTextModalIllus}
  <div
    class="modal-overlay"
    role="dialog"
    aria-modal="true"
    aria-labelledby="alt-text-modal-title"
  >
    <div class="modal-box">
      <h3 id="alt-text-modal-title" class="modal-title">
        {t('illustrations.altTextModalTitle')}
      </h3>
      <p class="modal-subtitle">
        {altTextModalIllus.placeholderName}
      </p>

      <label for="alt-text-area" class="modal-label">
        {t('illustrations.altTextLabel')}
      </label>
      <textarea
        id="alt-text-area"
        class="modal-textarea"
        rows={4}
        placeholder={t('illustrations.altTextPlaceholder')}
        bind:value={altTextValue}
        aria-describedby="alt-text-hint"
        disabled={altTextSaving}
        maxlength={500}
      ></textarea>
      <p id="alt-text-hint" class="modal-hint">
        {altTextValue.trim().length}/500 {t('illustrations.altTextMinHint')}
      </p>

      <div class="modal-actions">
        <button
          class="btn-skip"
          type="button"
          onclick={skipAltText}
          disabled={altTextSaving}
          aria-label={t('illustrations.altTextSkip')}
        >
          ⚠ {t('illustrations.altTextSkip')}
        </button>
        <button
          class="btn-save"
          type="button"
          onclick={saveAltText}
          disabled={!altTextSaveEnabled || altTextSaving}
          aria-label={t('illustrations.altTextSave')}
        >
          {#if altTextSaving}
            <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" width="14" height="14" aria-hidden="true">
              <path d="M21 12a9 9 0 1 1-6.219-8.56" />
            </svg>
          {/if}
          {t('illustrations.altTextSave')}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- ── Dropzone modal (reprocess) ────────────────────────────────────── -->
{#if dropzoneIllus}
  <div
    class="modal-overlay"
    role="dialog"
    aria-modal="true"
    aria-labelledby="dropzone-modal-title"
  >
    <div class="modal-box">
      <div class="modal-header-row">
        <h3 id="dropzone-modal-title" class="modal-title">
          {t('illustrations.importTitle', { name: dropzoneIllus.placeholderName })}
        </h3>
        <button class="btn-close" onclick={closeDropzoneModal} aria-label={t('common.close')}>✕</button>
      </div>
      <IllustrationDropzone
        illustrationId={dropzoneIllus.id}
        placeholderName={dropzoneIllus.placeholderName}
        {projectId}
        onimported={onIllustrationImported}
        onerror={(msg) => toast.error(msg)}
      />
    </div>
  </div>
{/if}

<!-- ── Main gallery ───────────────────────────────────────────────────── -->
<div class="illustration-gallery" data-testid="illustration-gallery">
  <!-- Persistent error banner -->
  {#if loadError}
    <div class="gallery-error" role="alert">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span>{loadError}</span>
      <button class="gallery-error__retry" onclick={loadIllustrations} aria-label={t('common.retry')}>
        {t('common.retry')}
      </button>
    </div>
  {/if}

  <!-- Filters -->
  <nav class="gallery-filters" aria-label={t('illustrations.filtersLabel')}>
    {#each filters as filter}
      <button
        class="filter-btn"
        class:filter-btn--active={activeFilter === filter.key}
        onclick={() => (activeFilter = filter.key)}
        aria-pressed={activeFilter === filter.key}
        aria-label="{filter.label} ({counts[filter.key]})"
      >
        {filter.label}
        <span class="filter-count">{counts[filter.key]}</span>
      </button>
    {/each}
  </nav>

  <!-- Loading -->
  {#if loading}
    <div class="gallery-status" role="status" aria-live="polite">
      <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="2" width="20" height="20" aria-hidden="true">
        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
      </svg>
      <span>{t('illustrations.loading')}</span>
    </div>

  <!-- Empty state -->
  {:else if filteredIllustrations.length === 0}
    <div class="gallery-empty" role="status">
      <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="1.5" aria-hidden="true">
        <rect x="3" y="3" width="18" height="18" rx="2" />
        <circle cx="8.5" cy="8.5" r="1.5" />
        <polyline points="21 15 16 10 5 21" />
      </svg>
      <p>
        {activeFilter === ILLUSTRATION_FILTERS.ALL
          ? t('illustrations.emptyAll')
          : t('illustrations.emptyFiltered')}
      </p>
    </div>

  <!-- Grid -->
  {:else}
    <div class="gallery-grid" role="list" aria-label={t('illustrations.gridLabel')}>
      {#each filteredIllustrations as illustration (illustration.id)}
        <div role="listitem">
          <IllustrationCard
            {illustration}
            onEditAltText={openAltTextModal}
            onReprocess={onReprocess}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .illustration-gallery {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  /* Error banner */
  .gallery-error {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 0.625rem 0.75rem;
    background: var(--color-error-subtle, #fef2f2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-sm, 6px);
    color: var(--color-error, #ef4444);
    font-size: 0.8125rem;
  }

  .gallery-error span { flex: 1; }

  .gallery-error__retry {
    padding: var(--space-1) 0.625rem;
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-sm, 6px);
    background: transparent;
    color: var(--color-error, #ef4444);
    font-size: var(--text-xs);
    cursor: pointer;
    white-space: nowrap;
  }

  .gallery-error__retry:hover {
    background: var(--color-error, #ef4444);
    color: #fff;
  }

  /* Filters */
  .gallery-filters {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
  }

  .filter-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.375rem 0.75rem;
    border-radius: 9999px;
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid var(--color-border, #e5e7eb);
    background: var(--color-surface, #fff);
    color: var(--color-text-secondary, #6b7280);
    transition: all var(--duration-fast);
  }

  .filter-btn:hover {
    background: var(--color-surface-secondary, #f9fafb);
    color: var(--color-text, #374151);
  }

  .filter-btn--active {
    background: var(--color-primary, #3b82f6);
    border-color: var(--color-primary, #3b82f6);
    color: #fff;
  }

  .filter-count { font-size: 0.6875rem; opacity: 0.8; }

  /* Status / empty */
  .gallery-status, .gallery-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    text-align: center;
    color: var(--color-text-secondary, #6b7280);
    gap: var(--space-3);
  }

  .gallery-status {
    flex-direction: row;
    padding: var(--space-8);
    font-size: var(--text-sm);
  }

  .gallery-empty p { font-size: var(--text-sm); margin: 0; }

  /* Grid */
  .gallery-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: var(--space-4);
  }

  /* Spinner */
  @keyframes spin { to { transform: rotate(360deg); } }
  .spinner { animation: spin 0.8s linear infinite; }

  /* Modal overlay */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 900;
  }

  .modal-box {
    background: var(--color-surface, #fff);
    border-radius: var(--radius-md, 8px);
    padding: var(--space-6);
    max-width: 420px;
    width: 90%;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.16);
  }

  .modal-header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-4);
  }

  .modal-title {
    font-size: var(--text-base);
    font-weight: 600;
    margin: 0;
    color: var(--color-text, #111827);
  }

  .modal-subtitle {
    font-size: 0.8125rem;
    color: var(--color-text-secondary, #6b7280);
    margin: 0 0 var(--space-4);
  }

  .modal-label {
    display: block;
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text, #374151);
    margin-bottom: 0.375rem;
  }

  .modal-textarea {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: var(--radius-sm, 6px);
    font-size: var(--text-sm);
    resize: vertical;
    background: var(--color-surface, #fff);
    color: var(--color-text, #111827);
    box-sizing: border-box;
  }

  .modal-textarea:focus {
    outline: none;
    border-color: var(--color-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.12);
  }

  .modal-hint {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #9ca3af);
    margin: var(--space-1) 0 var(--space-4);
  }

  .modal-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-2);
  }

  .btn-skip {
    font-size: 0.8125rem;
    color: #d97706;
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.375rem 0;
  }

  .btn-save {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.4375rem 1.25rem;
    border-radius: var(--radius-sm, 6px);
    font-size: var(--text-sm);
    font-weight: 500;
    border: none;
    background: var(--color-primary, #3b82f6);
    color: #fff;
    cursor: pointer;
    transition: background var(--duration-fast);
  }

  .btn-save:hover:not(:disabled) { background: var(--color-primary-hover, #2563eb); }
  .btn-save:disabled { opacity: 0.5; cursor: not-allowed; }

  .btn-close {
    background: transparent;
    border: none;
    font-size: var(--text-base);
    color: var(--color-text-secondary, #6b7280);
    cursor: pointer;
    line-height: 1;
    padding: var(--space-1);
  }
</style>
