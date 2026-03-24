<script lang="ts">
  import { onMount } from 'svelte';
  import { browser, dev } from '$app/environment';
  import { TIMING } from '$lib/constants/timing';

  let isActive = $state(false);
  let elements = $state<Array<{ id: string; rect: DOMRect }>>([]);
  let copiedId = $state<string | null>(null);

  function scanDataTestIds() {
    if (!browser) return;
    const allElements = document.querySelectorAll('[data-testid]');
    elements = Array.from(allElements).map((el) => ({
      id: el.getAttribute('data-testid')!,
      rect: el.getBoundingClientRect(),
    }));
  }

  function handleToggle() {
    if (!isActive) {
      scanDataTestIds();
    }
    isActive = !isActive;
  }

  async function handleCopy(testId: string) {
    const copyText = `data-testid="${testId}"`;
    try {
      await navigator.clipboard.writeText(copyText);
    } catch {
      const textArea = document.createElement('textarea');
      textArea.value = copyText;
      textArea.style.position = 'fixed';
      textArea.style.left = '-9999px';
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
    }
    copiedId = testId;
    setTimeout(() => (copiedId = null), TIMING.COPY_FEEDBACK_RESET);
  }

  $effect(() => {
    if (!isActive || !browser) return;

    const handleUpdate = () => scanDataTestIds();

    window.addEventListener('scroll', handleUpdate, true);
    window.addEventListener('resize', handleUpdate);

    return () => {
      window.removeEventListener('scroll', handleUpdate, true);
      window.removeEventListener('resize', handleUpdate);
    };
  });
</script>

{#if dev}
  <!-- Botao flutuante -->
  <button
    onclick={handleToggle}
    style="
      position: fixed;
      top: 12px;
      right: 12px;
      z-index: 99999;
      padding: 6px 12px;
      font-size: 12px;
      font-weight: 600;
      font-family: monospace;
      border: 2px solid {isActive ? '#ffffff' : '#ef4444'};
      border-radius: 6px;
      background-color: {isActive ? '#ef4444' : '#ffffff'};
      color: {isActive ? '#ffffff' : '#ef4444'};
      cursor: pointer;
      box-shadow: 0 2px 8px rgba(0,0,0,0.15);
      user-select: none;
    "
    aria-label={isActive ? 'Esconder data-testid overlays' : 'Mostrar data-testid overlays'}
  >
    [data-test]
  </button>

  <!-- Overlays dos data-testid -->
  {#if isActive}
    {#each elements as el (`${el.id}-${el.rect.top}-${el.rect.left}`)}
      <button
        onclick={() => handleCopy(el.id)}
        title="Clique para copiar: {el.id}"
        style="
          position: fixed;
          top: {el.rect.top}px;
          left: {el.rect.left}px;
          z-index: 99998;
          padding: 2px 6px;
          font-size: 10px;
          font-weight: 600;
          font-family: monospace;
          background-color: {copiedId === el.id ? '#16a34a' : '#ef4444'};
          color: #ffffff;
          border-radius: 3px;
          border: none;
          cursor: pointer;
          white-space: nowrap;
          pointer-events: auto;
          box-shadow: 0 1px 4px rgba(0,0,0,0.2);
          line-height: 1.4;
        "
      >
        {copiedId === el.id ? 'Copiado!' : el.id}
      </button>
    {/each}
  {/if}
{/if}
