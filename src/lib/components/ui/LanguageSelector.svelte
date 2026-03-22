<script lang="ts">
  import { locale, setLocale, availableLocales, type Locale } from '$lib/i18n/engine';
  import { setLanguage } from '$lib/stores/preferencesStore';
  import { UILanguage } from '$lib/types/enums';

  const currentLocale = $derived($locale);

  async function handleChange(e: Event) {
    const select = e.target as HTMLSelectElement;
    const newLocale = select.value as Locale;
    setLocale(newLocale);
    // Sincroniza com preferencesStore e SQLite
    await setLanguage(newLocale as UILanguage);
  }
</script>

<div data-testid="header-language-selector" class="lang-selector" aria-label="Idioma da interface">
  <label for="lang-select" class="sr-only">Idioma</label>
  <select
    data-testid="header-language-select"
    id="lang-select"
    class="lang-select"
    value={currentLocale}
    onchange={handleChange}
    aria-label="Selecionar idioma da interface"
  >
    {#each availableLocales as loc}
      <option value={loc.value} selected={loc.value === currentLocale}>
        {loc.label}
      </option>
    {/each}
  </select>
</div>

<style>
  .lang-selector {
    position: relative;
  }

  .lang-select {
    appearance: none;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    font-family: var(--font-sans);
    font-size: var(--text-xs);
    padding: var(--space-1) var(--space-3) var(--space-1) var(--space-2);
    cursor: pointer;
    transition:
      border-color var(--duration-fast) ease,
      background-color var(--duration-fast) ease;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' fill='none' stroke='%236B7280' stroke-width='1.5'%3E%3Cpath d='M1 1l4 4 4-4'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    min-width: 120px;
  }

  .lang-select:hover {
    border-color: var(--color-text-muted);
  }

  .lang-select:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
    border-color: var(--color-primary);
  }
</style>
