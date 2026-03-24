<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { toast } from '$lib/stores/toastStore';
  import { ipcUploadFont } from '$lib/ipc/typography';
  import { ipc } from '$lib/utils/ipc';
  import type { FontInfo } from '$lib/types/interfaces';

  interface Props {
    projectId: string;
    onFontAdded?: (font: FontInfo) => void;
  }

  let { projectId, onFontAdded }: Props = $props();

  let uploading = $state(false);

  async function handleSelectFont() {
    if (uploading || !projectId) return;

    // Open native file dialog via IPC (select_font_file command)
    const selected = await ipc<string | null>('select_font_file');
    if (!selected) return;

    uploading = true;
    try {
      const info = await ipcUploadFont(projectId, selected);
      toast.success(`${t('typography.fontAddedSuccess')}: "${info.name}"`);
      onFontAdded?.(info);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (msg.includes('VAL_002')) {
        toast.error(t('typography.fontInvalidExtension'));
      } else if (msg.includes('VAL_003')) {
        toast.error(t('typography.fontTooLarge'));
      } else {
        toast.error(t('typography.fontUploadError'));
      }
    } finally {
      uploading = false;
    }
  }
</script>

<!-- Rock-2/TASK-2: Upload de fontes OTF/TTF -->
<div class="font-uploader" data-testid="font-uploader">
  <button
    class="upload-btn"
    onclick={handleSelectFont}
    disabled={uploading || !projectId}
    aria-busy={uploading}
    aria-label={t('typography.addFont')}
  >
    {#if uploading}
      <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16" aria-hidden="true">
        <path d="M21 12a9 9 0 1 1-6.219-8.56" />
      </svg>
      {t('common.loading')}
    {:else}
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16" aria-hidden="true">
        <path d="M12 5v14M5 12l7-7 7 7" />
      </svg>
      {t('typography.addFont')}
    {/if}
  </button>
  <p class="upload-hint">{t('typography.fontUploaderHint')}</p>
</div>

<style>
  .font-uploader {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .upload-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: var(--radius-sm, 6px);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid var(--color-border, #d1d5db);
    background: var(--color-surface, #fff);
    color: var(--color-text, #374151);
    transition: all 0.15s;
  }

  .upload-btn:hover:not(:disabled) {
    border-color: var(--color-primary, #3b82f6);
    color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.04);
  }

  .upload-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .upload-hint {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #6b7280);
    margin: 0;
  }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spinner { animation: spin 0.8s linear infinite; }
</style>
