<script lang="ts">
  import { t, setLocale, availableLocales, locale } from '$lib/i18n/engine';
  import { preferencesStore, setTheme, setLanguage, setAnalyticsOptIn } from '$lib/stores/preferencesStore';
  import type { UILanguage } from '$lib/types/enums';

  // RESOLVED: Changed availableLocales field from .code to .value
  const prefs = $derived($preferencesStore);
  const currentTheme = $derived(prefs?.theme ?? 'light');
  // RESOLVED: Changed prefs.language to prefs.uiLanguage (correct field name)
  const currentLang = $derived(prefs?.uiLanguage ?? 'pt-BR');
  const analyticsOptIn = $derived(prefs?.analyticsOptIn ?? false);

  function handleThemeChange(theme: 'light' | 'dark') {
    setTheme(theme);
  }

  function handleLanguageChange(e: Event) {
    const select = e.target as HTMLSelectElement;
    const lang = select.value as UILanguage;
    setLanguage(lang);
    setLocale(lang);
  }

  function handleAnalyticsChange(e: Event) {
    const checkbox = e.target as HTMLInputElement;
    setAnalyticsOptIn(checkbox.checked);
  }
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.settings')}</title>
</svelte:head>

<div data-testid="settings-page" class="settings-page">
  <div data-testid="settings-page-header" class="settings-page__header">
    <h1 data-testid="settings-page-title" class="settings-page__title">
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="3"/>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
      </svg>
      {t('nav.settings')}
    </h1>
  </div>

  <div data-testid="settings-page-body" class="settings-page__body">
    <!-- Seção: Aparência -->
    <section data-testid="settings-appearance-section" class="settings-section" aria-labelledby="appearance-heading">
      <h2 class="settings-section__title" id="appearance-heading">
        {t('settings.appearance')}
      </h2>

      <!-- Tema -->
      <fieldset class="settings-field">
        <legend class="settings-field__label">{t('settings.theme')}</legend>
        <div class="settings-field__radios">
          <label class="settings-radio">
            <input
              data-testid="settings-theme-light-radio"
              type="radio"
              name="theme"
              value="light"
              checked={currentTheme === 'light'}
              onchange={() => handleThemeChange('light')}
            />
            <span class="settings-radio__label">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
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
              {t('settings.themeLight')}
            </span>
          </label>

          <label class="settings-radio">
            <input
              data-testid="settings-theme-dark-radio"
              type="radio"
              name="theme"
              value="dark"
              checked={currentTheme === 'dark'}
              onchange={() => handleThemeChange('dark')}
            />
            <span class="settings-radio__label">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
              </svg>
              {t('settings.themeDark')}
            </span>
          </label>
        </div>
      </fieldset>

      <!-- Idioma -->
      <div class="settings-field">
        <label class="settings-field__label" for="lang-select">
          {t('settings.language')}
        </label>
        <select
          data-testid="settings-language-select"
          id="lang-select"
          class="settings-field__select"
          value={currentLang}
          onchange={handleLanguageChange}
        >
          {#each availableLocales as loc}
            <option value={loc.value}>{loc.label}</option>
          {/each}
        </select>
      </div>
    </section>

    <div class="settings-divider" role="separator"></div>

    <!-- Seção: Privacidade -->
    <section data-testid="settings-privacy-section" class="settings-section" aria-labelledby="privacy-heading">
      <h2 class="settings-section__title" id="privacy-heading">
        {t('settings.privacy')}
      </h2>

      <div class="settings-field settings-field--inline">
        <div class="settings-field__text">
          <label class="settings-field__label" for="analytics-toggle">
            {t('settings.telemetry')}
          </label>
          <p class="settings-field__desc">{t('settings.telemetryDesc')}</p>
        </div>
        <input
          data-testid="settings-analytics-toggle"
          type="checkbox"
          id="analytics-toggle"
          class="settings-field__checkbox"
          checked={analyticsOptIn}
          onchange={handleAnalyticsChange}
          aria-describedby="analytics-lgpd"
        />
      </div>
      <p class="settings-field__hint" id="analytics-lgpd">
        {t('settings.lgpdNote')}
      </p>
    </section>

    <div class="settings-divider" role="separator"></div>

    <!-- Seção: Sobre -->
    <section data-testid="settings-about-section" class="settings-section" aria-labelledby="about-heading">
      <h2 class="settings-section__title" id="about-heading">
        {t('settings.about')}
      </h2>
      <p class="settings-about__name">BES Book Formatter</p>
      <p class="settings-about__version">{t('settings.version')} 0.1.0</p>
    </section>
  </div>
</div>

<style>
  .settings-page {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .settings-page__header {
    padding: var(--space-6) var(--space-6) var(--space-4);
    border-bottom: 1px solid var(--color-border);
  }

  .settings-page__title {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-family: var(--font-serif);
    font-size: var(--text-2xl);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  .settings-page__body {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-6);
    max-width: 560px;
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .settings-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .settings-section__title {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin: 0;
  }

  .settings-divider {
    height: 1px;
    background: var(--color-border);
  }

  .settings-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .settings-field--inline {
    flex-direction: row;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .settings-field__label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text);
  }

  .settings-field__desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .settings-field__hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .settings-field__radios {
    display: flex;
    gap: var(--space-4);
  }

  .settings-radio {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    cursor: pointer;
  }

  .settings-radio input[type="radio"] {
    accent-color: var(--color-primary);
    width: 16px;
    height: 16px;
  }

  .settings-radio__label {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    font-size: var(--text-sm);
    color: var(--color-text);
  }

  .settings-field__select {
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    color: var(--color-text);
    font-size: var(--text-sm);
    font-family: var(--font-sans);
    cursor: pointer;
    width: 100%;
    max-width: 280px;
  }

  .settings-field__select:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .settings-field__checkbox {
    width: 18px;
    height: 18px;
    accent-color: var(--color-primary);
    cursor: pointer;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .settings-about__name {
    font-family: var(--font-serif);
    font-size: var(--text-lg);
    font-weight: 700;
    color: var(--color-text);
    margin: 0;
  }

  .settings-about__version {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
    font-family: var(--font-mono);
  }
</style>
