<script lang="ts">
  import { t } from '$lib/i18n/engine';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
</script>

<svelte:head>
  <title>BES Book Formatter — {t('nav.import')}</title>
</svelte:head>

<!-- Rock-2: Importação de Projeto BES -->
<!-- TODO: Implementar backend — invoke('open_file_dialog') + import_project -->
<EmptyState
  icon="folder"
  title={t('import.title')}
  description={t('import.desc')}
  size="lg"
/>
