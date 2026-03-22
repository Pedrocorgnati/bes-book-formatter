<script lang="ts">
  import { t } from '$lib/i18n/engine';

  interface TypoIssue {
    issueType: string;     // "orphan" | "widow"
    pageNumber: number;
    lineText: string;
    lineYPercent: number;
    severity: string;
  }

  interface Props {
    issues: TypoIssue[];
    pageNumber: number;
    pageWidthPx: number;
    pageHeightPx: number;
    visible?: boolean;
  }

  let {
    issues,
    pageNumber,
    pageWidthPx,
    pageHeightPx,
    visible = true,
  }: Props = $props();

  const pageIssues = $derived(issues.filter((i) => i.pageNumber === pageNumber));
</script>

{#if visible && pageIssues.length > 0}
  <div
    class="orphan-widow-layer"
    style="width:{pageWidthPx}px; height:{pageHeightPx}px;"
    aria-hidden="true"
  >
    {#each pageIssues as issue}
      <div
        class="typo-highlight typo-{issue.issueType}"
        style="top:{issue.lineYPercent}%; width:100%; height:1.5em;"
        title="{issue.issueType === 'orphan' ? t('preview.orphan') : t('preview.widow')}: {issue.lineText}"
      >
        <span class="typo-badge">
          ⚠ {issue.issueType === 'orphan' ? t('preview.orphan') : t('preview.widow')}
        </span>
      </div>
    {/each}
  </div>
{/if}

<style>
  .orphan-widow-layer {
    position: absolute;
    top: 0;
    left: 0;
    pointer-events: none;
    z-index: 9;
  }

  .typo-highlight {
    position: absolute;
    left: 0;
    pointer-events: none;
  }

  .typo-orphan {
    background: rgba(239, 68, 68, 0.15);
    border-left: 3px solid var(--color-error, #ef4444);
  }

  .typo-widow {
    background: rgba(245, 158, 11, 0.15);
    border-left: 3px solid var(--color-warning, #f59e0b);
  }

  .typo-badge {
    font-size: 10px;
    color: var(--color-text);
    padding: 1px 4px;
    background: rgba(255, 255, 255, 0.7);
    border-radius: 2px;
    display: inline-block;
    margin-left: 4px;
    margin-top: 1px;
  }
</style>
