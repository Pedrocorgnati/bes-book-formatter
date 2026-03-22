<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.illustrations')}</title>
</svelte:head>

<!-- Rock-4: Gerenciamento de ilustrações -->
<!-- TODO: Implementar backend — listagem, validação, processamento de imagens -->
{#if !project}
  <EmptyState
    icon="image"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <EmptyState
    icon="image"
    title={t('nav.illustrations')}
    description={t('common.comingSoon')}
    size="md"
  />
{/if}
