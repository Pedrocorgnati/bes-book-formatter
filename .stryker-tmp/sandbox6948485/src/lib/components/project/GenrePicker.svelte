<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { Genre } from '$lib/types/enums';

  interface Props {
    value: Genre | null;
    onChange: (genre: Genre) => void;
  }

  const { value = null, onChange }: Props = $props();

  interface GenreOption {
    value: Genre;
    icon: string;
    fontFamily: string;
  }

  const GENRES: GenreOption[] = [
    { value: Genre.NONFICTION,  icon: '📖', fontFamily: 'var(--font-serif)' },
    { value: Genre.SELF_HELP,   icon: '🌱', fontFamily: 'var(--font-serif)' },
    { value: Genre.TECHNICAL,   icon: '⚙️',  fontFamily: 'var(--font-mono)' },
    { value: Genre.ACADEMIC,    icon: '🎓', fontFamily: 'var(--font-mono)' },
    { value: Genre.FICTION,     icon: '✨', fontFamily: 'var(--font-serif)' },
    { value: Genre.ROMANCE,     icon: '❤️',  fontFamily: 'var(--font-serif)' },
    { value: Genre.BUSINESS,    icon: '💼', fontFamily: 'var(--font-sans)' },
    { value: Genre.MANAGEMENT,  icon: '📊', fontFamily: 'var(--font-sans)' },
    { value: Genre.CHILDREN,    icon: '🌈', fontFamily: 'var(--font-sans)' },
    { value: Genre.YA,          icon: '🚀', fontFamily: 'var(--font-serif)' },
  ];
</script>

<div data-testid="genre-picker" class="genre-picker" role="radiogroup" aria-label={t('wizard.selectGenre')}>
  {#each GENRES as option (option.value)}
    <button
      data-testid="genre-option-{option.value}"
      type="button"
      role="radio"
      aria-checked={value === option.value}
      class="genre-picker__option"
      class:genre-picker__option--selected={value === option.value}
      onclick={() => onChange(option.value)}
    >
      <span class="genre-picker__icon" aria-hidden="true">{option.icon}</span>
      <span class="genre-picker__label" style="font-family: {option.fontFamily}">
        {t(`genre.${option.value}`)}
      </span>
    </button>
  {/each}
</div>

<style>
  .genre-picker {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: var(--space-2);
  }

  .genre-picker__option {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-2);
    background: var(--color-surface);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition:
      border-color var(--duration-fast) ease,
      background var(--duration-fast) ease;
  }

  .genre-picker__option:hover {
    border-color: var(--color-primary);
    background: var(--color-bg-secondary);
  }

  .genre-picker__option:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  .genre-picker__option--selected {
    border-color: var(--color-primary);
    background: color-mix(in srgb, var(--color-primary) 10%, transparent);
  }

  .genre-picker__icon {
    font-size: var(--text-2xl);
    line-height: 1;
  }

  .genre-picker__label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text);
    text-align: center;
  }
</style>
