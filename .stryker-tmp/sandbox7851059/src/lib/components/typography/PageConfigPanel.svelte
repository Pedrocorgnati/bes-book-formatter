<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { typographyStore, typographyLoadingStore } from '$lib/stores/typography';
  import { toast } from '$lib/stores/toastStore';
  import { ipcSetTypographyConfig } from '$lib/ipc/typography';

  interface Props {
    projectId: string;
  }

  let { projectId }: Props = $props();

  const config = $derived($typographyStore);
  const loading = $derived($typographyLoadingStore);

  // 6 standard formats: label, width (in), height (in)
  const PAGE_FORMATS = $derived([
    { key: 'letter',   label: 'Carta (8.5×11)',  width: 8.5,  height: 11.0 },
    { key: 'a4',       label: 'A4 (8.27×11.69)', width: 8.27, height: 11.69 },
    { key: '6x9',      label: '6×9',             width: 6.0,  height: 9.0 },
    { key: '7x10',     label: '7×10',            width: 7.0,  height: 10.0 },
    { key: '5.5x8.5',  label: '5.5×8.5',         width: 5.5,  height: 8.5 },
    { key: 'a5',       label: 'A5 (5.83×8.27)',  width: 5.83, height: 8.27 },
    { key: 'custom',   label: t('typography.formatCustom'), width: 0, height: 0 },
  ]);

  const CHAPTER_START_OPTIONS = $derived([
    { key: 'odd',        label: t('typography.chapterStartOdd') },
    { key: 'even',       label: t('typography.chapterStartEven') },
    { key: 'continuous', label: t('typography.chapterStartContinuous') },
  ]);

  const DROP_CAP_OPTIONS = $derived([
    { key: 'none',                label: t('typography.dropCapNone') },
    { key: 'first_letter',        label: t('typography.dropCapFirstLetter') },
    { key: 'first_word_small_caps', label: t('typography.dropCapSmallCaps') },
  ]);

  const ORNAMENT_OPTIONS = $derived([
    { key: 'none',      label: t('typography.ornamentNone') },
    { key: 'line',      label: t('typography.ornamentLine') },
    { key: 'vignette',  label: t('typography.ornamentVignette') },
    { key: 'asterisks', label: t('typography.ornamentAsterisks') },
  ]);

  // Derive current format key from config dimensions
  const currentFormatKey = $derived(() => {
    if (!config) return '6x9';
    for (const fmt of PAGE_FORMATS.slice(0, -1)) {
      if (Math.abs(config.pageWidth - fmt.width) < 0.01 && Math.abs(config.pageHeight - fmt.height) < 0.01) {
        return fmt.key;
      }
    }
    return 'custom';
  });

  const isCustomFormat = $derived(currentFormatKey() === 'custom');

  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  function scheduleConfigSave(partial: Record<string, unknown>) {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => persistUpdate(partial), 500);
  }

  async function persistUpdate(partial: Record<string, unknown>) {
    if (!config || !projectId) return;
    typographyLoadingStore.set(true);
    try {
      const updated = await ipcSetTypographyConfig(projectId, partial as never);
      if (updated) typographyStore.set(updated);
      toast.success(t('typography.saveSuccess'));
    } catch {
      toast.error(t('typography.saveError'));
    } finally {
      typographyLoadingStore.set(false);
    }
  }

  function onFormatSelect(key: string) {
    const fmt = PAGE_FORMATS.find(f => f.key === key);
    if (!fmt || fmt.key === 'custom') return;
    scheduleConfigSave({ pageWidth: fmt.width, pageHeight: fmt.height });
  }

  function onWidthChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (isNaN(val) || val <= 0) { toast.error(t('typography.pageValidationError')); return; }
    scheduleConfigSave({ pageWidth: val });
  }

  function onHeightChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (isNaN(val) || val <= 0) { toast.error(t('typography.pageValidationError')); return; }
    scheduleConfigSave({ pageHeight: val });
  }

  function onMarginChange(field: 'marginTop' | 'marginBottom' | 'marginInner' | 'marginOuter', e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (isNaN(val) || val < 0) { toast.error(t('typography.marginValidationError')); return; }
    scheduleConfigSave({ [field]: val });
  }

  function onChapterStartChange(e: Event) {
    scheduleConfigSave({ chapterStart: (e.target as HTMLSelectElement).value });
  }

  function onDropCapChange(e: Event) {
    scheduleConfigSave({ dropCapStyle: (e.target as HTMLSelectElement).value });
  }

  function onOrnamentChange(e: Event) {
    scheduleConfigSave({ ornamentStyle: (e.target as HTMLSelectElement).value });
  }
</script>

<div class="page-config-panel" data-testid="page-config-panel">
  {#if !config}
    <div class="panel-empty" role="status">{t('common.loading')}</div>
  {:else}
    <!-- Formato de página -->
    <section class="config-section" aria-labelledby="section-format">
      <h3 id="section-format" class="section-title">{t('typography.pageFormat')}</h3>

      <fieldset class="format-grid" aria-label={t('typography.pageFormat')}>
        <legend class="sr-only">{t('typography.pageFormat')}</legend>
        {#each PAGE_FORMATS as fmt}
          <label class="format-card" class:format-card--active={currentFormatKey() === fmt.key}>
            <input
              type="radio"
              name="page-format"
              value={fmt.key}
              checked={currentFormatKey() === fmt.key}
              disabled={loading}
              onchange={() => onFormatSelect(fmt.key)}
              class="sr-only"
            />
            <span class="format-label">{fmt.label}</span>
            {#if fmt.key !== 'custom'}
              <span class="format-dims">{fmt.width}×{fmt.height}"</span>
            {/if}
          </label>
        {/each}
      </fieldset>

      {#if isCustomFormat}
        <div class="custom-dims" aria-label={t('typography.customDimensions')}>
          <div class="dim-field">
            <label for="page-width" class="field-label">{t('typography.pageWidth')}</label>
            <input
              id="page-width"
              type="number"
              class="field-input field-input--sm"
              value={config.pageWidth}
              min="3"
              max="17"
              step="0.01"
              disabled={loading}
              onchange={onWidthChange}
              aria-label={t('typography.pageWidthLabel')}
            />
            <span class="field-unit">in</span>
          </div>
          <div class="dim-field">
            <label for="page-height" class="field-label">{t('typography.pageHeight')}</label>
            <input
              id="page-height"
              type="number"
              class="field-input field-input--sm"
              value={config.pageHeight}
              min="4"
              max="17"
              step="0.01"
              disabled={loading}
              onchange={onHeightChange}
              aria-label={t('typography.pageHeightLabel')}
            />
            <span class="field-unit">in</span>
          </div>
        </div>
      {/if}
    </section>

    <!-- Margens -->
    <section class="config-section" aria-labelledby="section-margins">
      <h3 id="section-margins" class="section-title">{t('typography.margins')}</h3>
      <div class="margin-grid">
        {#each [
          { id: 'margin-top',    field: 'marginTop',    label: t('typography.marginTop') },
          { id: 'margin-bottom', field: 'marginBottom', label: t('typography.marginBottom') },
          { id: 'margin-inner',  field: 'marginInner',  label: t('typography.marginInner') },
          { id: 'margin-outer',  field: 'marginOuter',  label: t('typography.marginOuter') },
        ] as margin}
          <div class="margin-field">
            <label for={margin.id} class="field-label">{margin.label}</label>
            <div class="field-with-unit">
              <input
                id={margin.id}
                type="number"
                class="field-input field-input--sm"
                value={(config as unknown as Record<string, unknown>)[margin.field]}
                min="0"
                max="4"
                step="0.0625"
                disabled={loading}
                onchange={(e) => onMarginChange(margin.field as never, e)}
              />
              <span class="field-unit">in</span>
            </div>
          </div>
        {/each}
      </div>
    </section>

    <!-- Início de capítulo -->
    <section class="config-section" aria-labelledby="section-chapter">
      <h3 id="section-chapter" class="section-title">{t('typography.chapterStart')}</h3>
      <select
        id="chapter-start"
        class="field-select"
        value={config.chapterStart}
        disabled={loading}
        onchange={onChapterStartChange}
        aria-label={t('typography.chapterStart')}
      >
        {#each CHAPTER_START_OPTIONS as opt}
          <option value={opt.key}>{opt.label}</option>
        {/each}
      </select>
    </section>

    <!-- Drop Cap e Ornamentos -->
    <section class="config-section" aria-labelledby="section-decoration">
      <h3 id="section-decoration" class="section-title">{t('typography.decoration')}</h3>

      <div class="field-group">
        <label for="drop-cap" class="field-label">{t('typography.dropCap')}</label>
        <select
          id="drop-cap"
          class="field-select"
          value={config.dropCapStyle}
          disabled={loading}
          onchange={onDropCapChange}
        >
          {#each DROP_CAP_OPTIONS as opt}
            <option value={opt.key}>{opt.label}</option>
          {/each}
        </select>
      </div>

      <div class="field-group">
        <label for="ornament" class="field-label">{t('typography.ornament')}</label>
        <select
          id="ornament"
          class="field-select"
          value={config.ornamentStyle}
          disabled={loading}
          onchange={onOrnamentChange}
        >
          {#each ORNAMENT_OPTIONS as opt}
            <option value={opt.key}>{opt.label}</option>
          {/each}
        </select>
      </div>
    </section>
  {/if}
</div>

<style>
  .page-config-panel {
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .config-section {
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--color-border-light, #f3f4f6);
  }

  .section-title {
    font-size: 0.6875rem;
    font-weight: 700;
    color: var(--color-text-secondary, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin: 0 0 0.75rem;
  }

  .panel-empty {
    padding: 2rem;
    text-align: center;
    color: var(--color-text-secondary, #6b7280);
    font-size: 0.875rem;
  }

  /* Format grid */
  .format-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 0.5rem;
    border: none;
    padding: 0;
    margin: 0;
  }

  .format-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.125rem;
    padding: 0.5rem 0.625rem;
    border: 1px solid var(--color-border, #e5e7eb);
    border-radius: var(--radius-sm, 6px);
    cursor: pointer;
    text-align: center;
    transition: all 0.15s;
    background: var(--color-surface, #fff);
  }

  .format-card:hover {
    border-color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.04);
  }

  .format-card--active {
    border-color: var(--color-primary, #3b82f6);
    background: rgba(59, 130, 246, 0.06);
  }

  .format-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text, #111827);
  }

  .format-dims {
    font-size: 0.6875rem;
    color: var(--color-text-secondary, #6b7280);
  }

  /* Custom dimensions */
  .custom-dims {
    display: flex;
    gap: 1rem;
    margin-top: 0.75rem;
  }

  .dim-field {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  /* Margin grid */
  .margin-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .margin-field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .field-with-unit {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  /* Common fields */
  .field-label {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #374151);
  }

  .field-unit {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #6b7280);
  }

  .field-input, .field-select {
    padding: 0.4375rem 0.625rem;
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: var(--radius-sm, 6px);
    background: var(--color-surface, #fff);
    color: var(--color-text, #111827);
    font-size: 0.875rem;
    width: 100%;
  }

  .field-input:focus, .field-select:focus {
    outline: none;
    border-color: var(--color-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.12);
  }

  .field-input--sm { max-width: 5rem; width: 5rem; }

  .field-group {
    margin-bottom: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .field-group:last-child { margin-bottom: 0; }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    border: 0;
  }
</style>
