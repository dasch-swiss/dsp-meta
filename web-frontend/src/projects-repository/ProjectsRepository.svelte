<script lang='ts'>
  import Tile from './Tile.svelte';
  import Category from './Category.svelte';
  import { onMount } from 'svelte';
  import Pagination from './Pagination.svelte';
  import { getProjectsMetadata, handleSnackbar, pagedResults } from '../store';
  import { fade } from 'svelte/transition';
  import Snackbar from '../Snackbar.svelte';

  let message = 'Loading...';

  $: document.title = 'DaSCH Metadata Browser';

  setTimeout(() => {
    const noData = 'No data retrived. Please check the connection and retry.';
    const noProject = 'No projects found.'
      message = $pagedResults && $pagedResults.length ? noData : noProject;
    }, 3000);
  
  onMount(async () => {
    // TODO: add preventing go back button to get back out of domain

    // get searchUri and 
    const searchUri = window.location.search;
    const params = new URLSearchParams(searchUri);
    const page = Number(params.get('_page'));
    const query = params.get('q');
    console.log(searchUri, query, page, params.get('_limit'),);

    // load projects
    if (!$pagedResults && !searchUri) {
      // first page on main page arrival
      await getProjectsMetadata(1);
    } else {
      // preserved on rehresh or manually entered query
      await getProjectsMetadata(page, query);
    }
  });
  
</script>

<nav>
  <div class="category-container hidden m-inline-block">
    <Category />
  </div>
</nav>

{#if $handleSnackbar.isSnackbar}
  <div>
    <svelte:component this={Snackbar} />
  </div>
{/if}

<main in:fade="{{duration: 200}}">
  <div class=tile-container>
    {#if $pagedResults && $pagedResults.length}
      {#each $pagedResults as project}
        <!-- LATER: remove actual metadata content from BE response on /projects endpoint, and see what that would break -->
        <Tile metadata={project}/>
      {/each}
    {:else}
      <p>{message}</p>
    {/if}
  </div>
  {#if $pagedResults && $pagedResults.length && $pagedResults.length > 1}
    <Pagination />
  {/if}
</main>

<style>
* {
  box-sizing: border-box;
}

nav, main {
  width: fit-content;
  min-height: auto;
}
nav {
  padding: 0;
}
.category-container {
  max-width: 210px;
}
main {
  padding: 0, 0, 10px, 10px;
}
.tile-container {
  padding: 10px 5px;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  max-width: 1200px;
}
@media screen and (min-width: 992px) {

  nav {
    padding: 10px 0;
  }
  .tile-container {
    min-width: 742px;
  }
}
@media screen and (min-width: 1200px) {
  .tile-container {
    min-width: 940px;
  }
}
@media screen and (min-width: 768px) and (max-width: 1023px) { }
@media screen and (min-width: 1024px) and (max-width: 1365px) { }
@media screen and (min-width: 1366px) {}
</style>
