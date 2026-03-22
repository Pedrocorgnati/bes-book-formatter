<script lang="ts">
  import { toast } from '$lib/stores/toastStore';

  const toasts = $derived($toast);
</script>

<!-- Portal de toasts — canto superior direito -->
<div data-testid="toast-container" class="toast-container" aria-live="polite" aria-atomic="false" role="region" aria-label="Notificações">
  {#each toasts as t (t.id)}
    <div
      data-testid="toast-{t.id}"
      class="toast toast--{t.type}"
      role={t.type === 'error' || t.type === 'warning' ? 'alert' : 'status'}
      aria-live={t.type === 'error' || t.type === 'warning' ? 'assertive' : 'polite'}
    >
      <!-- Ícone por tipo -->
      <div class="toast__icon" aria-hidden="true">
        {#if t.type === 'success'}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        {:else if t.type === 'error'}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        {:else if t.type === 'warning'}
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

      <span class="toast__message">{t.message}</span>

      {#if t.dismissible}
        <button
          data-testid="toast-dismiss-{t.id}"
          class="toast__dismiss"
          onclick={() => toast.remove(t.id)}
          aria-label="Fechar notificação"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      {/if}
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: calc(var(--header-height) + var(--space-3));
    right: var(--space-4);
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    max-width: 360px;
    pointer-events: none;
  }

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
    animation: fade-in 200ms ease-out;
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
    background: #D97706;
    color: #FFFFFF;
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
