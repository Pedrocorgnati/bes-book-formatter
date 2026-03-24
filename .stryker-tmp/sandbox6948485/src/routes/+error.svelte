<script lang="ts">
  import { page } from '$app/stores';
  import { t } from '$lib/i18n/engine';
  import ErrorPage from '$lib/components/ui/ErrorPage.svelte';

  const status = $derived($page.status);
  const is404 = $derived(status === 404);
</script>

<svelte:head>
  <title>BES Book Formatter — {is404 ? t('errors.notFound') : t('errors.generic')}</title>
</svelte:head>

<ErrorPage
  code={status}
  showReloadButton={!is404}
/>
