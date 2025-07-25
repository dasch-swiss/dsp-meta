<script lang="ts">
  import type { Metadata } from '../interfaces';
  import { onDestroy, onMount, tick } from 'svelte';
  import {
    handleSnackbar,
    isTestEnvironment,
    previousRoute,
    projectMetadata,
  } from '../store';
  import ProjectWidget from './ProjectWidget.svelte';
  // import DownloadWidget from "./DownloadWidget.svelte";  // LATER: bring back with download widget
  import Tab from './Tab.svelte';
  import { fade } from 'svelte/transition';
  import Snackbar from '../Snackbar.svelte';
  import { getText } from '../functions';
  import Loading from '../Loading.svelte';

  const mobileResolution = window.innerWidth < 992;
  const descriptionLanguages = new Map<string, string>([
    // contains languages that are presented in provided descriptions, update if necessary
    ['ar', 'Arabic'],
    ['de', 'German'],
    ['en', 'English'],
    ['fr', 'French'],
    ['it', 'Italian'],
  ]);

  let isDescriptionExpanded: boolean;
  let descriptionLinesNumber: number;
  let arePublicationsExpanded: boolean;
  let displayedDescriptionsLanguage: string = 'en';
  let availableDescriptionsIso: string[] = [];

  const getIso = (language: string): string => {
    const lang =
      availableDescriptionsIso.length === 1
        ? availableDescriptionsIso[0]
        : language;
    const found = [...descriptionLanguages].find(([key, val]) => val === lang);
    return found ? found[0] : lang;
  };

  onMount(async () => {
    // wait with component creation for the data to be fetched
    await getProjectMetadata();
    availableDescriptionsIso = Object.keys(
      $projectMetadata?.project.description || {},
    );
    // initialize iso language to load => use first available language if only one, otherwise prefer English or fall back to first
    if (availableDescriptionsIso.length > 0) {
      displayedDescriptionsLanguage =
        availableDescriptionsIso.length === 1
          ? availableDescriptionsIso[0]
          : availableDescriptionsIso.includes('en')
            ? 'en'
            : availableDescriptionsIso[0];
    }
  });

  onDestroy(() => {
    // clear cached project
    projectMetadata.set(undefined);
  });

  const getProjectMetadata = async () => {
    const projectID = window.location.pathname.split('/')[2];

    const res = await fetch(`/api/v1/projects/${projectID}`);
    const metadata: Metadata = await res.json();

    projectMetadata.set(metadata);

    document.title = metadata.project.name;

    await tick();
    getDivHeight();
  };

  const toggleDescriptionExpand = () => {
    isDescriptionExpanded = !isDescriptionExpanded;
    !isDescriptionExpanded ? window.scrollTo(0, 0) : null;
  };

  const togglePublicationExpand = () => {
    arePublicationsExpanded = !arePublicationsExpanded;
    !arePublicationsExpanded ? window.scrollTo(0, 300) : null;
  };

  const getDivHeight = () => {
    setTimeout(() => {
      let lineHeight: number;
      let divHeight: number;
      try {
        const el = document.getElementById('description');
        divHeight = el.scrollHeight;
        lineHeight = parseInt(
          window.getComputedStyle(el).getPropertyValue('line-height'),
        );
      } catch (error) {
        lineHeight = 20;
        divHeight = 19;
      }
      descriptionLinesNumber = divHeight / lineHeight;
      isDescriptionExpanded = descriptionLinesNumber > 6 ? false : true;
    }, 100);
  };
</script>

{#if $handleSnackbar.isSnackbar}
  <div>
    <svelte:component this={Snackbar} />
  </div>
{/if}

{#if $projectMetadata}
  <div class="container" in:fade={{ duration: 250 }}>
    {#if mobileResolution}
      <button
        on:click={() => history.back()}
        class="goback-button"
        title="go back to the projects list"
        disabled={!$previousRoute && window.history.length <= 2}>
        <svg
          class="icon"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10 19l-7-7m0 0l7-7m-7 7h18" />
        </svg>
        <span class="button-label">Go Back</span>
      </button>
    {/if}

    <!-- Project name and alternative names -->
    <div class="row" style="flex-wrap: wrap">
      {#if $projectMetadata?.project.name}
        <h1 class="title top-heading">
          {$projectMetadata?.project.name}
        </h1>
      {:else if $isTestEnvironment}
        <div class="warning top-heading">Project Name missing</div>
      {/if}
      {#if $projectMetadata?.project.alternativeNames}
        <div class="row">
          <h4 class="title new-title">
            Also known as:&nbsp;
            <span style="font-style: italic">
              {$projectMetadata?.project.alternativeNames
                .map((t) => {
                  return getText(t);
                })
                .join(', ')}
            </span>
          </h4>
        </div>
      {/if}
    </div>
    <div class="row">
      <div class="column-left">
        <!-- Description -->
        <div class="property-row">
          {#if $projectMetadata?.project.description && getText($projectMetadata?.project.description)}
            <span class="label new-subtitle" style="display: block;"
              >Description
              <span
                style={availableDescriptionsIso.length <= 1
                  ? 'display: none'
                  : 'display: contents'}>
                in
                {#each Object.keys($projectMetadata?.project.description).map((k) => descriptionLanguages.get(k) || k) as l}
                  <button
                    class="language {availableDescriptionsIso.length > 1 &&
                    displayedDescriptionsLanguage === getIso(l)
                      ? 'active'
                      : ''}"
                    on:click={() =>
                      (displayedDescriptionsLanguage = getIso(l))}>
                    {l}
                  </button>
                {/each}
              </span>
            </span>
            <div
              id="description"
              class="data new-text {isDescriptionExpanded
                ? ''
                : 'description-short'}">
              {$projectMetadata?.project.description[
                displayedDescriptionsLanguage || availableDescriptionsIso[0]
              ]}
            </div>
            {#if descriptionLinesNumber > 6}
              <div on:click={toggleDescriptionExpand} class="expand-button">
                show {isDescriptionExpanded ? 'less' : 'more'}
              </div>
            {/if}
          {:else if $isTestEnvironment}
            <div class="warning" id="description">Description missing</div>
          {/if}
        </div>

        <!-- Publications -->
        {#if $projectMetadata?.project.publications && Array.isArray($projectMetadata?.project.publications)}
          <div class="property-row">
            <span class="label new-subtitle">Publications</span>
            {#each $projectMetadata?.project.publications as p, i}
              {#if i > 1}
                <span
                  class={arePublicationsExpanded ? 'data new-text' : 'hidden'}
                  >{p.text}
                  {#if p.url}
                    {#each p.url as url, n}
                      <a
                        href={url.url}
                        class="publication-link {arePublicationsExpanded
                          ? 'data'
                          : 'hidden'}"
                        target="_">
                        {url.text}
                        {p.url.length > 0 && n < p.url.length - 1 ? ',' : ''}
                      </a>
                    {/each}
                  {/if}
                </span>
              {:else}
                <span class="data new-text"
                  >{p.text}
                  {#if p.url}
                    {#each p.url as url, n}
                      <a
                        href={url.url}
                        class="publication-link {arePublicationsExpanded
                          ? 'data'
                          : 'hidden'}"
                        target="_">
                        {url.text}
                        {p.url.length > 0 && n < p.url.length - 1 ? ',' : ''}
                      </a>
                    {/each}
                  {/if}
                </span>
              {/if}
            {/each}
          </div>
          {#if $projectMetadata?.project.publications.length > 2}
            <div on:click={togglePublicationExpand} class="expand-button">
              show {arePublicationsExpanded ? 'less' : 'more'}
            </div>
          {/if}
        {/if}

        <div class="tabs">
          <Tab datasets={$projectMetadata?.datasets} />
        </div>

        {#if !mobileResolution}
          <button
            on:click={() =>
              window.scrollTo({ top: 0, left: 0, behavior: 'smooth' })}
            class="gototop-button"
            title="Get back to the top">
            <svg
              class="icon"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 10l7-7m0 0l7 7m-7-7v18" />
            </svg>
          </button>
        {/if}
      </div>
      <div class="column-right">
        {#if !mobileResolution}
          <button
            on:click={() => history.back()}
            class="goback-button"
            title="go back to the projects list"
            disabled={!$previousRoute && window.history.length <= 2}>
            <svg
              class="icon"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
            <span class="button-label">Go Back</span>
          </button>
        {/if}

        <div class="widget">
          <ProjectWidget />
        </div>

        <!-- LATER: temp disabled download widget -->
        <!-- <div class=widget>
            <DownloadWidget />
          </div> -->

        {#if mobileResolution}
          <button
            on:click={() => {
              window.scrollTo({ top: 0, left: 0, behavior: 'smooth' });
            }}
            class="gototop-button m-hidden"
            title="Get back to the top">
            <svg
              class="icon"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 10l7-7m0 0l7 7m-7-7v18" />
            </svg>
          </button>
        {/if}
      </div>
    </div>
  </div>
{:else}
  <Loading />
{/if}

<style>
  button {
    color: var(--lead-colour);
    box-shadow: var(--shadow-1);
    border: 1px solid #cdcdcd;
    border-radius: 0.25rem;
  }
  button.goback-button {
    margin-top: 10px;
    width: 100%;
    font-size: 1rem;
    text-align: left;
    margin-bottom: 6px;
    padding: 10px 10px 8px;
    background-color: #fff;
    border: none;
    box-shadow: none;
    color: var(--dasch-text);
  }
  .button-label {
    position: relative;
    bottom: 10px;
  }
  button.gototop-button {
    display: inline-block;
    vertical-align: middle;
    background-color: #fff;
    padding: 10px;
    width: 3.5rem;
    height: 3.5rem;
    border: 1px solid var(--lead-colour);
    box-shadow: none;
    color: var(--lead-colour);
  }
  button.gototop-button:hover,
  button.goback-button:hover {
    background-color: #fff;
  }
  .container {
    padding: 0 10px;
    display: block;
    max-width: 1200px;
  }
  .title {
    display: flex;
    flex-direction: row;
    flex-basis: 100%;
    margin-bottom: 0;
    padding: 0 20px;
  }
  .column-left,
  .column-right {
    display: flex;
    flex-direction: column;
    flex-basis: 100%;
    flex: 2;
    padding: 0 5px;
    height: fit-content;
  }
  .column-right {
    flex: 1;
  }
  .description-short {
    display: -webkit-box;
    -webkit-line-clamp: 6;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .publication-link {
    display: contents;
    color: var(--lead-colour);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  button.language {
    width: 80px;
    margin: 0 5px;
    border: 1px solid var(--lead-colour);
    background-color: #fff;
    display: inline-block;
    vertical-align: middle;
    border-radius: 0.25rem;
    padding: 5px 10px;
    color: var(--lead-colour);
    box-shadow: var(--shadow-1);
  }
  button.language.active {
    color: white;
    background-color: var(--dasch-secondary);
    border-color: #dee2e6 #dee2e6 #fff;
  }

  @supports (-moz-appearance: none) {
    button.gototop-button {
      margin-bottom: 40px;
    }
  }
  @media screen and (min-width: 992px) {
    .container {
      padding: 0 40px;
    }
    .column-left,
    .column-right {
      padding: 20px;
    }
    .column-left {
      min-width: 52vw;
    }
    .column-right {
      min-width: 30vw;
    }
    .row {
      flex-direction: row;
    }
  }
  @media screen and (min-width: 1200px) {
    .column-left {
      min-width: 688px;
    }
    .column-right {
      min-width: 352px;
    }
  }
</style>
