<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.cover')}</title>
</svelte:head>

<!-- Rock-7: Editor de capa (módulo-6) -->
<!-- TODO: Implementar backend — editor visual de capa com mockup 3D -->
{#if !project}
  <EmptyState
    icon="layout"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <EmptyState
    icon="layout"
    title={t('nav.cover')}
    description={t('common.comingSoon')}
    size="md"
  />
{/if}
