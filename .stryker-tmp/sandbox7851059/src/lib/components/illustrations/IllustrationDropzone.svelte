<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { toast } from '$lib/stores/toastStore';
  import { ipcValidateIllustrationDpi, ipcProcessIllustration } from '$lib/ipc/typography';
  import type { Illustration, DpiValidation } from '$lib/types/interfaces';

  interface Props {
    illustrationId: string;
    placeholderName?: string;
    projectId: string;
    onimported?: (illustration: Illustration) => void;
    onerror?: (error: string) => void;
    disabled?: boolean;
  }

  let {
    illustrationId,
    placeholderName = '',
    projectId,
    onimported,
    onerror,
    disabled = false,
  }: Props = $props();

  // Accepted MIME types and extensions
  const ALLOWED_EXTS = ['jpg', 'jpeg', 'png', 'svg', 'tiff', 'tif', 'bmp'];
  const ALLOWED_MIME = [
    'image/jpeg',
    'image/png',
    'image/svg+xml',
    'image/tiff',
    'image/bmp',
  ];

  let isDragOver = $state(false);
  let isProcessing = $state(false);
  let confirmDpiData = $state<{ path: string; validation: DpiValidation } | null>(null);

  // ── Drag handlers ────────────────────────────────────────────────────────────

  function onDragOver(e: DragEvent) {
    if (disabled || isProcessing) return;
    e.preventDefault();
    isDragOver = true;
  }

  function onDragLeave() {
    isDragOver = false;
  }

  async function onDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    if (disabled || isProcessing) return;

    const file = e.dataTransfer?.files?.[0];
    if (!file) return;
    await handleFile(file);
  }

  // ── File input (keyboard / accessibility fallback) ────────────────────────

  let fileInputRef: HTMLInputElement | undefined = $state();

  function onClickDropzone() {
    if (disabled || isProcessing) return;
    fileInputRef?.click();
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onClickDropzone();
    }
  }

  async function onFileSelect(e: Event) {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;
    await handleFile(file);
  }

  // ── Core upload flow ──────────────────────────────────────────────────────

  async function handleFile(file: File) {
    // 1. Client-side extension validation
    const ext = file.name.split('.').pop()?.toLowerCase() ?? '';
    if (!ALLOWED_EXTS.includes(ext) || (!ALLOWED_MIME.includes(file.type) && file.type !== '')) {
      const msg = t('illustrations.invalidFormat', { ext });
      toast.error(msg);
      onerror?.(msg);
      return;
    }

    // Tauri exposes the native path via the File object in a WebView context
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const filePath: string = (file as any).path ?? '';
    if (!filePath) {
      toast.error(t('illustrations.noPathError'));
      return;
    }

    // 2. Quick DPI validation (reads header only)
    let validation: DpiValidation;
    try {
      validation = await ipcValidateIllustrationDpi(filePath);
    } catch (err) {
      const msg = `${t('illustrations.dpiValidationError')}: ${err}`;
      toast.error(msg);
      onerror?.(msg);
      return;
    }

    // 3a. DPI < 150 → hard block
    if (!validation.adequate) {
      toast.error(t('illustrations.dpiCritical', { dpi: String(validation.dpi) }));
      onerror?.(`DPI_CRITICAL: ${validation.dpi}`);
      return;
    }

    // 3b. DPI in [150, 300) → confirm dialog (warning case)
    if (validation.warning) {
      confirmDpiData = { path: filePath, validation };
      return;
    }

    // 3c. DPI ≥ 300 → proceed directly
    await processFile(filePath);
  }

  // Called after user confirms low-DPI warning
  async function onConfirmDpiWarning() {
    if (!confirmDpiData) return;
    const path = confirmDpiData.path;
    confirmDpiData = null;
    await processFile(path);
  }

  function onCancelDpiWarning() {
    confirmDpiData = null;
  }

  async function processFile(filePath: string) {
    isProcessing = true;
    try {
      const illustration = await ipcProcessIllustration(illustrationId, filePath, projectId);
      if (illustration) {
        toast.success(t('illustrations.importSuccess'));
        onimported?.(illustration);
      }
    } catch (err) {
      const msg = `${t('illustrations.processError')}: ${err}`;
      toast.error(msg);
      onerror?.(msg);
    } finally {
      isProcessing = false;
      // Reset file input so the same file can be re-selected
      if (fileInputRef) fileInputRef.value = '';
    }
  }
</script>

<!-- Hidden file input -->
<input
  bind:this={fileInputRef}
  type="file"
  accept={ALLOWED_EXTS.map((e) => `.${e}`).join(',')}
  style="display:none"
  aria-hidden="true"
  onchange={onFileSelect}
/>

<!-- DPI warning confirm dialog -->
{#if confirmDpiData}
  <div class="dpi-confirm-overlay" role="dialog" aria-modal="true" aria-labelledby="dpi-confirm-title">
    <div class="dpi-confirm-box">
      <h3 id="dpi-confirm-title" class="dpi-confirm-title">
        {t('illustrations.dpiWarningTitle')}
      </h3>
      <p class="dpi-confirm-body">
        {confirmDpiData.validation.warning}
      </p>
      <p class="dpi-confirm-hint">{t('illustrations.dpiWarningHint')}</p>
      <div class="dpi-confirm-actions">
        <button class="btn-cancel" onclick={onCancelDpiWarning}>
          {t('common.cancel')}
        </button>
        <button class="btn-confirm" onclick={onConfirmDpiWarning}>
          {t('illustrations.dpiWarningConfirm')}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Dropzone area -->
<div
  class="dropzone"
  class:dropzone--dragover={isDragOver}
  class:dropzone--processing={isProcessing}
  class:dropzone--disabled={disabled}
  data-testid="illustration-dropzone"
  role="button"
  tabindex={disabled || isProcessing ? -1 : 0}
  aria-label={placeholderName
    ? t('illustrations.dropzoneLabel', { name: placeholderName })
    : t('illustrations.dropzoneLabelGeneric')}
  aria-disabled={disabled || isProcessing}
  aria-busy={isProcessing}
  ondragover={onDragOver}
  ondragleave={onDragLeave}
  ondrop={onDrop}
  onclick={onClickDropzone}
  onkeydown={onKeyDown}
>
  {#if isProcessing}
    <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
         width="28" height="28" aria-hidden="true">
      <path d="M21 12a9 9 0 1 1-6.219-8.56" />
    </svg>
    <p class="dropzone-label">{t('illustrations.processing')}</p>
  {:else}
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"
         width="32" height="32" aria-hidden="true">
      <polyline points="16 16 12 12 8 16" />
      <line x1="12" y1="12" x2="12" y2="21" />
      <path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3" />
    </svg>

    {#if placeholderName}
      <p class="dropzone-label">{t('illustrations.dropzoneLabel', { name: placeholderName })}</p>
    {:else}
      <p class="dropzone-label">{t('illustrations.dropzoneLabelGeneric')}</p>
    {/if}

    <p class="dropzone-hint">{t('illustrations.dropzoneHint')}</p>

    <button
      class="btn-select"
      type="button"
      onclick={(e) => { e.stopPropagation(); onClickDropzone(); }}
      disabled={disabled || isProcessing}
      aria-label={t('illustrations.selectFileButton')}
    >
      {t('illustrations.selectFileButton')}
    </button>
  {/if}
</div>

<style>
  .dropzone {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 2rem 1.5rem;
    border: 2px dashed var(--color-border, #d1d5db);
    border-radius: var(--radius-md, 8px);
    background: var(--color-surface-secondary, #f9fafb);
    color: var(--color-text-secondary, #6b7280);
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s, color 0.15s;
    text-align: center;
    user-select: none;
  }

  .dropzone:focus-visible {
    outline: 2px solid var(--color-primary, #3b82f6);
    outline-offset: 2px;
  }

  .dropzone--dragover {
    border-color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.05);
    color: var(--color-primary, #3b82f6);
    border-width: 3px;
  }

  .dropzone--processing {
    cursor: wait;
    opacity: 0.75;
  }

  .dropzone--disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }

  .dropzone-label {
    font-size: 0.875rem;
    font-weight: 500;
    margin: 0;
  }

  .dropzone-hint {
    font-size: 0.75rem;
    margin: 0;
    opacity: 0.75;
  }

  .btn-select {
    margin-top: 0.25rem;
    padding: 0.375rem 1rem;
    border-radius: var(--radius-sm, 6px);
    font-size: 0.8125rem;
    font-weight: 500;
    border: 1px solid var(--color-border, #d1d5db);
    background: var(--color-surface, #fff);
    color: var(--color-text, #374151);
    cursor: pointer;
    transition: background 0.1s;
  }

  .btn-select:hover:not(:disabled) {
    background: var(--color-surface-secondary, #f3f4f6);
  }

  .btn-select:disabled { opacity: 0.5; cursor: not-allowed; }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spinner { animation: spin 0.8s linear infinite; }

  /* DPI confirm overlay */
  .dpi-confirm-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .dpi-confirm-box {
    background: var(--color-surface, #fff);
    border-radius: var(--radius-md, 8px);
    padding: 1.5rem;
    max-width: 380px;
    width: 90%;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.16);
  }

  .dpi-confirm-title {
    font-size: 1rem;
    font-weight: 600;
    margin: 0 0 0.75rem;
    color: var(--color-text, #111827);
  }

  .dpi-confirm-body {
    font-size: 0.875rem;
    color: var(--color-text, #374151);
    margin: 0 0 0.5rem;
  }

  .dpi-confirm-hint {
    font-size: 0.8125rem;
    color: var(--color-text-secondary, #6b7280);
    margin: 0 0 1.25rem;
  }

  .dpi-confirm-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  .btn-cancel {
    padding: 0.4375rem 1rem;
    border-radius: var(--radius-sm, 6px);
    font-size: 0.875rem;
    border: 1px solid var(--color-border, #d1d5db);
    background: transparent;
    color: var(--color-text-secondary, #6b7280);
    cursor: pointer;
  }

  .btn-confirm {
    padding: 0.4375rem 1rem;
    border-radius: var(--radius-sm, 6px);
    font-size: 0.875rem;
    font-weight: 500;
    border: none;
    background: #f59e0b;
    color: #fff;
    cursor: pointer;
  }

  .btn-confirm:hover { background: #d97706; }
</style>
