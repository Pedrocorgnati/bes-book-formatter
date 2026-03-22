<script lang="ts">
  import { goto } from '$app/navigation';
  import { t } from '$lib/i18n/engine';
  import { projectsStore } from '$lib/stores/projectStore';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';

  const project = $derived($projectsStore.current);
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.preview')}</title>
</svelte:head>

<!-- Rock-6: Preview ao vivo do livro (módulo-5) -->
<!-- TODO: Implementar backend — renderização Typst em tempo real + modo distração -->
{#if !project}
  <EmptyState
    icon="eye"
    title={t('emptyState.openProjectFirst')}
    ctaLabel={t('nav.backToDashboard')}
    onCta={() => goto('/')}
  />
{:else}
  <EmptyState
    icon="eye"
    title={t('nav.preview')}
    description={t('common.comingSoon')}
    size="md"
  />
{/if}
