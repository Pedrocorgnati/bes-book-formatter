<script lang="ts">
  import { preferencesStore, setTheme } from '$lib/stores/preferencesStore';
  import { t } from '$lib/i18n/engine';

  const prefs = $derived($preferencesStore);
  const isDark = $derived(prefs.theme === 'dark');

  async function toggle() {
    const newTheme = isDark ? 'light' : 'dark';
    await setTheme(newTheme);
  }
</script>

<button
  data-testid="header-theme-toggle-button"
  class="theme-toggle"
  onclick={toggle}
  role="switch"
  aria-checked={isDark}
  aria-label={isDark ? t('header.switchToLight') : t('header.switchToDark')}
  title={isDark ? t('header.switchToLight') : t('header.switchToDark')}
>
  {#if isDark}
    <!-- Lua (dark mode ativo) -->
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
  {:else}
    <!-- Sol (light mode ativo) -->
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <circle cx="12" cy="12" r="5"/>
      <line x1="12" y1="1" x2="12" y2="3"/>
      <line x1="12" y1="21" x2="12" y2="23"/>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
      <line x1="1" y1="12" x2="3" y2="12"/>
      <line x1="21" y1="12" x2="23" y2="12"/>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
    </svg>
  {/if}
</button>

<style>
  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
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

  .theme-toggle:hover {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }

  .theme-toggle:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }
</style>
