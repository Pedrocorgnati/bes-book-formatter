<script lang="ts">
  import type { CoverConfig } from '$lib/types/interfaces';
  import { t } from '$lib/i18n/engine';
  import { ipcGenerateCoverPdf, ipcExportCoverImage } from '$lib/ipc/cover';
  import { toast } from '$lib/stores/toastStore';

  interface Props {
    projectId: string;
    config: CoverConfig | null;
    previewTrigger?: number;
  }

  let { projectId, config, previewTrigger = 0 }: Props = $props();

  // Respond to external trigger (from CoverEditor "Visualizar" button)
  $effect(() => {
    if (previewTrigger > 0) {
      generatePreview();
    }
  });

  // base64 PNG preview (returned from generate_cover_pdf)
  let previewBase64 = $state<string | null>(null);
  let loading = $state(false);
  let previewError = $state<string | null>(null);
  let exporting = $state(false);
  let exportFormat = $state<'png' | 'jpeg'>('png');
  let exportResolution = $state(300);

  async function generatePreview() {
    if (!config) {
      toast.warning('Salve a configuração de capa antes de visualizar.');
      return;
    }
    loading = true;
    previewError = null;
    try {
      const b64 = await ipcGenerateCoverPdf(projectId);
      previewBase64 = b64;
    } catch (e) {
      previewError = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleExport() {
    if (!config) {
      toast.warning('Salve a configuração de capa antes de exportar.');
      return;
    }
    exporting = true;
    try {
      const path = await ipcExportCoverImage(projectId, exportFormat, exportResolution);
      toast.success(`Capa exportada: ${path}`);
    } catch (e) {
      toast.error(String(e));
    } finally {
      exporting = false;
    }
  }
</script>

<div data-testid="cover-preview" class="preview">

  <!-- Persistent error banner -->
  {#if previewError}
    <div class="preview__error-banner" role="alert">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="15" y1="9" x2="9" y2="15"/>
        <line x1="9" y1="9" x2="15" y2="15"/>
      </svg>
      <span>{previewError}</span>
      <button class="preview__error-retry" onclick={generatePreview}>Tentar novamente</button>
    </div>
  {/if}

  <!-- Preview canvas area -->
  <div class="preview__canvas" aria-label="Visualização da capa">
    {#if loading}
      <div class="preview__loading" role="status">
        <span class="preview__spinner" aria-hidden="true"></span>
        <span class="preview__loading-text">Gerando preview…</span>
      </div>
    {:else if previewBase64}
      <!-- 3D perspective mockup -->
      <div class="preview__3d" aria-label="Mockup 3D da capa">
        <div class="preview__book">
          <!-- Front cover -->
          <div class="preview__face preview__face--front">
            <img
              src="data:image/png;base64,{previewBase64}"
              alt="Preview da capa"
              class="preview__img"
            />
          </div>
          <!-- Spine -->
          <div
            class="preview__face preview__face--spine"
            style="background: {config?.primaryColor ?? '#991B1B'};"
            aria-hidden="true"
          ></div>
        </div>
        <!-- Shadow -->
        <div class="preview__shadow" aria-hidden="true"></div>
      </div>

      <button
        data-testid="btn-regenerate-preview"
        class="preview__regen-btn"
        onclick={generatePreview}
        aria-label="Regenerar preview"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <polyline points="1 4 1 10 7 10"/>
          <path d="M3.51 15a9 9 0 1 0 .49-3.87"/>
        </svg>
        Regenerar
      </button>
    {:else}
      <!-- Empty state -->
      <div class="preview__empty">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="preview__empty-icon" aria-hidden="true">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <line x1="9" y1="3" x2="9" y2="21"/>
        </svg>
        <p class="preview__empty-text">Clique em "Visualizar" para gerar o preview da capa</p>
        <button
          data-testid="btn-generate-preview"
          class="preview__gen-btn"
          onclick={generatePreview}
          disabled={!config}
        >
          Gerar Preview
        </button>
      </div>
    {/if}
  </div>

  <!-- Export panel -->
  <div class="preview__export" data-testid="cover-export-panel">
    <h3 class="preview__export-title">Exportar</h3>

    <div class="preview__export-fields">
      <div class="preview__field">
        <label class="preview__label" for="select-format">Formato</label>
        <select id="select-format" data-testid="select-export-format" class="preview__select" bind:value={exportFormat}>
          <option value="png">PNG</option>
          <option value="jpeg">JPEG</option>
        </select>
      </div>
      <div class="preview__field">
        <label class="preview__label" for="select-resolution">Resolução (DPI)</label>
        <select id="select-resolution" data-testid="select-export-resolution" class="preview__select" bind:value={exportResolution}>
          <option value={150}>150 DPI (web)</option>
          <option value={300}>300 DPI (impressão)</option>
          <option value={600}>600 DPI (alta qualidade)</option>
        </select>
      </div>
    </div>

    <button
      data-testid="btn-export-cover"
      class="preview__export-btn"
      onclick={handleExport}
      disabled={exporting || !config}
      aria-busy={exporting}
    >
      {#if exporting}
        <span class="spinner-sm" aria-hidden="true"></span>
        Exportando…
      {:else}
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="7 10 12 15 17 10"/>
          <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
        Exportar capa
      {/if}
    </button>

    {#if !config}
      <p class="preview__hint">Salve a configuração para exportar.</p>
    {/if}
  </div>
</div>

<style>
  .preview {
    display: flex;
    flex-direction: column;
    height: 100%;
    gap: var(--space-4);
    padding: var(--space-4);
  }

  /* Error banner */
  .preview__error-banner {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--color-error-subtle, #fef2f2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-md);
    color: var(--color-error, #ef4444);
    font-size: var(--text-sm);
  }

  .preview__error-banner span { flex: 1; }

  .preview__error-retry {
    padding: var(--space-1) var(--space-2);
    border: 1px solid var(--color-error, #ef4444);
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--color-error, #ef4444);
    font-size: var(--text-xs);
    cursor: pointer;
    white-space: nowrap;
  }

  .preview__error-retry:hover {
    background: var(--color-error, #ef4444);
    color: #fff;
  }

  /* Canvas */
  .preview__canvas {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-secondary);
    border-radius: var(--radius-lg);
    position: relative;
    min-height: 300px;
  }

  /* 3D mockup */
  .preview__3d {
    position: relative;
    perspective: 800px;
    display: flex;
    align-items: flex-end;
    justify-content: center;
  }

  .preview__book {
    position: relative;
    transform: rotateY(-20deg) rotateX(5deg);
    transform-style: preserve-3d;
    transition: transform 0.4s ease;
  }

  .preview__book:hover {
    transform: rotateY(-10deg) rotateX(3deg);
  }

  .preview__face {
    display: block;
  }

  .preview__face--front {
    width: 160px;
    height: 240px;
    box-shadow: 4px 4px 20px rgba(0,0,0,0.4);
  }

  .preview__face--spine {
    position: absolute;
    top: 0;
    left: -20px;
    width: 20px;
    height: 240px;
    transform: rotateY(90deg) translateX(-10px);
    transform-origin: right;
    opacity: 0.85;
  }

  .preview__img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .preview__shadow {
    position: absolute;
    bottom: -12px;
    left: 50%;
    transform: translateX(-50%) rotateX(90deg);
    width: 140px;
    height: 20px;
    background: radial-gradient(ellipse, rgba(0,0,0,0.3) 0%, transparent 70%);
    filter: blur(6px);
  }

  .preview__regen-btn {
    position: absolute;
    bottom: var(--space-3);
    right: var(--space-3);
    display: flex;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-1) var(--space-2);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: all var(--duration-fast) ease;
  }

  .preview__regen-btn:hover {
    color: var(--color-text);
    border-color: var(--color-text);
  }

  /* Loading */
  .preview__loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
  }

  .preview__spinner {
    width: 36px;
    height: 36px;
    border: 3px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .preview__loading-text {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  /* Empty */
  .preview__empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
    text-align: center;
    padding: var(--space-6);
  }

  .preview__empty-icon {
    color: var(--color-text-muted);
    opacity: 0.5;
  }

  .preview__empty-text {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
    max-width: 220px;
    margin: 0;
  }

  .preview__gen-btn {
    padding: var(--space-2) var(--space-4);
    background: var(--color-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: 500;
    cursor: pointer;
    transition: opacity var(--duration-fast) ease;
  }

  .preview__gen-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .preview__gen-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Export panel */
  .preview__export {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .preview__export-title {
    font-size: var(--text-sm);
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
  }

  .preview__export-fields {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-2);
  }

  .preview__field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .preview__label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .preview__select {
    padding: var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-bg);
    font-size: var(--text-sm);
    color: var(--color-text);
  }

  .preview__export-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    background: var(--color-primary);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: 500;
    cursor: pointer;
    transition: opacity var(--duration-fast) ease;
  }

  .preview__export-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .preview__export-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .preview__hint {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    margin: 0;
    text-align: center;
  }

  .spinner-sm {
    width: 12px;
    height: 12px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
