<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.typography')}</title>
</svelte:head>

<!-- Rock-3: Configurações de tipografia -->
<!-- TODO: Implementar backend — fontes, margens, espaçamento -->
{#if !project}
  <EmptyState
    icon="type"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <EmptyState
    icon="type"
    title={t('nav.typography')}
    description={t('common.comingSoon')}
    size="md"
  />
{/if}
