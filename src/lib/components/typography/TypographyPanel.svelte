<script lang="ts">
  import { onDestroy } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { typographyStore, typographyLoadingStore } from '$lib/stores/typography';
  import { toast } from '$lib/stores/toastStore';
  import { ipcSetTypographyConfig } from '$lib/ipc/typography';
  import GenrePresetPicker from './GenrePresetPicker.svelte';
  import { TIMING } from '$lib/constants/timing';

  interface Props {
    projectId: string;
  }

  let { projectId }: Props = $props();

  const config = $derived($typographyStore);
  const loading = $derived($typographyLoadingStore);
  let saveError = $state<string | null>(null);

  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  onDestroy(() => {
    if (saveTimer) { clearTimeout(saveTimer); saveTimer = null; }
  });

  // Debounced save — 500ms after last change
  function scheduleSave(partial: Record<string, unknown>) {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => persistConfig(partial), TIMING.DEBOUNCE_CONFIG_SAVE);
  }

  async function persistConfig(partial: Record<string, unknown>) {
    if (!config || !projectId) return;
    typographyLoadingStore.set(true);
    saveError = null;
    try {
      const updated = await ipcSetTypographyConfig(projectId, partial as never);
      if (updated) typographyStore.set(updated);
      toast.success(t('typography.savedSuccess'));
    } catch (err) {
      saveError = t('typography.saveError');
      console.error('[TypographyPanel] save error:', err instanceof Error ? err.message : String(err));
    } finally {
      typographyLoadingStore.set(false);
    }
  }

  function onGenreChange(genre: string) {
    scheduleSave({ genrePreset: genre });
  }

  function onLeadingChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (isNaN(val)) return;
    scheduleSave({ leading: val });
  }

  function onFontSizeChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseFloat(input.value);
    if (isNaN(val) || val < 8 || val > 48) {
      toast.error(t('typography.fontSizeRangeError'));
      if (config) input.value = String(config.fontSizeBody);
      return;
    }
    scheduleSave({ fontSizeBody: val });
  }

  function onFontBodyChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    scheduleSave({ fontBody: val, fontHeading: val });
  }

  function onHyphenationToggle(e: Event) {
    scheduleSave({ hyphenation: (e.target as HTMLInputElement).checked });
  }

  function onJustificationToggle(e: Event) {
    scheduleSave({ justification: (e.target as HTMLInputElement).checked });
  }

  function onOrphanControlChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value);
    if (isNaN(val) || val < 1 || val > 6) {
      toast.error(t('typography.orphanWidowRangeError'));
      if (config) input.value = String(config.orphanControl);
      return;
    }
    scheduleSave({ orphanControl: val });
  }

  function onWidowControlChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const val = parseInt(input.value);
    if (isNaN(val) || val < 1 || val > 6) {
      toast.error(t('typography.orphanWidowRangeError'));
      if (config) input.value = String(config.widowControl);
      return;
    }
    scheduleSave({ widowControl: val });
  }

  function onIllustrationMissingModeChange(e: Event) {
    const val = (e.target as HTMLInputElement).value as
      'placeholder_visual' | 'remove_space' | 'block_generation';
    scheduleSave({ illustrationMissingMode: val });
  }

  const availableFonts = ['EB Garamond', 'Source Serif 4', 'JetBrains Mono'];
</script>

<div class="typography-panel" data-testid="typography-panel">
  <header class="panel-header">
    <h2 class="panel-title">{t('typography.title')}</h2>
    {#if loading}
      <span class="saving-indicator" aria-live="polite" aria-label={t('common.saving')}>
        <svg class="spinner" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
          <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
        {t('common.saving')}
      </span>
    {/if}
  </header>

  {#if saveError}
    <div class="panel-error" role="alert">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span>{saveError}</span>
    </div>
  {/if}

  {#if !config}
    <div class="panel-skeleton" role="status" aria-label={t('typography.loading')}>
      <div class="skeleton-line skeleton-line--lg"></div>
      <div class="skeleton-line skeleton-line--md"></div>
      <div class="skeleton-line skeleton-line--sm"></div>
      <div class="skeleton-line skeleton-line--lg"></div>
      <div class="skeleton-line skeleton-line--md"></div>
    </div>
  {:else}
    <div class="panel-body">
      <!-- Seção: Preset por Gênero -->
      <section class="panel-section" aria-labelledby="section-preset">
        <h3 id="section-preset" class="section-title">{t('typography.presetSection')}</h3>
        <GenrePresetPicker
          value={config.genrePreset}
          onchange={onGenreChange}
          disabled={loading}
        />
      </section>

      <!-- Seção: Fonte -->
      <section class="panel-section" aria-labelledby="section-font">
        <h3 id="section-font" class="section-title">{t('typography.fontSection')}</h3>

        <div class="field-group">
          <label for="font-body" class="field-label">{t('typography.fontBody')}</label>
          <select
            id="font-body"
            class="field-select"
            value={config.fontBody}
            disabled={loading}
            onchange={onFontBodyChange}
          >
            {#each availableFonts as font}
              <option value={font}>{font}</option>
            {/each}
          </select>
        </div>

        <div class="field-group">
          <label for="font-size" class="field-label">
            {t('typography.fontSizeBody')}
            <span class="field-unit">pt</span>
          </label>
          <input
            id="font-size"
            type="number"
            class="field-input"
            value={config.fontSizeBody}
            min="8"
            max="48"
            step="0.5"
            disabled={loading}
            onchange={onFontSizeChange}
            aria-describedby="font-size-hint"
          />
          <span id="font-size-hint" class="field-hint">8–48pt</span>
        </div>
      </section>

      <!-- Seção: Espaçamento -->
      <section class="panel-section" aria-labelledby="section-spacing">
        <h3 id="section-spacing" class="section-title">{t('typography.spacingSection')}</h3>

        <div class="field-group">
          <label for="leading" class="field-label">
            {t('typography.leading')}
          </label>
          <input
            id="leading"
            type="range"
            class="field-range"
            value={config.leading}
            min="0.8"
            max="3.0"
            step="0.05"
            disabled={loading}
            oninput={onLeadingChange}
            aria-valuenow={config.leading}
            aria-valuemin={0.8}
            aria-valuemax={3.0}
          />
          <span class="range-value">{config.leading.toFixed(2)}</span>
        </div>
      </section>

      <!-- Seção: Hifenação e Alinhamento -->
      <section class="panel-section" aria-labelledby="section-hyphens">
        <h3 id="section-hyphens" class="section-title">{t('typography.hyphenationSection')}</h3>

        <div class="field-toggle">
          <label for="justification" class="toggle-label">
            <input
              id="justification"
              type="checkbox"
              class="toggle-checkbox"
              checked={config.justification}
              disabled={loading}
              onchange={onJustificationToggle}
            />
            <span class="toggle-text">{t('typography.justification')}</span>
          </label>
        </div>

        <div class="field-toggle">
          <label for="hyphenation" class="toggle-label">
            <input
              id="hyphenation"
              type="checkbox"
              class="toggle-checkbox"
              checked={config.hyphenation}
              disabled={loading}
              onchange={onHyphenationToggle}
            />
            <span class="toggle-text">{t('typography.hyphenation')}</span>
          </label>
        </div>
      </section>

      <!-- Seção: Controle de Órfãs/Viúvas -->
      <section class="panel-section" aria-labelledby="section-orphans">
        <h3 id="section-orphans" class="section-title">{t('typography.orphanWidowSection')}</h3>

        <div class="field-group fields-row">
          <div class="field-group">
            <label for="orphan-control" class="field-label">{t('typography.orphanControl')}</label>
            <input
              id="orphan-control"
              type="number"
              class="field-input field-input--sm"
              value={config.orphanControl}
              min="1"
              max="6"
              disabled={loading}
              onchange={onOrphanControlChange}
            />
          </div>
          <div class="field-group">
            <label for="widow-control" class="field-label">{t('typography.widowControl')}</label>
            <input
              id="widow-control"
              type="number"
              class="field-input field-input--sm"
              value={config.widowControl}
              min="1"
              max="6"
              disabled={loading}
              onchange={onWidowControlChange}
            />
          </div>
        </div>
      </section>

      <!-- Seção: Ilustrações Pendentes -->
      <section class="panel-section" aria-labelledby="section-illus-mode">
        <h3 id="section-illus-mode" class="section-title">{t('typography.illustrationModeSection')}</h3>
        <p class="section-hint">{t('typography.illustrationModeHint')}</p>

        <fieldset class="mode-fieldset" disabled={loading}>
          <legend class="visually-hidden">{t('typography.illustrationModeSection')}</legend>

          {#each [
            { value: 'placeholder_visual', labelKey: 'typography.modePlaceholderVisual', hintKey: 'typography.modePlaceholderVisualHint' },
            { value: 'remove_space',       labelKey: 'typography.modeRemoveSpace',       hintKey: 'typography.modeRemoveSpaceHint' },
            { value: 'block_generation',   labelKey: 'typography.modeBlockGeneration',   hintKey: 'typography.modeBlockGenerationHint' },
          ] as mode}
            <label class="radio-option" class:radio-option--selected={config.illustrationMissingMode === mode.value}>
              <input
                type="radio"
                name="illustration-missing-mode"
                value={mode.value}
                checked={config.illustrationMissingMode === mode.value}
                onchange={onIllustrationMissingModeChange}
                class="radio-input"
              />
              <span class="radio-body">
                <span class="radio-label">{t(mode.labelKey)}</span>
                <span class="radio-hint">{t(mode.hintKey)}</span>
              </span>
            </label>
          {/each}
        </fieldset>
      </section>
    </div>
  {/if}
</div>

<style>
  .typography-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow-y: auto;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4) 1.25rem;
    border-bottom: 1px solid var(--color-border, #e5e7eb);
    position: sticky;
    top: 0;
    background: var(--color-surface, #fff);
    z-index: 1;
  }

  .panel-title {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--color-text, #111827);
    margin: 0;
  }

  .saving-indicator {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #6b7280);
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  .spinner { animation: spin 0.8s linear infinite; }

  .panel-error {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin: var(--space-3) 1.25rem;
    padding: var(--space-2) var(--space-3);
    background: var(--color-error-subtle, #fef2f2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-sm, 6px);
    color: var(--color-error, #ef4444);
    font-size: 0.8125rem;
  }

  .panel-error span { flex: 1; }

  .panel-skeleton {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: 1.25rem;
  }

  .skeleton-line {
    height: 0.75rem;
    border-radius: 4px;
    background: var(--color-border-light, #f3f4f6);
    animation: skeleton-pulse 1.2s ease-in-out infinite;
  }

  .skeleton-line--lg { width: 80%; }
  .skeleton-line--md { width: 60%; }
  .skeleton-line--sm { width: 40%; }

  @keyframes skeleton-pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }

  .panel-body {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding-bottom: var(--space-8);
  }

  .panel-section {
    padding: var(--space-4) 1.25rem;
    border-bottom: 1px solid var(--color-border-light, #f3f4f6);
  }

  .section-title {
    font-size: 0.6875rem;
    font-weight: 700;
    color: var(--color-text-secondary, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin: 0 0 var(--space-3);
  }

  .field-group {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    margin-bottom: var(--space-3);
  }

  .field-group:last-child { margin-bottom: 0; }

  .field-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #374151);
    display: flex;
    align-items: center;
    gap: var(--space-1);
  }

  .field-unit {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #6b7280);
  }

  .field-hint {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #6b7280);
  }

  .field-input, .field-select {
    padding: 0.4375rem 0.625rem;
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: var(--radius-sm, 6px);
    background: var(--color-surface, #fff);
    color: var(--color-text, #111827);
    font-size: var(--text-sm);
    width: 100%;
  }

  .field-input:focus, .field-select:focus {
    outline: none;
    border-color: var(--color-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.12);
  }

  .field-input--sm { max-width: 5rem; }

  .field-range {
    width: 100%;
    accent-color: var(--color-primary, #3b82f6);
  }

  .range-value {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #374151);
    text-align: right;
  }

  .fields-row {
    flex-direction: row;
    gap: var(--space-4);
    align-items: flex-start;
  }

  .field-toggle {
    margin-bottom: var(--space-2);
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    cursor: pointer;
    font-size: var(--text-sm);
    color: var(--color-text, #374151);
  }

  .toggle-checkbox {
    width: 1rem;
    height: 1rem;
    accent-color: var(--color-primary, #3b82f6);
  }

  .toggle-text { user-select: none; }

  .section-hint {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #9ca3af);
    margin: -0.25rem 0 var(--space-3);
  }

  .mode-fieldset {
    border: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .mode-fieldset:disabled { opacity: 0.6; }

  .radio-option {
    display: flex;
    align-items: flex-start;
    gap: 0.625rem;
    padding: 0.625rem 0.75rem;
    border: 1px solid var(--color-border, #e5e7eb);
    border-radius: var(--radius-sm, 6px);
    cursor: pointer;
    transition: border-color var(--duration-fast), background var(--duration-fast);
  }

  .radio-option:hover {
    border-color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.04);
  }

  .radio-option--selected {
    border-color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.06);
  }

  .radio-input {
    margin-top: 0.1875rem;
    accent-color: var(--color-primary, #3b82f6);
    flex-shrink: 0;
  }

  .radio-body {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .radio-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #374151);
    user-select: none;
  }

  .radio-hint {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #9ca3af);
    user-select: none;
  }

  .visually-hidden {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
