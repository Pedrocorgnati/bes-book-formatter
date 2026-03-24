<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n/engine';
  import { toast } from '$lib/stores/toastStore';
  import type { ApiResponse } from '$lib/types';

  interface Annotation {
    id: string;
    projectId: string;
    pageNumber: number;
    xPercent: number;
    yPercent: number;
    annotationType: string;
    color: string;
    content: string;
    createdAt: string;
  }

  interface Props {
    projectId: string;
    pageNumber: number;
    pageWidthPx: number;
    pageHeightPx: number;
    visible?: boolean;
  }

  let {
    projectId,
    pageNumber,
    pageWidthPx,
    pageHeightPx,
    visible = true,
  }: Props = $props();

  let annotations = $state<Annotation[]>([]);
  let showModal = $state(false);
  let clickPos = $state({ x: 0, y: 0 });
  let newContent = $state('');
  let newType = $state<'comment' | 'highlight' | 'flag'>('comment');
  let newColor = $state('#FFC107');
  let saving = $state(false);

  async function loadAnnotations() {
    if (!projectId) return;
    try {
      const res = await invoke<ApiResponse<Annotation[]>>('get_annotations', {
        projectId,
        pageNumber,
      });
      if (res.data) annotations = res.data;
    } catch (e) {
      console.error('[AnnotationLayer] load error:', e);
    }
  }

  // Reload when page changes
  $effect(() => {
    loadAnnotations();
  });

  onMount(loadAnnotations);

  function handleLayerClick(e: MouseEvent) {
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    clickPos = {
      x: ((e.clientX - rect.left) / rect.width) * 100,
      y: ((e.clientY - rect.top) / rect.height) * 100,
    };
    showModal = true;
    newContent = '';
    newType = 'comment';
    newColor = '#FFC107';
  }

  async function saveAnnotation() {
    if (!newContent.trim() && newType === 'comment') return;
    saving = true;
    try {
      const res = await invoke<ApiResponse<Annotation>>('add_annotation', {
        projectId,
        pageNumber,
        xPercent: clickPos.x,
        yPercent: clickPos.y,
        annotationType: newType,
        color: newColor,
        content: newContent.trim(),
      });
      if (res.data) {
        annotations = [...annotations, res.data];
        toast.success(t('common.saved'));
      }
      showModal = false;
    } catch (e) {
      console.error('[AnnotationLayer] save error:', e);
      toast.error(t('common.errorGeneric'));
    } finally {
      saving = false;
    }
  }

  async function deleteAnnotation(id: string) {
    try {
      await invoke('delete_annotation', { annotationId: id });
      annotations = annotations.filter((a) => a.id !== id);
      toast.success(t('common.deleted'));
    } catch (e) {
      console.error('[AnnotationLayer] delete error:', e);
      toast.error(t('common.errorGeneric'));
    }
  }

  function annotationIcon(type: string) {
    if (type === 'comment') return '💬';
    if (type === 'flag') return '🚩';
    return '🖍';
  }
</script>

{#if visible}
  <!-- Annotation overlay layer — canvas-like region; tabindex required for keyboard access -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    class="annotation-layer"
    style="width:{pageWidthPx}px; height:{pageHeightPx}px;"
    onclick={handleLayerClick}
    onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); handleLayerClick(e as unknown as MouseEvent); } }}
    role="application"
    aria-label={t('preview.annotationLayerLabel')}
    tabindex="0"
  >
    {#each annotations as ann (ann.id)}
      <div
        class="annotation-pin annotation-pin--{ann.annotationType}"
        style="left:{ann.xPercent}%; top:{ann.yPercent}%; background:{ann.color};"
        title={ann.content || ann.annotationType}
        aria-label="{t('preview.annotation')}: {ann.content}"
        role="img"
      >
        {annotationIcon(ann.annotationType)}
        <span class="annotation-tooltip">{ann.content}</span>
        <button
          class="annotation-delete"
          onclick={(e) => { e.stopPropagation(); deleteAnnotation(ann.id); }}
          aria-label={t('common.delete')}
          title={t('common.delete')}
        >×</button>
      </div>
    {/each}
  </div>

  <!-- Add annotation modal -->
  {#if showModal}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="annotation-modal-overlay"
      onclick={() => (showModal = false)}
      onkeydown={(e) => { if (e.key === 'Escape') showModal = false; }}
      role="presentation"
    >
      <div
        class="annotation-modal"
        onclick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-label={t('preview.addAnnotation')}
        tabindex="-1"
      >
        <h4>{t('preview.addAnnotation')}</h4>

        <div class="annotation-modal__field">
          <label for="ann-type">{t('preview.annotationType')}</label>
          <select id="ann-type" bind:value={newType}>
            <option value="comment">💬 {t('preview.typeComment')}</option>
            <option value="highlight">🖍 {t('preview.typeHighlight')}</option>
            <option value="flag">🚩 {t('preview.typeFlag')}</option>
          </select>
        </div>

        <div class="annotation-modal__field">
          <label for="ann-content">{t('preview.annotationContent')}</label>
          <textarea
            id="ann-content"
            bind:value={newContent}
            rows="3"
            maxlength="1000"
            placeholder={t('preview.annotationPlaceholder')}
          ></textarea>
        </div>

        <div class="annotation-modal__field annotation-modal__color">
          <label for="ann-color">{t('preview.annotationColor')}</label>
          <input id="ann-color" type="color" bind:value={newColor} />
        </div>

        <div class="annotation-modal__actions">
          <button
            class="btn btn--primary"
            onclick={saveAnnotation}
            disabled={saving}
          >
            {saving ? t('common.saving') : t('common.save')}
          </button>
          <button
            class="btn btn--ghost"
            onclick={() => (showModal = false)}
          >
            {t('common.cancel')}
          </button>
        </div>
      </div>
    </div>
  {/if}
{/if}

<style>
  .annotation-layer {
    position: absolute;
    top: 0;
    left: 0;
    cursor: crosshair;
    z-index: 10;
  }

  .annotation-pin {
    position: absolute;
    width: 24px;
    height: 24px;
    border-radius: 50% 50% 50% 0;
    border: 2px solid rgba(0, 0, 0, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    cursor: pointer;
    transform: translate(-50%, -100%);
    transition: transform var(--duration-fast);
    z-index: 11;
  }

  .annotation-pin:hover {
    transform: translate(-50%, -100%) scale(1.2);
  }

  .annotation-tooltip {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    background: var(--color-text);
    color: var(--color-bg);
    font-size: var(--text-xs);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    white-space: nowrap;
    max-width: 200px;
    overflow: hidden;
    text-overflow: ellipsis;
    pointer-events: none;
    opacity: 0;
    transition: opacity var(--duration-fast);
  }

  .annotation-pin:hover .annotation-tooltip {
    opacity: 1;
  }

  .annotation-delete {
    position: absolute;
    top: -6px;
    right: -6px;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--color-error);
    color: white;
    font-size: 10px;
    line-height: 14px;
    text-align: center;
    cursor: pointer;
    border: none;
    display: none;
    z-index: 12;
  }

  .annotation-pin:hover .annotation-delete {
    display: block;
  }

  /* Modal */
  .annotation-modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .annotation-modal {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-6);
    width: 320px;
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    box-shadow: var(--shadow-lg);
  }

  .annotation-modal h4 {
    margin: 0;
    font-size: var(--text-base);
    font-weight: 600;
    color: var(--color-text);
  }

  .annotation-modal__field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .annotation-modal__field label {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .annotation-modal__field select,
  .annotation-modal__field textarea {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-bg);
    color: var(--color-text);
    font-size: var(--text-sm);
    padding: var(--space-2);
  }

  .annotation-modal__color {
    flex-direction: row;
    align-items: center;
  }

  .annotation-modal__actions {
    display: flex;
    gap: var(--space-2);
    justify-content: flex-end;
  }
</style>
