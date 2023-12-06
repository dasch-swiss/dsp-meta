<script lang="ts">
  import DefaultTabComponent from "./DefaultTabComponent.svelte";
  import type { Dataset } from "../interfaces";

  export let datasets = [] as Dataset[];
  export let activeTabValue = 0;

  const handleTabsBrowsing = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<ul>
  {#each datasets as dataset, i}
    <li class={activeTabValue === i ? 'active' : ''}>
      {#if datasets.length > 1 && activeTabValue !== i}
        <span on:click={handleTabsBrowsing(i)} title={dataset.title}>{`${dataset.title.substring(0,12)}...`}</span>
      {:else}
        <span on:click={handleTabsBrowsing(i)}>{dataset.title}</span>
      {/if}
    </li>
  {/each}
</ul>
{#each datasets as dataset, i}
  {#if activeTabValue === i}
    <div class=box>
      <svelte:component this={DefaultTabComponent} dataset={dataset} />
    </div>
  {/if}
{/each}


<style>
  .box {
    margin-bottom: 10px;
    padding: 0 10px;
    border: 1px solid #dee2e6;
    border-radius: 0 0 .5rem .5rem;
    border-top: 0;
    overflow-wrap: break-word;
    box-shadow: var(--shadow-2);
  }
  ul {
    display: flex;
    flex-wrap: wrap;
    padding-left: 0;
    margin-bottom: 0;
    list-style: none;
    border-bottom: 1px solid #dee2e6;
  }
  li {
    margin-bottom: -1px;
  }
  span {
    color: var(--dasch-primary);
    border: 1px solid var(--dasch-primary);
    border-top-left-radius: 0.25rem;
    border-top-right-radius: 0.25rem;
    display: block;
    padding: 0.5rem 1rem;
    cursor: pointer;
  }
  span:hover {
    color: white;
    background-color: var(--dasch-primary);
  }
  li.active > span {
    color: white;
    background-color: var(--dasch-secondary);
    border-color: #dee2e6 #dee2e6 #fff;
  }
</style>
