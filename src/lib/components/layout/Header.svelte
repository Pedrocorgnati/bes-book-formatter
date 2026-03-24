<script lang="ts">
  import { goto } from '$app/navigation';
  import ThemeToggle from '$lib/components/ui/ThemeToggle.svelte';
  import LanguageSelector from '$lib/components/ui/LanguageSelector.svelte';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import { ROUTES } from '$lib/constants/routes';

  interface Props {
    breadcrumb?: string;
    showImportCta?: boolean;
    onImportClick?: () => void;
  }

  let {
    breadcrumb = '',
    showImportCta = false,
    onImportClick = () => goto(ROUTES.IMPORT),
  }: Props = $props();

  const project = $derived($projectsStore.current);
</script>

<!-- Logo + breadcrumb + controles -->
<div data-testid="header-logo" class="header-logo">
  <!-- @ASSET_PLACEHOLDER
  name: logo-header
  type: image
  extension: svg
  format: flexivel
  dimensions: 120x28
  description: Logo BES Book Formatter para o header da aplicação. Tipografia editorial com iniciais "BES" em estilo serifado seguido de "Book Formatter" em sans-serif. Deve ter contraste adequado em fundo branco e escuro.
  context: Header do app desktop — exibido sempre
  style: Editorial, minimalista, tipográfico
  mood: Profissional, formal, confiável
  colors: primary (#991B1B light / #EF4444 dark)
  elements: Tipografia, possível ícone de livro aberto
  avoid: Ícone complexo, gradientes, muitas cores
  -->
  <span class="header-brand" aria-label={t('a11y.logoAlt')}>
    <span class="header-brand__bes">BES</span>
    <span class="header-brand__rest"> Book Formatter</span>
  </span>
</div>

<div data-testid="header-breadcrumb" class="header-breadcrumb" aria-label={t('a11y.breadcrumb')}>
  {#if project}
    <span class="breadcrumb-project">{project.name}</span>
    {#if breadcrumb}
      <span class="breadcrumb-sep" aria-hidden="true">/</span>
      <span class="breadcrumb-current">{breadcrumb}</span>
    {/if}
  {:else}
    <span class="breadcrumb-current">{breadcrumb || t('nav.dashboard')}</span>
  {/if}
</div>

<div data-testid="header-actions" class="header-actions">
  {#if showImportCta}
    <button data-testid="header-import-button" class="btn btn--primary btn--sm" onclick={onImportClick}>
      {t('nav.import')}
    </button>
  {/if}
  <LanguageSelector />
  <ThemeToggle />
</div>

<style>
  .header-logo {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .header-brand {
    font-size: var(--text-base);
    font-weight: 700;
    letter-spacing: -0.01em;
    user-select: none;
  }

  .header-brand__bes {
    color: var(--color-primary);
    font-family: var(--font-serif);
  }

  .header-brand__rest {
    color: var(--color-text);
    font-size: var(--text-sm);
    font-weight: 500;
    font-family: var(--font-sans);
    margin-left: var(--space-1);
  }

  .header-breadcrumb {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex: 1;
    padding: 0 var(--space-4);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    min-width: 0;
  }

  .breadcrumb-project {
    font-weight: 500;
    color: var(--color-text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .breadcrumb-sep {
    color: var(--color-border);
  }

  .breadcrumb-current {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-shrink: 0;
    margin-left: auto;
  }

  /* Botões globais (definidos aqui para uso no header) */
  :global(.btn) {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    font-family: var(--font-sans);
    font-weight: 500;
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    text-decoration: none;
    transition:
      background-color var(--duration-fast) ease,
      color var(--duration-fast) ease,
      opacity var(--duration-fast) ease;
  }

  :global(.btn:focus-visible) {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
  }

  :global(.btn:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
  }

  :global(.btn--primary) {
    background: var(--color-primary);
    color: var(--color-on-primary);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
  }

  :global(.btn--primary:hover) {
    background: var(--color-primary-hover);
  }

  :global(.btn--ghost) {
    background: transparent;
    color: var(--color-text-secondary);
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-sm);
  }

  :global(.btn--ghost:hover) {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }

  :global(.btn--sm) {
    padding: var(--space-1) var(--space-3);
    font-size: var(--text-xs);
  }

  :global(.btn--destructive) {
    background: var(--color-danger);
    color: var(--color-on-danger);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
  }

  :global(.btn--destructive:hover) {
    opacity: 0.9;
  }
</style>
