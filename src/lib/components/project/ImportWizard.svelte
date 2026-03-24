<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { ipc } from '$lib/utils/ipc';
  import { projectsStore } from '$lib/stores/projectStore';
  import { toast } from '$lib/stores/toastStore';
  import { Genre } from '$lib/types/enums';
  import type { ApiResponse, BookConfig, BookProject } from '$lib/types';
  import { PROJECT_ROUTES } from '$lib/constants/routes';
  import GenrePicker from './GenrePicker.svelte';

  // ----- State -----

  let step = $state<1 | 2 | 3>(1);
  let selectedPath = $state<string | null>(null);
  let bookConfig = $state<BookConfig | null>(null);
  let configError = $state<string | null>(null);
  let analyzing = $state(false);

  // Step 3 editable fields
  let projectName = $state('');
  let selectedGenre = $state<Genre | null>(null);

  let importing = $state(false);

  // Inline validation
  const nameError = $derived(
    projectName.trim().length > 0 && projectName.trim().length < 3
      ? t('wizard.nameMinLength')
      : null
  );

  // ----- Step 1: select folder -----

  async function handleSelectFolder() {
    try {
      const path = await ipc<string | null>('select_directory');
      if (!path) return;
      selectedPath = path;
    } catch {
      toast.error(t('errors.generic'));
    }
  }

  async function handleNext() {
    if (step === 1) {
      if (!selectedPath) return;
      await analyseFolder();
    } else if (step === 2) {
      step = 3;
    }
  }

  function handleBack() {
    if (step === 2) {
      bookConfig = null;
      configError = null;
      step = 1;
    } else if (step === 3) {
      step = 2;
    }
  }

  // ----- Step 2: analyse folder -----

  async function analyseFolder() {
    analyzing = true;
    configError = null;
    try {
      const res = await ipc<ApiResponse<BookConfig>>('read_book_config', { path: selectedPath! });
      if (res.error) {
        configError = res.error;
        step = 2;
      } else if (res.data) {
        bookConfig = res.data;
        // Pre-fill step 3
        projectName = res.data.title || '';
        selectedGenre = (res.data.genre as Genre) ?? null;
        step = 2;
        // Auto-advance to step 3 if config is clean
        if (!res.error && res.warnings.length === 0) {
          step = 3;
        }
      }
    } catch {
      configError = t('errors.generic');
      step = 2;
    } finally {
      analyzing = false;
    }
  }

  // ----- Step 3: confirm & import -----

  async function handleImport() {
    if (!selectedPath || !bookConfig) return;
    importing = true;
    try {
      const res = await ipc<ApiResponse<BookProject>>('import_project', {
        besRootPath: selectedPath,
      });

      if (res.error || !res.data) {
        toast.error(res.error ?? t('wizard.importError'));
        return;
      }

      const project = res.data;

      // If name or genre differ from detected config, write updated bes-format.yaml
      if (projectName !== (bookConfig.title ?? '') || (selectedGenre && selectedGenre !== bookConfig.genre)) {
        const updatedConfig: BookConfig = {
          ...bookConfig,
          title: projectName,
          genre: selectedGenre ?? bookConfig.genre,
        };
        await ipc<ApiResponse<boolean>>('write_bes_format', {
          projectId: project.id,
          config: updatedConfig,
        });
      }

      projectsStore.addProject(project);
      toast.success(t('wizard.importSuccess'));
      goto(PROJECT_ROUTES.ROOT(project.id));
    } catch {
      toast.error(t('wizard.importError'));
    } finally {
      importing = false;
    }
  }

  // ----- Derived -----

  const canProceedStep1 = $derived(selectedPath != null && !analyzing);
  const canProceedStep2 = $derived(bookConfig != null && !analyzing);
  const canImport = $derived(projectName.trim().length > 0 && selectedGenre != null && !importing);
</script>

<!-- Step indicator -->
<div data-testid="wizard-steps" class="wizard-steps" aria-label="Progresso do assistente">
  {#each ([1, 2, 3] as const) as s}
    <div
      class="wizard-steps__step"
      class:wizard-steps__step--active={step === s}
      class:wizard-steps__step--done={step > s}
      aria-current={step === s ? 'step' : undefined}
    >
      <span class="wizard-steps__dot">{step > s ? '✓' : s}</span>
      <span class="wizard-steps__label">
        {t(`wizard.step${s}Title`)}
      </span>
    </div>
    {#if s < 3}
      <div class="wizard-steps__connector" aria-hidden="true"></div>
    {/if}
  {/each}
</div>

<!-- Step content -->
<div class="wizard-body">
  <!-- Step 1: Select folder -->
  {#if step === 1}
    <section data-testid="wizard-step-1" class="wizard-step">
      <p class="wizard-step__desc">{t('wizard.step1Desc')}</p>

      <button
        data-testid="wizard-select-folder"
        type="button"
        class="btn btn--primary wizard-step__folder-btn"
        onclick={handleSelectFolder}
        disabled={analyzing}
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        {t('wizard.selectFolderBtn')}
      </button>

      {#if selectedPath}
        <div data-testid="wizard-selected-path" class="wizard-step__path">
          <span class="wizard-step__path-label">{t('wizard.selectedPath')}:</span>
          <code class="wizard-step__path-value">{selectedPath}</code>
        </div>
      {:else}
        <p class="wizard-step__hint">{t('wizard.noFolderSelected')}</p>
      {/if}
    </section>

  <!-- Step 2: Analyse -->
  {:else if step === 2}
    <section data-testid="wizard-step-2" class="wizard-step">
      <p class="wizard-step__desc">{t('wizard.step2Desc')}</p>

      {#if analyzing}
        <div data-testid="wizard-analyzing" class="wizard-step__analyzing">
          <div class="spinner" aria-hidden="true"></div>
          <span>{t('wizard.analyzing')}</span>
        </div>
      {:else if configError}
        <div data-testid="wizard-bes-error" class="wizard-step__error" role="alert">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="12"/>
            <line x1="12" y1="16" x2="12.01" y2="16"/>
          </svg>
          {t('wizard.noBesDetected')}
        </div>
      {:else if bookConfig}
        <dl data-testid="wizard-config-summary" class="wizard-step__summary">
          <div class="wizard-step__summary-row">
            <dt>{t('wizard.projectName')}</dt>
            <dd>{bookConfig.title}</dd>
          </div>
          {#if bookConfig.genre}
            <div class="wizard-step__summary-row">
              <dt>{t('wizard.selectGenre')}</dt>
              <dd>{t(`genre.${bookConfig.genre}`)}</dd>
            </div>
          {/if}
        </dl>
      {/if}
    </section>

  <!-- Step 3: Confirm -->
  {:else if step === 3}
    <section data-testid="wizard-step-3" class="wizard-step">
      <p class="wizard-step__desc">{t('wizard.step3Desc')}</p>

      <div class="wizard-step__field">
        <label for="wizard-project-name" class="wizard-step__field-label">
          {t('wizard.projectName')}
        </label>
        <input
          id="wizard-project-name"
          data-testid="wizard-project-name-input"
          type="text"
          class="wizard-step__input"
          class:wizard-step__input--error={nameError}
          bind:value={projectName}
          maxlength={120}
          aria-invalid={!!nameError}
          aria-describedby={nameError ? 'wizard-name-error' : undefined}
        />
        {#if nameError}
          <p id="wizard-name-error" class="wizard-step__field-error" role="alert">{nameError}</p>
        {/if}
      </div>

      <div class="wizard-step__field">
        <p class="wizard-step__field-label">{t('wizard.selectGenre')}</p>
        <GenrePicker
          value={selectedGenre}
          onChange={(g) => { selectedGenre = g; }}
        />
      </div>
    </section>
  {/if}
</div>

<!-- Footer actions -->
<div data-testid="wizard-footer" class="wizard-footer">
  {#if step > 1}
    <button
      data-testid="wizard-back"
      type="button"
      class="btn btn--ghost"
      onclick={handleBack}
      disabled={analyzing || importing}
    >
      {t('wizard.backBtn')}
    </button>
  {:else}
    <div></div>
  {/if}

  {#if step < 3}
    <button
      data-testid="wizard-next"
      type="button"
      class="btn btn--primary"
      onclick={handleNext}
      disabled={step === 1 ? !canProceedStep1 : !canProceedStep2}
    >
      {t('wizard.nextBtn')}
    </button>
  {:else}
    <button
      data-testid="wizard-import"
      type="button"
      class="btn btn--primary"
      onclick={handleImport}
      disabled={!canImport}
    >
      {importing ? t('common.saving') : t('wizard.importBtn')}
    </button>
  {/if}
</div>

<style>
  /* Step indicator */
  .wizard-steps {
    display: flex;
    align-items: center;
    gap: 0;
    margin-bottom: var(--space-8);
  }

  .wizard-steps__step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-1);
    flex: 1;
    min-width: 0;
  }

  .wizard-steps__dot {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-full);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--text-sm);
    font-weight: 700;
    background: var(--color-border);
    color: var(--color-text-muted);
    transition: background var(--duration-fast), color var(--duration-fast);
  }

  .wizard-steps__step--active .wizard-steps__dot {
    background: var(--color-primary);
    color: #fff;
  }

  .wizard-steps__step--done .wizard-steps__dot {
    background: var(--color-success, #22c55e);
    color: #fff;
  }

  .wizard-steps__label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100px;
  }

  .wizard-steps__step--active .wizard-steps__label {
    color: var(--color-primary);
    font-weight: 600;
  }

  .wizard-steps__connector {
    flex: 1;
    height: 2px;
    background: var(--color-border);
    margin: 0 var(--space-2);
    align-self: flex-start;
    margin-top: 13px;
  }

  /* Body */
  .wizard-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }

  .wizard-step {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .wizard-step__desc {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
  }

  .wizard-step__folder-btn {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    align-self: flex-start;
  }

  .wizard-step__path {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3);
    background: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .wizard-step__path-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    font-weight: 500;
  }

  .wizard-step__path-value {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text);
    word-break: break-all;
  }

  .wizard-step__hint {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    margin: 0;
  }

  .wizard-step__analyzing {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    padding: var(--space-4) 0;
  }

  .wizard-step__error {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    font-size: var(--text-sm);
    color: var(--color-error, #ef4444);
    padding: var(--space-3);
    background: color-mix(in srgb, var(--color-error, #ef4444) 8%, transparent);
    border-radius: var(--radius-md);
    border: 1px solid color-mix(in srgb, var(--color-error, #ef4444) 30%, transparent);
  }

  .wizard-step__summary {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin: 0;
  }

  .wizard-step__summary-row {
    display: flex;
    gap: var(--space-3);
  }

  .wizard-step__summary-row dt {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text-muted);
    min-width: 100px;
  }

  .wizard-step__summary-row dd {
    font-size: var(--text-sm);
    color: var(--color-text);
    margin: 0;
  }

  .wizard-step__field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .wizard-step__field-label {
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text);
    margin: 0;
  }

  .wizard-step__input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-base);
    color: var(--color-text);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    outline: none;
    transition: border-color var(--duration-fast);
    font-family: var(--font-sans);
  }

  .wizard-step__input--error {
    border-color: var(--color-error, #ef4444);
  }

  .wizard-step__field-error {
    font-size: var(--text-xs);
    color: var(--color-error, #ef4444);
    margin: var(--space-1) 0 0;
  }

  .wizard-step__input:focus {
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--color-primary) 20%, transparent);
  }

  /* Footer */
  .wizard-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: var(--space-6);
    border-top: 1px solid var(--color-border);
    margin-top: var(--space-6);
    gap: var(--space-3);
  }

  /* Spinner */
  .spinner {
    width: 18px;
    height: 18px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Button resets — rely on global .btn classes */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
    font-weight: 500;
    border-radius: var(--radius-md);
    border: none;
    cursor: pointer;
    transition:
      background var(--duration-fast),
      color var(--duration-fast),
      opacity var(--duration-fast);
    font-family: var(--font-sans);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn--primary {
    background: var(--color-primary);
    color: #fff;
  }

  .btn--primary:hover:not(:disabled) {
    background: color-mix(in srgb, var(--color-primary) 85%, #000);
  }

  .btn--ghost {
    background: transparent;
    color: var(--color-text-muted);
    border: 1px solid var(--color-border);
  }

  .btn--ghost:hover:not(:disabled) {
    background: var(--color-bg-secondary);
    color: var(--color-text);
  }
</style>
