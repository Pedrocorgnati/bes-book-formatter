<script lang="ts">
  import { toast } from '$lib/stores/toastStore';
  import { t } from '$lib/i18n/engine';
  import Toast from './Toast.svelte';

  const toasts = $derived($toast);
</script>

<!-- Portal de toasts — canto superior direito -->
<div data-testid="toast-container" class="toast-container" aria-live="polite" aria-atomic="false" role="region" aria-label={t('a11y.notifications')}>
  {#each toasts as item (item.id)}
    <Toast
      id={item.id}
      type={item.type}
      message={item.message}
      dismissible={item.dismissible}
      onDismiss={(id) => toast.remove(id)}
    />
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
</style>
