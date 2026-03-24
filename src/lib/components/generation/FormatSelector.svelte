<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import type { FormatSelection } from '$lib/types';
  import { OutputFormat, Platform } from '$lib/types/enums';
  import { FORMAT_TABS, type FormatTab } from '$lib/constants/ui-tabs';

  interface Props {
    onChange: (selection: FormatSelection) => void;
    disabled?: boolean;
  }

  let { onChange, disabled = false }: Props = $props();

  let activeTab = $state<FormatTab>(FORMAT_TABS.PRESET);

  // Adaptive Distribution Presets
  const PRESETS = [
    {
      key: 'kdp_complete',
      label: 'KDP Completo',
      platform: Platform.KDP,
      formats: [OutputFormat.EPUB3, OutputFormat.PDF_PRINT, OutputFormat.PDF_EBOOK],
      description: 'EPUB 3.3 + PDF/X-1a (impressão) + PDF/A-3 (e-book)',
    },
    {
      key: 'ingramspark',
      label: 'IngramSpark',
      platform: Platform.INGRAM_SPARK,
      formats: [OutputFormat.EPUB3, OutputFormat.PDF_PRINT],
      description: 'EPUB 3.3 + PDF/X-4 para distribuição expandida',
    },
    {
      key: 'max_compat',
      label: 'Máxima Compatibilidade',
      platform: Platform.GENERIC,
      formats: [OutputFormat.EPUB3, OutputFormat.PDF_PRINT, OutputFormat.PDF_EBOOK, OutputFormat.DOCX, OutputFormat.TXT],
      description: 'Todos os formatos principais',
    },
    {
      key: 'review',
      label: 'Revisão',
      platform: Platform.GENERIC,
      formats: [OutputFormat.DOCX, OutputFormat.PDF_EBOOK],
      description: 'DOCX (editor) + PDF/A-3 (leitura)',
    },
  ];

  const ALL_FORMATS = [
    { key: OutputFormat.EPUB3, label: 'EPUB 3.3' },
    { key: OutputFormat.PDF_PRINT, label: 'PDF Impressão' },
    { key: OutputFormat.PDF_EBOOK, label: 'PDF E-book' },
    { key: OutputFormat.DOCX, label: 'DOCX (Word)' },
    { key: OutputFormat.MARKDOWN_CLEAN, label: 'Markdown' },
    { key: OutputFormat.TXT, label: 'TXT' },
    { key: OutputFormat.HTML5, label: 'HTML5' },
    { key: OutputFormat.JSON_STRUCTURAL, label: 'JSON' },
  ];

  const PLATFORMS = [
    { key: Platform.KDP, label: 'Amazon KDP' },
    { key: Platform.KDP_PRINT, label: 'KDP Impressão' },
    { key: Platform.INGRAM_SPARK, label: 'IngramSpark' },
    { key: Platform.KOBO, label: 'Kobo' },
    { key: Platform.APPLE_BOOKS, label: 'Apple Books' },
    { key: Platform.GENERIC, label: 'Genérico' },
  ];

  let selectedPreset = $state<string | null>(PRESETS[0].key);
  let manualFormats = $state<Set<OutputFormat>>(new Set([OutputFormat.EPUB3]));
  let manualPlatform = $state<Platform>(Platform.KDP);

  function selectPreset(key: string) {
    selectedPreset = key;
    const preset = PRESETS.find((p) => p.key === key)!;
    emit(preset.formats, preset.platform, key);
  }

  function toggleManualFormat(key: string) {
    const next = new Set(manualFormats);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    manualFormats = next;
    emit([...manualFormats], manualPlatform, null);
  }

  function onManualPlatformChange(e: Event) {
    manualPlatform = (e.target as HTMLSelectElement).value;
    emit([...manualFormats], manualPlatform, null);
  }

  function emit(formats: string[], platform: string, preset: string | null) {
    onChange({ formats, platform, preset });
  }

  // Tracks whether the current selection is valid (preset always valid; manual requires ≥1 format)
  let canProceed = $derived(activeTab === FORMAT_TABS.PRESET || manualFormats.size > 0);

  // Emit initial selection on mount
  $effect(() => {
    if (activeTab === FORMAT_TABS.PRESET && selectedPreset) {
      const preset = PRESETS.find((p) => p.key === selectedPreset)!;
      emit(preset.formats, preset.platform, selectedPreset);
    } else {
      emit([...manualFormats], manualPlatform, null);
    }
  });
</script>

<div data-testid="format-selector" class="format-selector" class:format-selector--disabled={disabled}>
  <!-- Tabs -->
  <div class="format-selector__tabs" role="tablist" aria-label="Modo de seleção">
    <button
      role="tab"
      aria-selected={activeTab === FORMAT_TABS.PRESET}
      class="format-selector__tab"
      class:format-selector__tab--active={activeTab === FORMAT_TABS.PRESET}
      onclick={() => (activeTab = FORMAT_TABS.PRESET)}
      {disabled}
    >
      {t('generation.adaptiveTitle')}
    </button>
    <button
      role="tab"
      aria-selected={activeTab === FORMAT_TABS.MANUAL}
      class="format-selector__tab"
      class:format-selector__tab--active={activeTab === FORMAT_TABS.MANUAL}
      onclick={() => (activeTab = FORMAT_TABS.MANUAL)}
      {disabled}
    >
      {t('generation.manualTab')}
    </button>
  </div>

  <!-- Preset cards -->
  {#if activeTab === FORMAT_TABS.PRESET}
    <div class="format-selector__presets" role="radiogroup" aria-label="Presets de geração">
      {#each PRESETS as preset (preset.key)}
        <button
          role="radio"
          aria-checked={selectedPreset === preset.key}
          class="preset-card"
          class:preset-card--selected={selectedPreset === preset.key}
          onclick={() => selectPreset(preset.key)}
          {disabled}
        >
          <div class="preset-card__header">
            <span class="preset-card__label">{preset.label}</span>
            {#if selectedPreset === preset.key}
              <span class="preset-card__check" aria-hidden="true">✓</span>
            {/if}
          </div>
          <p class="preset-card__desc">{preset.description}</p>
          <div class="preset-card__formats" aria-label="Formatos incluídos">
            {#each preset.formats as fmt}
              <span class="format-chip">{fmt.toUpperCase()}</span>
            {/each}
          </div>
        </button>
      {/each}
    </div>

  <!-- Manual selection -->
  {:else}
    <div class="format-selector__manual">
      <div class="format-selector__platform-row">
        <label for="manual-platform" class="label">{t('generation.selectPlatform')}</label>
        <select
          id="manual-platform"
          class="select"
          value={manualPlatform}
          onchange={onManualPlatformChange}
          {disabled}
        >
          {#each PLATFORMS as p (p.key)}
            <option value={p.key}>{p.label}</option>
          {/each}
        </select>
      </div>

      <fieldset class="format-selector__checkboxes" {disabled}>
        <legend class="label">{t('generation.selectFormat')}</legend>
        <div class="format-selector__grid">
          {#each ALL_FORMATS as fmt (fmt.key)}
            <label class="checkbox-label">
              <input
                type="checkbox"
                checked={manualFormats.has(fmt.key)}
                onchange={() => toggleManualFormat(fmt.key)}
                {disabled}
              />
              <span>{fmt.label}</span>
            </label>
          {/each}
        </div>
        {#if !canProceed}
          <p class="format-selector__warning" role="alert">
            {t('generation.selectAtLeastOne')}
          </p>
        {/if}
      </fieldset>
    </div>
  {/if}
</div>

<style>
  .format-selector {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .format-selector--disabled {
    opacity: 0.6;
    pointer-events: none;
  }

  .format-selector__tabs {
    display: flex;
    border-bottom: 1px solid var(--color-border);
    gap: var(--space-1);
  }

  .format-selector__tab {
    padding: var(--space-2) var(--space-4);
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin-bottom: -1px;
  }

  .format-selector__tab--active {
    color: var(--color-accent);
    border-bottom-color: var(--color-accent);
    font-weight: 500;
  }

  .format-selector__presets {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--space-3);
  }

  .preset-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-4);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    cursor: pointer;
    text-align: left;
    transition: border-color var(--duration-fast);
  }

  .preset-card--selected {
    border-color: var(--color-accent);
    background: var(--color-accent-subtle, var(--color-surface-raised));
  }

  .preset-card__header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .preset-card__label {
    font-weight: 600;
    font-size: var(--text-sm);
  }

  .preset-card__check {
    color: var(--color-accent);
    font-weight: 700;
  }

  .preset-card__desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .preset-card__formats {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-top: auto;
  }

  .format-chip {
    padding: 2px var(--space-2);
    background: var(--color-border);
    border-radius: var(--radius-sm);
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--color-text-muted);
  }

  .preset-card--selected .format-chip {
    background: var(--color-accent);
    color: white;
  }

  .format-selector__manual {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .format-selector__platform-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .format-selector__checkboxes {
    border: none;
    padding: 0;
    margin: 0;
  }

  .format-selector__grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: var(--space-2);
    margin-top: var(--space-2);
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    cursor: pointer;
  }

  .format-selector__warning {
    margin: var(--space-2) 0 0;
    font-size: var(--text-xs);
    color: var(--color-warning, #b45309);
  }
</style>
