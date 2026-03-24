<script lang="ts">
  import { t } from '$lib/i18n/engine';

  interface Props {
    value: string;
    onchange: (genre: string) => void;
    disabled?: boolean;
  }

  let { value, onchange, disabled = false }: Props = $props();

  const presets = [
    { value: 'nonfiction',  label: 'Não-ficção',     font: 'Source Serif 4', size: '11pt', page: '6×9' },
    { value: 'fiction',     label: 'Ficção',          font: 'EB Garamond',   size: '11pt', page: '5.5×8.5' },
    { value: 'romance',     label: 'Romance',         font: 'EB Garamond',   size: '11pt', page: '5.5×8.5' },
    { value: 'technical',   label: 'Técnico',         font: 'Source Serif 4', size: '10pt', page: '7×10' },
    { value: 'children',    label: 'Infantil',        font: 'Source Serif 4', size: '14pt', page: '8.5×8.5' },
  ];

  const selectedPreset = $derived(presets.find((p) => p.value === value) ?? presets[0]);
</script>

<div class="genre-preset-picker" data-testid="genre-preset-picker">
  <label class="preset-label" for="genre-preset-select">
    {t('typography.genrePreset')}
  </label>
  <select
    id="genre-preset-select"
    class="preset-select"
    {value}
    {disabled}
    onchange={(e) => onchange((e.target as HTMLSelectElement).value)}
    aria-label={t('typography.genrePreset')}
  >
    {#each presets as preset}
      <option value={preset.value}>{preset.label}</option>
    {/each}
  </select>

  {#if selectedPreset}
    <div class="preset-info" aria-live="polite">
      <span class="preset-info__item">
        <span class="preset-info__key">Fonte:</span>
        <span class="preset-info__val">{selectedPreset.font}</span>
      </span>
      <span class="preset-info__item">
        <span class="preset-info__key">Tamanho:</span>
        <span class="preset-info__val">{selectedPreset.size}</span>
      </span>
      <span class="preset-info__item">
        <span class="preset-info__key">Página:</span>
        <span class="preset-info__val">{selectedPreset.page}</span>
      </span>
    </div>
  {/if}
</div>

<style>
  .genre-preset-picker {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs, 0.25rem);
  }

  .preset-label {
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--color-text-secondary, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .preset-select {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border, #d1d5db);
    border-radius: var(--radius-sm, 6px);
    background: var(--color-surface, #ffffff);
    color: var(--color-text, #111827);
    font-size: var(--text-sm);
    cursor: pointer;
    transition: border-color var(--duration-fast);
  }

  .preset-select:focus {
    outline: none;
    border-color: var(--color-primary, #3b82f6);
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
  }

  .preset-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .preset-info {
    display: flex;
    gap: var(--space-3);
    flex-wrap: wrap;
    padding: var(--space-2) var(--space-3);
    background: var(--color-surface-secondary, #f9fafb);
    border-radius: var(--radius-sm, 6px);
    font-size: 0.8125rem;
  }

  .preset-info__item {
    display: flex;
    gap: var(--space-1);
  }

  .preset-info__key {
    color: var(--color-text-secondary, #6b7280);
  }

  .preset-info__val {
    color: var(--color-text, #111827);
    font-weight: 500;
  }
</style>
