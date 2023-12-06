<script lang='ts'>
  import { projectMetadata, handleSnackbar } from "../store";
  import { getText, findObjectByID, findGrantByID, findOrganizationByID, copyPermalink, copyHowToCite } from "../functions";

  let isTestEnvironment: boolean = window.location.hostname === 'localhost' || window.location.hostname.startsWith('meta.test')

  const truncateString = (s: string) => {
    // TODO: can this be improved? 1. dynamic langth depending on space; 2. show full text on hover
    if (s.length > 35) {
      return `${s.slice(0, 35)}...`;
    } else return s;
  };

  const getARK = () => {
    const shortcode = $projectMetadata?.project.shortcode
    return `https://ark.dasch.swiss/ark:/72163/1/${shortcode}`
  };
</script>


{#if $projectMetadata}
  <div class="widget">
    <!-- URLs -->
    {#if $projectMetadata?.project.url}
      <div class=label>Project Data</div>
      <a class="data" href={$projectMetadata?.project.url.url} target=_>
        {truncateString($projectMetadata?.project.url.text)}
        <img class=chevron src="assets/icon/Chevron_right.svg" alt="chevron right indicating a link" />
      </a>
    {:else if isTestEnvironment}
      <div class=label>Project Data</div>
      <div class=warning>URL missing</div>
    {/if}
    <!-- Secondary URL -->
    {#if $projectMetadata?.project.secondaryURL}
      <a class="data" href={$projectMetadata?.project.secondaryURL.url} target=_>
        {truncateString($projectMetadata?.project.secondaryURL.text)}
        <img class=chevron src="assets/icon/Chevron_right.svg" alt="chevron right indicating a link" />
      </a>
    {/if}
  </div>

  <div class="widget">
    <!-- Permalink -->
    <div class=label>
      <span style="display:inline">
        Permalink
          <button on:click={copyPermalink} title="copy permalink to the clipboard">
            <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
          </button>
      </span>
    </div>
    <a id="permalink" href={getARK()} target=_>{getARK()}</a>

    <!-- Shortcode -->
    {#if $projectMetadata?.project.shortcode}
      <div class=label>DSP Internal Shortcode</div>
      <div class=data>{$projectMetadata?.project.shortcode}</div>
    {:else if isTestEnvironment}
      <div class=label>DSP Internal Shortcode</div>
      <div class=warning>Shortcode missing</div>
    {/if}

    <!-- DMP -->
    <div class=label>Data Management Plan</div>
    <div class=data>{$projectMetadata?.project.dataManagementPlan ? "available" : "unavailable"}</div>

    <!-- How to Cite -->
    {#if $projectMetadata?.project.howToCite}
      <div class=label>
        <span style="display:inline">
          How To Cite
          {#if $projectMetadata?.project.howToCite}
            <button on:click={copyHowToCite} title="copy citation to the clipboard">
              <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
            </button>
          {/if}
        </span>
      </div>
      <div class=data>{$projectMetadata?.project.howToCite}</div>
    {:else if isTestEnvironment}
      <div class=label>How To Cite</div>
      <div class=warning>How-to-cite missing</div>
    {/if}


    <!-- Disciplines -->
    {#if $projectMetadata?.project.disciplines}
      <div class=label>Disciplines</div>
      {#each $projectMetadata?.project.disciplines as d}
        {#if d.__type === "URL"}
          <a class="data" href={d.url} target=_>{truncateString(d.text)}</a>
        {:else}
          {#if getText(d).match(/^\d+ /)}
            <a class="data" href=http://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf target=_>{truncateString(getText(d))}</a>
          {:else}
            <div class=data>{getText(d)}</div>
          {/if}
        {/if}
      {/each}
    {:else if isTestEnvironment}
      <div class=label>Disciplines</div>
      <div class=warning>Disciplines missing</div>
    {/if}

    <!-- Temporal Coverage -->
    {#if $projectMetadata?.project.temporalCoverage}
      <div class=label>Temporal Coverage</div>
      {#each $projectMetadata?.project.temporalCoverage as t}
        {#if t.__type === "URL"}
          <a class="data" href={t.url} target=_>{t.text ? truncateString(t.text) : truncateString(t.url)}</a>
        {:else}
          <div class=data>{getText(t)}</div>
        {/if}
      {/each}
    {:else if isTestEnvironment}
      <div class=label>Temporal Coverage</div>
      <div class=warning>Temporal coverage missing</div>
    {/if}


    <!-- Spatial Coverage -->
    {#if $projectMetadata?.project.spatialCoverage}
      <div class=label>Spatial Coverage</div>
      {#each $projectMetadata?.project.spatialCoverage as s}
        <a class="data" style="text-transform: capitalize" href={s.url} target=_>{truncateString(s.text)}</a>
      {/each}
    {:else if isTestEnvironment}
      <div class=label>Spatial Coverage</div>
      <div class=warning>Spatial coverage missing</div>
    {/if}

    <!-- Start Date -->
    {#if $projectMetadata?.project.startDate}
      <div class=label>Start date</div>
      <div class=data>{$projectMetadata?.project.startDate}</div>
    {:else if isTestEnvironment}
      <div class=label>Start date</div>
      <div class=warning>Start date missing</div>
    {/if}
    
    <!-- End Date -->
    {#if $projectMetadata?.project.endDate}
      <div class=label>End date</div>
      <div class=data>{$projectMetadata?.project.endDate}</div>
    {/if}

    <!-- Funders -->
    {#if $projectMetadata?.project.funders && $projectMetadata?.project.funders.map((o) => {return findObjectByID(o)}).filter(e => e).length>0 }
      <div class=label>Funder</div>
      {#each $projectMetadata?.project.funders.map((o) => {return findObjectByID(o)}) as f}
          {#if f.__type === "Person"}
            {console.log('person',f)}
            <!-- TODO: handle funding person - need to find example -->
            <!-- <div class=data>{findObjectById(f)?.givenName.split(";").join(" ")} {findObjectById(f)?.familyName}</div> -->
          {:else if f.__type === "Organization"}
            <div class=data>{f.name}</div>
          {/if}
      {/each}
    {:else if isTestEnvironment}
      <div class=label>Funder</div>
      <div class=warning>funders missing</div>
    {/if}
    
    <!-- Grants -->
    {#if $projectMetadata?.project.grants}
      <div class=label>Grant</div>
      {#each $projectMetadata?.project.grants.map(id => {return findGrantByID(id)}) as g}
        {#if g?.number && g?.url && g?.name}
          <a class="data" href={g?.url.url} target=_>{truncateString(`${g?.name}: ${g?.number}`)}</a>
        {:else if g?.number && g?.url}
          <a class="data" href={g?.url.url} target=_>{g?.number}</a>
        {:else if g?.number}
          <span class=data>{g?.number}</span>
        {:else}
          {#each [g?.funders[0]].map(o => {return findOrganizationByID(o)}) as f}
            <span class=data>{f.name}</span>
          {/each}
        {/if}
      {/each}
    {/if}

    <!-- Contact Point -->
    {#if $projectMetadata?.project.contactPoint && findObjectByID($projectMetadata?.project.contactPoint)}
      <div class=label>Contact</div>
      {#each [findObjectByID($projectMetadata?.project.contactPoint)] as c}
        {#if c.__type === 'Organization'}
          <div id=contact class=data>{c.name}</div>
          {#if c.email}
            <a class="data email" href="mailto:{c?.email}">{c?.email}</a>
          {/if}
        {:else if c.__type === 'Person'}
          {#if c?.givenNames && c?.familyNames}
            <div id=contact class=data>{c?.givenNames?.join(" ")} {c?.familyNames.join(" ")}</div>
          {/if}
          {#if Array.isArray(c?.affiliation)}
            {#each c?.affiliation as o}
              {#each [findOrganizationByID(o)] as org}
                <span class=data>{org.name}</span>
              {/each}
            {/each}
          {/if}
          {#if c.email}
            <a class="data email" href="mailto:{c?.email}">{c?.email}</a>
          {/if}
        {/if}
      {/each}
    {/if}

    <!-- Keywords -->
    {#if $projectMetadata?.project.keywords}
      <div class=label>Keywords</div>
      <span class="keyword">{$projectMetadata?.project.keywords.map(t => {return getText(t)}).join(", ")}</span>
    {:else if isTestEnvironment}
      <div class=label>Keywords</div>
      <div class=warning>keywords missing</div>
    {/if}
  </div>

  {:else if isTestEnvironment}
    <div class=warning>Project not available</div>
{/if}

<style>
  a {
    display: block;
    color: var(--lead-colour);
  }
  /* a::after {
    display: block;
    content: ' ';
    background-image: url('assets/icon/chevron.svg');
    background-size: 28px 28px;
    height: 28px;
    width: 28px;
  } */
  .keyword {
    padding: 0;
    
  }
  .label, .data {
    margin: 5px 0;
  }
  .label {
    padding: 10px 0 0;
  }
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
  .widget {
    border: 1px solid #cdcdcd;
    border-radius: 3px;
    background-color: var(--dasch-grey-3);
    margin: 15px 0;
    padding: 0 10px 10px;
    box-shadow: var(--shadow-1);
  }
  .chevron {
    padding: 0;
    height: 100%;
    vertical-align: text-bottom;
  }
</style>
