<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { getText, findOrganizationByID, findObjectByID, copyHowToCite } from "../functions";
  import type { Dataset } from "../interfaces";

  export let dataset: Dataset;

  let isAbstractExpanded: boolean;
  let abstractLinesNumber: number;
  let isTestEnvironment: boolean = window.location.hostname === 'localhost' || window.location.hostname.startsWith('meta.test')

  const toggleExpand = () => {
    isAbstractExpanded = !isAbstractExpanded;
  };

  onMount(() => {
    let lineHeight: number;
    let divHeight: number;
    try {
      const el = document.getElementById("abstract");
      divHeight = el.scrollHeight;
      lineHeight = parseInt(window.getComputedStyle(el).getPropertyValue('line-height'));
    } catch (error) {
      lineHeight = 20;
      divHeight = 19;
    }
    abstractLinesNumber = divHeight / lineHeight;
    isAbstractExpanded = abstractLinesNumber > 6 ? false : true;
  });

  const truncateString = (s: string) => {
    const browserWidth = window.innerWidth;
    if (browserWidth < 992 && s.length > ((browserWidth - 100) / 8)) {
      return `${s.substring(0, (browserWidth - 100) / 8)}...`;
    } else if (browserWidth >= 992 && s.length > (browserWidth / 17)) {
      return `${s.substring(0, (browserWidth / 17))}...`;
    } else return s;
  };
</script>

<div id=dataset in:fade={{duration: 200}}>
  {#if dataset}
    <!-- Alternative titles -->
    {#if dataset?.alternativeTitles}
      <div>
        <span class=label>Alternative Title</span>
        <span class=data>{dataset?.alternativeTitles.map((t => {return getText(t)})).join(', ')}</span>
      </div>
    {/if}

    <div class=grid-wrapper>
      <!-- Access conditions -->
      {#if dataset?.accessConditions}
        <div>
          <span class=label>Access</span>
          <span class=data>{dataset?.accessConditions}</span>
        </div>
      {:else if isTestEnvironment}
        <div>
          <span class=label>Access</span>
          <span class="warning data">access conditions missing</span>
        </div>
      {/if}

      <!-- Status -->
    <!--   {#if dataset?.status}
        <div>
          <span class=label>Status</span>
          <span class=data>{dataset?.status}</span>
        </div>
      {:else if isTestEnvironment}
        <div>
          <span class=label>Status</span>
          <span class="warning data">status missing</span>
        </div>
      {/if} -->

      <!-- Dates -->
      {#if dataset.dateCreated}
        <div>
          <span class=label>Date Created</span>
          <span class=data>{dataset?.dateCreated}</span>
        </div>
      {/if}
      {#if dataset.datePublished}
        <div>
          <span class=label>Date Published</span>
          <span class=data>{dataset?.datePublished}</span>
        </div>
      {/if}
      {#if dataset.dateModified}
        <div>
          <span class=label>Date Modified</span>
          <span class=data>{dataset?.dateModified}</span>
        </div>
      {/if}

      <!-- Type of Data -->
      {#if dataset?.typeOfData}
        <div>
          <span class=label>Type of Data</span>
          <span class=data>{dataset?.typeOfData.join(', ')}</span>
        </div>
      {:else if isTestEnvironment}
        <div>
          <span class=label>Type of Data</span>
          <span class="warning data">type of data missing</span>
        </div>
      {/if}

      <!-- Additional -->
      {#if dataset?.additional}
        <div style="grid-column-start: 1;grid-column-end: 3;">
          <span class=label>Additional documentation</span>
          {#each dataset?.additional as d}
            {#if d.__type === "URL"}
              <a class="data" href={d.url} target=_>{truncateString(d.text)}</a>
            {:else}
              <span class=data>{getText(d)}</span>
            {/if}
          {/each}
        </div>
      {/if}
      
    </div>

    <!-- License -->
    <!-- TODO: check how this looks with multiple licenses -->
    {#if dataset?.licenses}
      <div>
        <span class=label>License</span>
        {#each dataset?.licenses as l}
          <div class=data>
            <a href={l.license.url} target=_>{l.license.text}</a>
            {#if l.details}
              <div>{l.details}</div>
            {/if}
            <div>({l.date})</div>
          </div>
        {/each}
      </div>
    {:else if isTestEnvironment}
      <div>
        <span class=label>License</span>
        <span class="warning data">licenses missing</span>
      </div>
    {/if}

    <!-- Languages -->
    {#if dataset?.languages}
      <div class=grid-wrapper style="grid-template-columns: repeat(1, 1fr)">
        <div>
          <span class=label>Languages</span>
          <span class=data>{dataset?.languages.map(l => {return getText(l)}).join(', ')}</span>
        </div>
      </div>
    {:else if isTestEnvironment}
      <div class=grid-wrapper style="grid-template-columns: repeat(1, 1fr)">
        <div>
          <span class=label>Languages</span>
          <span class="warning data">languages missing</span>
        </div>
      </div>
    {/if}


    <!-- How To Cite -->
    {#if dataset?.howToCite}
      <div class="property-row">
        <span class=label style="display:inline">
          How To Cite
          {#if dataset?.howToCite}
            <button on:click={copyHowToCite} title="copy citation to the clipboard">
              <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
            </button>
          {/if}
        </span>
        <span id=how-to-cite class=data>{dataset?.howToCite}</span>
      </div>
    {:else if isTestEnvironment}
      <div class="property-row">
        <span class=label style="display:inline">How To Cite</span>
        <span class="warning data">how to cite missing</span>
      </div>
    {/if}

    <!-- Abstract -->
    {#if dataset?.abstracts}
      <div>
        <span class=label>Abstract</span>
        <div id=abstract class="data {isAbstractExpanded ? '' : 'abstract-short'}">
          {#each dataset?.abstracts as a}
            {#if a.__type === "URL"}
              <div><a class="data" href={a.url} target=_>{truncateString(a.text)}</a></div>
            {:else}
              <div>{getText(a)}</div>
            {/if}
          {/each}
        </div>
      </div>
      {#if abstractLinesNumber > 6}
        <div on:click={toggleExpand} class=expand-button>show {isAbstractExpanded ? "less" : "more"}</div>
      {/if}
    {:else if isTestEnvironment}
      <div>
        <span class=label>Abstract</span>
        <span class="warning data" id="abstract">abstract missing</span>
      </div>
    {/if}

      <!-- URLs -->
      {#if dataset?.urls}
      <div class=grid-wrapper style="grid-template-columns: repeat(1, 1fr)">
        <div>
          <span class=label>Additional Links</span>
          {#each dataset?.urls as u}
            {#if u.__type === 'URL'}
              <div><a class="data" href={u.url} target=_>{truncateString(u.text)}</a></div>
            {/if}
          {/each}
        </div>
      </div>
    {/if}

    <!-- Attribution -->
    {#if dataset?.attributions}
      <span class=label>Attributions</span>
      <div class=grid-wrapper>
        {#each dataset?.attributions as a}
          <div class="attributions data">
            <div class=role>{a.roles.join(", ")}</div>
            {#each [findObjectByID(a.agent)] as p}
              {#if p.__type === 'Person'}
                {#if p.authorityRefs}
                  <a href={p.authorityRefs[0].url} target=_ class="attribution-name-link">{p.givenNames.join(" ")} {p.familyNames.join(" ")}</a>
                {:else}
                  <div class="attribution-name">{p.givenNames.join(" ")} {p.familyNames.join(" ")}</div>
                {/if}
                {#if p.affiliation}
                  {#each p.affiliation.map(o => {return findOrganizationByID(o)}) as org}
                    <div class="attribution-additional">{org.name}</div>
                  {/each}
                {/if}
                {#if p.jobTitles && p.jobTitles[0]}
                  <div class="attribution-additional">{p.jobTitles[0]}</div>
                {:else if isTestEnvironment}
                  <dif class="warning">Job Title missing</dif>
                {/if}
                {#if p.email}
                  <a class=email href="mailto:{p.email}">{p.email}</a>
                {/if}
              {:else if p.__type === 'Organization'}
                {#if p.url}
                  <a href={p.url.url} target=_ class="attribution-name-link">{p.name}</a>
                {/if}
                {#if p.email}
                  <a class=email href="mailto:{p.email}">{p.email}</a>
                {/if}
              {/if}
            {/each}
          </div>
        {/each}
      </div>
    {:else if isTestEnvironment}
      <span class=label>Attributions</span>
      <div class=grid-wrapper>
        <span class="warning data">attributions missing</span>
      </div>
    {/if}

  {/if}
</div>

<style>
  a {color: var(--lead-colour);}
  button {
    border: none;
    background-color: inherit;
    padding: 0;
    position: relative;
    top: 10px;
    color: var(--lead-colour);
    z-index: 0;
    margin: -1rem 0 0.25rem;
  }
  .icon:hover {
    color: var(--dasch-hover);
  }
  .role {
    font-weight: 700;
  }

  .attribution-name {
    font-family: robotobold;
    color: var(--text-darker);
  }

  .attribution-name-link {
    font-family: robotobold;
    color: var(--lead-colour);
  }

  .attribution-additional {
    font-weight: 400;
  }

  .abstract-short {
    display: -webkit-box;
    -webkit-line-clamp: 6;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .attributions {
    padding: 10px 10px 0 0;
    line-height: 1.5;
  }
  .grid-wrapper {
    display: grid;
    grid-template-columns: repeat(1, 1fr);
  }
  @media screen and (min-width: 576px) {
    .grid-wrapper {
      grid-template-columns: repeat(2, 1fr);
    }
  }
  @media screen and (min-width: 1200px) {
    .grid-wrapper {
      grid-template-columns: repeat(3, 1fr);
    }
  }
</style>
