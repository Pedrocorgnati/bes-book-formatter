<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';
  import { t } from '$lib/i18n/engine';

  interface Props {
    open: boolean;
    title: string;
    size?: 'sm' | 'md' | 'lg';
    onClose: () => void;
    children?: Snippet;
    footer?: Snippet;
    closeOnOverlay?: boolean;
  }

  let {
    open,
    title,
    size = 'md',
    onClose,
    children,
    footer,
    closeOnOverlay = true
  }: Props = $props();

  let modalEl: HTMLDivElement | undefined = $state();
  const titleId = `modal-title-${Math.random().toString(36).slice(2)}`;

  // Focus trap
  function trapFocus(e: KeyboardEvent) {
    if (!open || !modalEl) return;
    if (e.key !== 'Tab') return;

    const focusable = modalEl.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const first = focusable[0];
    const last = focusable[focusable.length - 1];

    if (e.shiftKey) {
      if (document.activeElement === first) {
        e.preventDefault();
        last?.focus();
      }
    } else {
      if (document.activeElement === last) {
        e.preventDefault();
        first?.focus();
      }
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) onClose();
    trapFocus(e);
  }

  // Foca no modal quando abre
  $effect(() => {
    if (open && modalEl) {
      const focusable = modalEl.querySelector<HTMLElement>('button, [href], input, [tabindex]:not([tabindex="-1"])');
      setTimeout(() => focusable?.focus(), 50);
    }
  });
</script>

<svelte:window on:keydown={handleKeydown} />

{#if open}
  <!-- Overlay -->
  <div
    data-testid="modal-overlay"
    class="modal-overlay"
    onclick={closeOnOverlay ? onClose : undefined}
    aria-hidden="true"
  ></div>

  <!-- Modal panel -->
  <div
    data-testid="modal"
    class="modal modal--{size}"
    role="dialog"
    aria-modal="true"
    aria-labelledby={titleId}
    bind:this={modalEl}
  >
    <!-- Header -->
    <div data-testid="modal-header" class="modal__header">
      <h2 data-testid="modal-title" class="modal__title" id={titleId}>{title}</h2>
      <button
        data-testid="modal-close-button"
        class="modal__close"
        onclick={onClose}
        aria-label={t('a11y.closeModal')}
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <!-- Divisor -->
    <div class="modal__divider" role="separator"></div>

    <!-- Corpo -->
    <div data-testid="modal-body" class="modal__body">
      {#if children}
        {@render children()}
      {/if}
    </div>

    <!-- Footer (slot opcional) -->
    {#if footer}
      <div class="modal__divider" role="separator"></div>
      <div data-testid="modal-footer" class="modal__footer">
        {@render footer()}
      </div>
    {/if}
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    z-index: 1000;
    animation: fade-in 200ms ease-out;
  }

  .modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 1001;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-xl);
    display: flex;
    flex-direction: column;
    max-height: 85vh;
    animation: scale-in 200ms ease-out;
    overflow: hidden;
  }

  .modal--sm { width: 400px; }
  .modal--md { width: 560px; }
  .modal--lg { width: 720px; }

  .modal__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4) var(--space-6);
  }

  .modal__title {
    font-size: var(--text-lg);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
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
    transition:
      background-color var(--duration-fast) ease,
      color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .modal__close:hover {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }

  .modal__divider {
    height: 1px;
    background: var(--color-border);
    flex-shrink: 0;
  }

  .modal__body {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-6);
  }

  .modal__footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-6);
  }
</style>
