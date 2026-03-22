<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.output')}</title>
</svelte:head>

<!-- Rock-5: Geração de saída (PDF, EPUB) -->
<!-- TODO: Implementar backend — Typst + Ghostscript + EPUBCheck -->
{#if !project}
  <EmptyState
    icon="package"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <EmptyState
    icon="package"
    title={t('nav.output')}
    description={t('common.comingSoon')}
    size="md"
  />
{/if}
