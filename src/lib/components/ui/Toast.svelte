<script lang="ts">
  import { fly } from 'svelte/transition';
  import { t } from '$lib/i18n/engine';

  interface Props {
    id: string;
    type: 'success' | 'error' | 'warning' | 'info';
    message: string;
    dismissible?: boolean;
    onDismiss: (id: string) => void;
  }

  let { id, type, message, dismissible, onDismiss }: Props = $props();

  // role mantido para semântica (alert/status), aria-live gerenciado pelo container (WCAG 4.1.3)
  const role = $derived(type === 'error' || type === 'warning' ? 'alert' : 'status');
</script>

<div
  data-testid="toast-{id}"
  class="toast toast--{type}"
  {role}
  in:fly={{ x: 80, duration: 250 }}
  out:fly={{ x: 80, duration: 200 }}
>
  <!-- Ícone por tipo -->
  <div class="toast__icon" aria-hidden="true">
    {#if type === 'success'}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <polyline points="20 6 9 17 4 12"/>
      </svg>
    {:else if type === 'error'}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    {:else if type === 'warning'}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
        <line x1="12" y1="9" x2="12" y2="13"/>
        <line x1="12" y1="17" x2="12.01" y2="17"/>
      </svg>
    {:else}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
    {/if}
  </div>

  <span class="toast__message">{message}</span>

  {#if dismissible}
    <button
      data-testid="toast-dismiss-{id}"
      class="toast__dismiss"
      onclick={() => onDismiss(id)}
      aria-label={t('a11y.closeNotification')}
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .toast {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    font-size: var(--text-sm);
    font-weight: 500;
    pointer-events: all;
    min-width: 280px;
  }

  .toast--success {
    background: #16A34A;
    color: #FFFFFF;
  }

  .toast--error {
    background: var(--color-primary);
    color: var(--color-on-primary);
  }

  .toast--warning {
    background: var(--color-warning);
    color: var(--color-on-warning);
  }

  .toast--info {
    background: #2563EB;
    color: #FFFFFF;
  }

  .toast__icon {
    flex-shrink: 0;
    margin-top: 1px;
  }

  .toast__message {
    flex: 1;
    line-height: 1.4;
  }

  .toast__dismiss {
    flex-shrink: 0;
    background: rgba(255, 255, 255, 0.2);
    border: none;
    border-radius: var(--radius-sm);
    color: inherit;
    cursor: pointer;
    padding: var(--space-1);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color var(--duration-fast) ease;
  }

  .toast__dismiss:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>
