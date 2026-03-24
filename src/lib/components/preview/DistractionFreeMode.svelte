<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { ipcToggleDistractionFree } from '$lib/ipc/preview';

  let distractionFree = $state(false);
  let unlisten: (() => void) | null = null;

  async function toggle(enabled: boolean) {
    await ipcToggleDistractionFree(enabled);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'F11') {
      e.preventDefault();
      toggle(!distractionFree);
    } else if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      toggle(!distractionFree);
    } else if (e.key === 'Escape' && distractionFree) {
      toggle(false);
    }
  }

  onMount(async () => {
    unlisten = await listen<{ enabled: boolean }>('preview:distraction-free', (event) => {
      distractionFree = event.payload.enabled;
      document.body.classList.toggle('distraction-free', event.payload.enabled);
    });
  });

  onDestroy(() => {
    unlisten?.();
    // Ensure we clean up distraction-free class on unmount
    document.body.classList.remove('distraction-free');
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Distraction-free mode toolbar overlay (shown when active) -->
{#if distractionFree}
  <div class="df-exit-hint" role="status" aria-live="polite">
    <span>{t('preview.distractionFreeHint')}</span>
    <button class="df-exit-btn" onclick={() => toggle(false)} aria-label={t('preview.exitDistractionFree')}>
      ✕ {t('preview.exitDistractionFree')}
    </button>
  </div>
{/if}

<style>
  .df-exit-hint {
    position: fixed;
    bottom: var(--space-4);
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: var(--space-3);
    background: rgba(0, 0, 0, 0.7);
    color: white;
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-full);
    font-size: var(--text-sm);
    z-index: 1000;
    animation: df-fade-in 0.3s ease;
    transition: opacity var(--duration-slow) ease;
  }

  @keyframes df-fade-in {
    from { opacity: 0; transform: translateX(-50%) translateY(8px); }
    to { opacity: 1; transform: translateX(-50%) translateY(0); }
  }

  .df-exit-btn {
    background: rgba(255, 255, 255, 0.2);
    color: white;
    border: 1px solid rgba(255, 255, 255, 0.3);
    border-radius: var(--radius-sm);
    padding: var(--space-1) var(--space-2);
    font-size: var(--text-xs);
    cursor: pointer;
    transition: background var(--duration-fast);
  }

  .df-exit-btn:hover {
    background: rgba(255, 255, 255, 0.3);
  }
</style>
