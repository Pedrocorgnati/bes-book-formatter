<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import type { SidecarStatus } from '$lib/types/interfaces';

  let dismissed = $state(false);
  let status = $state<SidecarStatus | null>(null);
  let checking = $state(true);

  onMount(async () => {
    // TODO: Rock implementa verificação real via IPC
    // stub: simula verificação rápida
    await new Promise(r => setTimeout(r, 500));
    status = {
      typst: { name: 'Typst', version: null, available: false },
      ghostscript: { name: 'Ghostscript', version: null, available: false },
      epubcheck: { name: 'EPUBCheck', version: null, available: false },
      checkedAt: new Date().toISOString()
    };
    checking = false;
  });

  // Mostra banner apenas se algum sidecar estiver indisponível
  const showBanner = $derived(
    !dismissed &&
    !checking &&
    status !== null &&
    (!status.typst.available || !status.ghostscript.available || !status.epubcheck.available)
  );

  function dismiss() {
    dismissed = true;
  }
</script>

{#if checking}
  <!-- Banner de verificação -->
  <div data-testid="sidecar-banner-checking" class="sidecar-banner sidecar-banner--checking" role="status" aria-live="polite">
    <div class="sidecar-banner__spinner" aria-hidden="true"></div>
    <span>{t('sidecar.checking')}</span>
  </div>
{:else if showBanner && status}
  <div data-testid="sidecar-banner-warning" class="sidecar-banner sidecar-banner--warning" role="status" aria-live="polite">
    <div class="sidecar-banner__items">
      <!-- Typst -->
      {#if status.typst.available}
        <span class="sidecar-item sidecar-item--ok">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>
          {t('sidecar.typst')} {status.typst.version || ''}
        </span>
      {:else}
        <span class="sidecar-item sidecar-item--warn">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          {t('sidecar.typst')} — não encontrado
        </span>
      {/if}

      <!-- Ghostscript -->
      {#if status.ghostscript.available}
        <span class="sidecar-item sidecar-item--ok">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>
          {t('sidecar.ghostscript')} {status.ghostscript.version || ''}
        </span>
      {:else}
        <span class="sidecar-item sidecar-item--warn">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          {t('sidecar.ghostscript')} — não encontrado
        </span>
      {/if}

      <!-- EPUBCheck -->
      {#if status.epubcheck.available}
        <span class="sidecar-item sidecar-item--ok">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>
          {t('sidecar.epubcheck')} {status.epubcheck.version || ''}
        </span>
      {:else}
        <span class="sidecar-item sidecar-item--warn">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          {t('sidecar.epubcheck')} — não encontrado
        </span>
      {/if}
    </div>

    <button data-testid="sidecar-dismiss-button" class="sidecar-banner__dismiss" onclick={dismiss} aria-label="Dispensar aviso de sidecars">
      {t('sidecar.dismiss')}
    </button>
  </div>
{/if}

<style>
  .sidecar-banner {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-xs);
    min-height: 36px;
  }

  .sidecar-banner--checking {
    background: var(--color-muted, #F3F4F6);
    color: var(--color-text-muted);
  }

  .sidecar-banner--warning {
    background: rgba(217, 119, 6, 0.1);
    border-bottom: 1px solid rgba(217, 119, 6, 0.3);
    color: var(--color-text-secondary);
  }

  .sidecar-banner__spinner {
    width: 12px;
    height: 12px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-text-muted);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    flex-shrink: 0;
  }

  .sidecar-banner__items {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    flex: 1;
    flex-wrap: wrap;
  }

  .sidecar-item {
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .sidecar-item--ok {
    color: #059669;
  }

  .sidecar-item--warn {
    color: #D97706;
  }

  .sidecar-banner__dismiss {
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: var(--text-xs);
    font-family: var(--font-sans);
    padding: var(--space-1) var(--space-2);
    transition: background-color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .sidecar-banner__dismiss:hover {
    background: var(--color-bg-secondary);
  }
</style>
