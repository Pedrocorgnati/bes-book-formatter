<script lang="ts">
  import { toast } from '$lib/stores/toastStore';
  import { t } from '$lib/i18n/engine';
  import Toast from './Toast.svelte';

  const toasts = $derived($toast);
</script>

<!-- Portal de toasts — canto superior direito -->
<!-- Dois containers separados para evitar aria-live aninhado (WCAG 4.1.3) -->
<!-- Polite: info e success -->
<div
  data-testid="toast-container"
  class="toast-container"
  role="region"
  aria-label={t('a11y.notifications')}
  aria-live="polite"
  aria-atomic="false"
>
  {#each toasts.filter(i => i.type === 'info' || i.type === 'success') as item (item.id)}
    <Toast
      id={item.id}
      type={item.type}
      message={item.message}
      dismissible={item.dismissible}
      onDismiss={(id) => toast.remove(id)}
    />
  {/each}
</div>

<!-- Assertive: error e warning -->
<div
  data-testid="toast-container-assertive"
  class="toast-container toast-container--assertive"
  role="region"
  aria-label={t('a11y.notifications')}
  aria-live="assertive"
  aria-atomic="false"
>
  {#each toasts.filter(i => i.type === 'error' || i.type === 'warning') as item (item.id)}
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

  .toast-container--assertive {
    top: calc(var(--header-height) + var(--space-3));
  }
</style>
