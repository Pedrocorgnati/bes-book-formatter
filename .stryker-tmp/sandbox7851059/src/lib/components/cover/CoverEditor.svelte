<script lang="ts">
  import type { CoverConfig, CoverConfigInput, CoverTemplate } from '$lib/types/interfaces';
  import CoverTemplateGallery from './CoverTemplateGallery.svelte';
  import { ipcSaveCoverConfig } from '$lib/ipc/cover';
  import { toast } from '$lib/stores/toastStore';
  import { t } from '$lib/i18n/engine';

  interface Props {
    projectId: string;
    config: CoverConfig | null;
    templates: CoverTemplate[];
    onSaved: (config: CoverConfig) => void;
    onPreviewRequest: () => void;
  }

  let { projectId, config, templates, onSaved, onPreviewRequest }: Props = $props();

  // Form state — derived from config or defaults
  let templateId = $state(config?.templateId ?? 'minimal');
  let genre = $state(config?.genre ?? 'fiction');
  let platform = $state<'amazon-kdp' | 'ingram' | 'generic'>(config?.platform ?? 'amazon-kdp');
  let paperType = $state<'white' | 'cream'>(config?.paperType ?? 'white');
  let titleOverride = $state(config?.titleOverride ?? '');
  let subtitle = $state(config?.subtitle ?? '');
  let authorOverride = $state(config?.authorOverride ?? '');
  let backCoverText = $state(config?.backCoverText ?? '');
  let primaryColor = $state(config?.primaryColor ?? '#991B1B');
  let secondaryColor = $state(config?.secondaryColor ?? '#F8F6F0');
  let fontTitle = $state(config?.fontTitle ?? 'Playfair Display');
  let fontAuthor = $state(config?.fontAuthor ?? 'Lato');

  let saving = $state(false);
  let activeSection = $state<'template' | 'text' | 'design'>('template');

  function handleTemplateSelect(tpl: CoverTemplate) {
    templateId = tpl.id;
    genre = tpl.genre;
    primaryColor = tpl.primaryColor;
    secondaryColor = tpl.secondaryColor;
    toast.success(`${t('cover.editor.templateApplied').replace('{name}', tpl.name)}`);
  }

  async function handleSave() {
    saving = true;
    try {
      const input: CoverConfigInput = {
        projectId,
        templateId,
        genre,
        platform,
        paperType,
        titleOverride: titleOverride || null,
        subtitle: subtitle || null,
        authorOverride: authorOverride || null,
        backCoverText: backCoverText || '',
        primaryColor,
        secondaryColor,
        fontTitle,
        fontAuthor,
      };
      const saved = await ipcSaveCoverConfig(input);
      onSaved(saved);
      toast.success(t('cover.editor.configSaved'));
    } catch (e) {
      toast.error(String(e));
    } finally {
      saving = false;
    }
  }
</script>

<div data-testid="cover-editor" class="editor">
  <!-- Section tabs -->
  <div class="editor__tabs" role="tablist">
    <button
      role="tab"
      data-testid="tab-template"
      class="editor__tab"
      class:editor__tab--active={activeSection === 'template'}
      aria-selected={activeSection === 'template'}
      onclick={() => (activeSection = 'template')}
    >{t('cover.editor.tabTemplates')}</button>
    <button
      role="tab"
      data-testid="tab-text"
      class="editor__tab"
      class:editor__tab--active={activeSection === 'text'}
      aria-selected={activeSection === 'text'}
      onclick={() => (activeSection = 'text')}
    >{t('cover.editor.tabTexts')}</button>
    <button
      role="tab"
      data-testid="tab-design"
      class="editor__tab"
      class:editor__tab--active={activeSection === 'design'}
      aria-selected={activeSection === 'design'}
      onclick={() => (activeSection = 'design')}
    >{t('cover.editor.tabDesign')}</button>
  </div>

  <div class="editor__body" role="tabpanel">

    <!-- Templates tab -->
    {#if activeSection === 'template'}
      <CoverTemplateGallery
        {templates}
        selectedId={templateId}
        onSelect={handleTemplateSelect}
      />

      <!-- Platform & paper -->
      <div class="editor__field-group">
        <div class="editor__field">
          <label class="editor__label" for="select-platform">{t('cover.editor.platform')}</label>
          <select id="select-platform" data-testid="select-platform" class="editor__select" bind:value={platform}>
            <option value="amazon-kdp">Amazon KDP</option>
            <option value="ingram">IngramSpark</option>
            <option value="generic">{t('cover.editor.platformGeneric')}</option>
          </select>
        </div>
        <div class="editor__field">
          <label class="editor__label" for="select-paper">{t('cover.editor.paper')}</label>
          <select id="select-paper" data-testid="select-paper" class="editor__select" bind:value={paperType}>
            <option value="white">{t('cover.editor.paperWhite')}</option>
            <option value="cream">{t('cover.editor.paperCream')}</option>
          </select>
        </div>
      </div>
    {/if}

    <!-- Texts tab -->
    {#if activeSection === 'text'}
      <div class="editor__fields">
        <div class="editor__field">
          <label class="editor__label" for="input-title">{t('cover.editor.titleOverride')}</label>
          <input
            id="input-title"
            data-testid="input-title-override"
            class="editor__input"
            type="text"
            placeholder={t('cover.editor.titleOverridePlaceholder')}
            bind:value={titleOverride}
          />
        </div>
        <div class="editor__field">
          <label class="editor__label" for="input-subtitle">{t('cover.editor.subtitle')}</label>
          <input
            id="input-subtitle"
            data-testid="input-subtitle"
            class="editor__input"
            type="text"
            placeholder={t('cover.editor.subtitlePlaceholder')}
            bind:value={subtitle}
          />
        </div>
        <div class="editor__field">
          <label class="editor__label" for="input-author">{t('cover.editor.author')}</label>
          <input
            id="input-author"
            data-testid="input-author-override"
            class="editor__input"
            type="text"
            placeholder={t('cover.editor.authorPlaceholder')}
            bind:value={authorOverride}
          />
        </div>
        <div class="editor__field">
          <label class="editor__label" for="input-back">{t('cover.editor.backCoverText')}</label>
          <textarea
            id="input-back"
            data-testid="input-back-cover-text"
            class="editor__textarea"
            rows={5}
            placeholder={t('cover.editor.backCoverPlaceholder')}
            bind:value={backCoverText}
          ></textarea>
        </div>
      </div>
    {/if}

    <!-- Design tab -->
    {#if activeSection === 'design'}
      <div class="editor__fields">
        <div class="editor__field-row">
          <div class="editor__field">
            <label class="editor__label" for="input-primary-color">{t('cover.editor.primaryColor')}</label>
            <div class="editor__color-wrap">
              <input
                id="input-primary-color"
                data-testid="input-primary-color"
                class="editor__color"
                type="color"
                bind:value={primaryColor}
              />
              <input
                class="editor__input editor__input--mono"
                type="text"
                maxlength={7}
                pattern="#[0-9A-Fa-f]{6}"
                bind:value={primaryColor}
                aria-label={t('cover.editor.primaryColorHex')}
              />
            </div>
          </div>
          <div class="editor__field">
            <label class="editor__label" for="input-secondary-color">{t('cover.editor.secondaryColor')}</label>
            <div class="editor__color-wrap">
              <input
                id="input-secondary-color"
                data-testid="input-secondary-color"
                class="editor__color"
                type="color"
                bind:value={secondaryColor}
              />
              <input
                class="editor__input editor__input--mono"
                type="text"
                maxlength={7}
                pattern="#[0-9A-Fa-f]{6}"
                bind:value={secondaryColor}
                aria-label={t('cover.editor.secondaryColorHex')}
              />
            </div>
          </div>
        </div>

        <div class="editor__field">
          <label class="editor__label" for="input-font-title">{t('cover.editor.fontTitle')}</label>
          <input
            id="input-font-title"
            data-testid="input-font-title"
            class="editor__input"
            type="text"
            bind:value={fontTitle}
          />
        </div>
        <div class="editor__field">
          <label class="editor__label" for="input-font-author">{t('cover.editor.fontAuthor')}</label>
          <input
            id="input-font-author"
            data-testid="input-font-author"
            class="editor__input"
            type="text"
            bind:value={fontAuthor}
          />
        </div>
      </div>
    {/if}
  </div>

  <!-- Action bar -->
  <div class="editor__actions">
    <button
      data-testid="btn-preview-cover"
      class="editor__btn editor__btn--secondary"
      onclick={onPreviewRequest}
    >
      {t('cover.editor.preview')}
    </button>
    <button
      data-testid="btn-save-cover"
      class="editor__btn editor__btn--primary"
      onclick={handleSave}
      disabled={saving}
      aria-busy={saving}
    >
      {saving ? t('cover.editor.saving') : t('cover.editor.save')}
    </button>
  </div>
</div>

<style>
  .editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: 0;
  }

  .editor__tabs {
    display: flex;
    border-bottom: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .editor__tab {
    flex: 1;
    padding: var(--space-2) var(--space-3);
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all var(--duration-fast) ease;
  }

  .editor__tab:hover {
    color: var(--color-text);
  }

  .editor__tab--active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .editor__body {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .editor__fields {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .editor__field-group {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .editor__field-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-2);
  }

  .editor__field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .editor__label {
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-text-secondary);
  }

  .editor__input,
  .editor__select,
  .editor__textarea {
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    font-family: var(--font-sans);
    font-size: var(--text-sm);
    color: var(--color-text);
    transition: border-color var(--duration-fast) ease;
    width: 100%;
    box-sizing: border-box;
  }

  .editor__input:focus,
  .editor__select:focus,
  .editor__textarea:focus {
    outline: none;
    border-color: var(--color-primary);
  }

  .editor__textarea {
    resize: vertical;
    min-height: 80px;
  }

  .editor__input--mono {
    font-family: var(--font-mono, monospace);
    font-size: var(--text-xs);
  }

  .editor__color-wrap {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .editor__color {
    width: 36px;
    height: 36px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    padding: 2px;
    background: transparent;
    flex-shrink: 0;
  }

  .editor__color-wrap .editor__input {
    flex: 1;
  }

  .editor__actions {
    display: flex;
    gap: var(--space-2);
    padding: var(--space-3);
    border-top: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .editor__btn {
    flex: 1;
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: 500;
    cursor: pointer;
    transition: all var(--duration-fast) ease;
    border: none;
  }

  .editor__btn--primary {
    background: var(--color-primary);
    color: white;
  }

  .editor__btn--primary:hover:not(:disabled) {
    opacity: 0.9;
  }

  .editor__btn--primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .editor__btn--secondary {
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    color: var(--color-text);
  }

  .editor__btn--secondary:hover {
    background: var(--color-border);
  }
</style>
