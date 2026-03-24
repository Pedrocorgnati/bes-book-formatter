<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import { IllustrationState } from '$lib/types/enums';
  import type { Illustration } from '$lib/types/interfaces';

  interface Props {
    illustration: Illustration;
    onEditAltText?: (id: string) => void;
    onReprocess?: (id: string) => void;
    onDelete?: (id: string) => void;
  }

  let { illustration, onEditAltText, onReprocess, onDelete }: Props = $props();

  const stateLabel = $derived({
    [IllustrationState.PENDING]:  t('illustrations.statePending'),
    [IllustrationState.IMPORTED]: t('illustrations.stateImported'),
    [IllustrationState.LINKED]:   t('illustrations.stateLinked'),
    [IllustrationState.ERROR]:    t('illustrations.stateError'),
  } as Record<string, string>);

  const stateClass: Record<string, string> = {
    [IllustrationState.PENDING]:  'badge--gray',
    [IllustrationState.IMPORTED]: 'badge--amber',
    [IllustrationState.LINKED]:   'badge--green',
    [IllustrationState.ERROR]:    'badge--red',
  };
</script>

<article class="illustration-card" data-testid="illustration-card-{illustration.id}" aria-label={t('illustrations.ariaIllustration').replace('{name}', illustration.placeholderName)}>
  <!-- Thumbnail ou placeholder -->
  <div class="card-thumb">
    {#if illustration.imagePath && illustration.state !== IllustrationState.PENDING}
      <img
        src={illustration.imagePath}
        alt={illustration.altText ?? illustration.placeholderName}
        class="card-img"
        loading="lazy"
      />
    {:else}
      <div class="card-placeholder" aria-label={t('illustrations.ariaAwaitingImport')}>
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
          <circle cx="8.5" cy="8.5" r="1.5" />
          <polyline points="21 15 16 10 5 21" />
        </svg>
      </div>
    {/if}

    <!-- Badge de estado -->
    <span class="badge {stateClass[illustration.state] ?? 'badge--gray'}">
      {stateLabel[illustration.state] ?? illustration.state}
    </span>
  </div>

  <!-- Informações -->
  <div class="card-info">
    <p class="card-name" title={illustration.placeholderName}>{illustration.placeholderName}</p>
    {#if illustration.validatedDpi}
      <p class="card-dpi">{illustration.validatedDpi} DPI</p>
    {/if}
  </div>

  <!-- Ações -->
  <div class="card-actions">
    {#if illustration.state === IllustrationState.IMPORTED}
      <button
        class="btn-action btn-action--primary"
        onclick={() => onEditAltText?.(illustration.id)}
        aria-label={t('illustrations.ariaEditAltText').replace('{name}', illustration.placeholderName)}
      >
        {t('illustrations.btnAltText')}
      </button>
    {/if}
    {#if illustration.state === IllustrationState.ERROR}
      <button
        class="btn-action btn-action--warning"
        onclick={() => onReprocess?.(illustration.id)}
        aria-label={t('illustrations.ariaReprocess').replace('{name}', illustration.placeholderName)}
      >
        {t('illustrations.btnReprocess')}
      </button>
    {/if}
    {#if onDelete}
      <button
        class="btn-action btn-action--danger"
        onclick={() => {
          if (confirm(t('illustrations.confirmDelete').replace('{name}', illustration.placeholderName))) {
            onDelete(illustration.id);
          }
        }}
        aria-label={t('illustrations.ariaDelete').replace('{name}', illustration.placeholderName)}
      >
        {t('illustrations.btnDelete')}
      </button>
    {/if}
  </div>
</article>

<style>
  .illustration-card {
    display: flex;
    flex-direction: column;
    background: var(--color-surface, #fff);
    border: 1px solid var(--color-border, #e5e7eb);
    border-radius: var(--radius-md, 8px);
    overflow: hidden;
    transition: box-shadow var(--duration-fast), border-color var(--duration-fast);
  }

  .illustration-card:hover {
    box-shadow: 0 2px 8px rgba(0,0,0,0.06);
    border-color: var(--color-border-hover, #d1d5db);
  }

  .card-thumb {
    position: relative;
    aspect-ratio: 4/3;
    background: var(--color-surface-secondary, #f9fafb);
    overflow: hidden;
  }

  .card-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .card-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    color: var(--color-text-tertiary, #9ca3af);
  }

  .badge {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    padding: 0.125rem var(--space-2);
    border-radius: 9999px;
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .badge--gray   { background: #f3f4f6; color: #374151; }
  .badge--amber  { background: #fef3c7; color: #92400e; }
  .badge--green  { background: #d1fae5; color: #065f46; }
  .badge--red    { background: #fee2e2; color: #991b1b; }

  .card-info {
    padding: var(--space-2) var(--space-3);
    flex: 1;
  }

  .card-name {
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--color-text, #111827);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .card-dpi {
    font-size: var(--text-xs);
    color: var(--color-text-secondary, #6b7280);
    margin: var(--space-1) 0 0;
  }

  .card-actions {
    padding: var(--space-2) var(--space-3);
    border-top: 1px solid var(--color-border-light, #f3f4f6);
    display: flex;
    gap: var(--space-2);
  }

  .btn-action {
    flex: 1;
    padding: 0.375rem 0.625rem;
    border-radius: var(--radius-sm, 6px);
    font-size: var(--text-xs);
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: background var(--duration-fast);
  }

  .btn-action--primary {
    background: var(--color-primary, #3b82f6);
    color: #fff;
  }
  .btn-action--primary:hover { background: var(--color-primary-hover, #2563eb); }

  .btn-action--warning {
    background: #fef3c7;
    color: #92400e;
  }
  .btn-action--warning:hover { background: #fde68a; }

  .btn-action--danger {
    color: var(--color-error, #dc2626);
    border-color: var(--color-error, #dc2626);
  }
  .btn-action--danger:hover {
    background: var(--color-error, #dc2626);
    color: white;
  }
</style>
