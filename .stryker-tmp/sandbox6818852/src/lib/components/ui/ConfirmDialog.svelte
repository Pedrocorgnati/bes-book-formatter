<script lang="ts">
  import { t } from '$lib/i18n/engine';

  interface Props {
    open: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    destructive?: boolean;
    onConfirm: () => void;
    onClose: () => void;
  }

  let {
    open,
    title,
    message,
    confirmLabel = t('common.confirm'),
    cancelLabel = t('common.cancel'),
    destructive = false,
    onConfirm,
    onClose
  }: Props = $props();

  const descId = `confirm-desc-${Math.random().toString(36).slice(2)}`;
  let cancelBtn: HTMLButtonElement | undefined = $state();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) onClose();
  }

  // Foca no Cancelar ao abrir (padrão seguro)
  $effect(() => {
    if (open) {
      setTimeout(() => cancelBtn?.focus(), 50);
    }
  });
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <!-- Overlay NÃO fecha (ação crítica) -->
  <div class="modal-overlay" aria-hidden="true"></div>

  <div
    data-testid="confirm-dialog"
    class="confirm-dialog"
    role="alertdialog"
    aria-modal="true"
    aria-labelledby="confirm-title"
    aria-describedby={descId}
  >
    <div data-testid="confirm-dialog-header" class="confirm-dialog__header">
      {#if destructive}
        <div class="confirm-dialog__warn-icon" aria-hidden="true">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#D97706" stroke-width="2">
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
            <line x1="12" y1="9" x2="12" y2="13"/>
            <line x1="12" y1="17" x2="12.01" y2="17"/>
          </svg>
        </div>
      {/if}
      <h2 class="confirm-dialog__title" id="confirm-title">{title}</h2>
      <button data-testid="confirm-dialog-close-button" class="modal__close" onclick={onClose} aria-label={t('common.close')}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <div class="confirm-dialog__divider" role="separator"></div>

    <div class="confirm-dialog__body" id={descId}>
      <p>{message}</p>
    </div>

    <div class="confirm-dialog__divider" role="separator"></div>

    <div data-testid="confirm-dialog-footer" class="confirm-dialog__footer">
      <!-- Cancelar sempre em foco (padrão seguro) -->
      <button data-testid="confirm-dialog-cancel-button" class="btn btn--ghost" onclick={onClose} bind:this={cancelBtn}>
        {cancelLabel}
      </button>
      <button
        data-testid="confirm-dialog-confirm-button"
        class="btn"
        class:btn--destructive={destructive}
        class:btn--primary={!destructive}
        onclick={() => { onConfirm(); onClose(); }}
      >
        {confirmLabel}
      </button>
    </div>
  </div>
{/if}

<style>
  .confirm-dialog {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 1001;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-xl);
    width: 420px;
    max-width: 90vw;
    animation: scale-in 200ms ease-out;
  }

  .confirm-dialog__header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-6);
  }

  .confirm-dialog__warn-icon {
    flex-shrink: 0;
  }

  .confirm-dialog__title {
    font-size: var(--text-lg);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
    flex: 1;
  }

  .confirm-dialog__divider {
    height: 1px;
    background: var(--color-border);
  }

  .confirm-dialog__body {
    padding: var(--space-6);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    line-height: 1.6;
  }

  .confirm-dialog__footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-6);
  }

  .modal__close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    cursor: pointer;
    transition: background-color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .modal__close:hover {
    background: var(--color-bg-secondary);
  }

  /* Reutiliza as classes .btn do Header.svelte (global) */
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    z-index: 1000;
    animation: fade-in 200ms ease-out;
  }
</style>
