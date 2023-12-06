<script lang="ts">
  import { Router, Link } from "svelte-routing";
  import Category from "./projects-repository/Category.svelte";
  import { getProjectsMetadata } from "./store";

  let showSearchbar = false;
  let showFilters = false;
  let showMenu = false;
  let enteredString = '';

  const getEnv = () => {
    if (window.location.host.includes('test')) {
      return 'test.';
    } else if (window.location.host.includes('staging')) {
      return 'staging.';
    } else return '';
  }

  function toggleSearchbar() {
    showSearchbar = !showSearchbar;
    showFilters = false;
    showMenu = false;
  }

  function toggleFilters() {
    showFilters = !showFilters;
    showSearchbar = false;
    showMenu = false;
  }

  function toggleMenu() {
    showMenu = !showMenu;
    showFilters = false;
    showSearchbar = false;
  }

  let search = (e: Event) => {
    const q = (e.target as HTMLInputElement).value;
    getProjectsMetadata(1, q);
    enteredString = '';
  }
</script>

<header>
  <div class="header-container">
    <div class="header-left">
      <Router>
        <Link to="/" class="regular-link">
          <img class="logo s-inline-block" src="assets/icon/Fingerprint_Logo.svg" alt="DaSCH logo" />
          <img class="icon-logo s-hidden" src="assets/icon/Fingerprint_Logo.svg" alt="DaSCH logo" />
        </Link>
      </Router>
      <Router>
        <Link to="/" class="regular-link">
          <h1 class="title">DaSCH Metadata Browser</h1>
        </Link>
      </Router>
    </div>
    <div class="header-right">
      <input on:change={search} bind:value={enteredString} class="searchbar-in-header xs-inline-block" type="text" name="searchbar" placeholder="search..." />
      <!-- searchbar button -->
      <button class="xs-hidden" on:click="{toggleSearchbar}">
        <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
      </button>
      <!-- filter button -->
      <button class="filter-button m-hidden"  on:click="{toggleFilters}">
        <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
        </svg>
      </button>
      <!-- menu button -->
      <button class="menu-button" on:click="{toggleMenu}">
        <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
    </div>
  </div>
  <div class="searchbar-container xs-hidden" class:hidden={!showSearchbar}>
    <input on:change={search} bind:value={enteredString} class="searchbar" name="searchbar" placeholder="search..." />
  </div>
  <div class="filter-container m-hidden" class:hidden={!showFilters}>
    <Category />
  </div>
  <div class="menu" class:hidden={!showMenu}>
    <a class="menu-item" href="https://dasch.swiss/" target="_blank">DaSCH Website</a>
    <a class="menu-item" href="{`https://admin.${getEnv()}dasch.swiss/`}" target="_blank">DSP-APP { getEnv() ? `(${getEnv().slice(0, -1)} server)` : ''}</a>
    <a class="menu-item" href="{`https://app.${getEnv()}dasch.swiss/`}" target="_blank">DSP-Tangoh { getEnv() ? `(${getEnv().slice(0, -1)} server)` : ''}</a>
    <a class="menu-item" href="https://docs.dasch.swiss/" target="_blank">Documentation</a>
  </div>
</header>

<style>
  header {
    /* background-color: var(--cl-background); */
    background-color: white;
    position: sticky;
    top: 0px;
    z-index: 1;
    box-shadow: 0px 1px 3px rgba(0, 0, 0, 0.1), 0px 1px 2px rgba(0, 0, 0, 0.06);
  }

  .header-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header-left {
    display: inline-flex;
    align-items: center;
  }

  .header-right {
    padding: 0.5rem 0rem;
    margin-right: 0.25rem;
  }
  .logo {
    height: 2.75rem;
    vertical-align: middle;
    cursor: pointer;
    display: none;
    padding: 20px;
  }
  .icon-logo {
    height: 3rem;
    padding: 5px;
    vertical-align: middle;
    cursor: pointer;
  }
  .title {
    color: var(--dasch-text);
    padding: 0;
    white-space: nowrap;
    font-size: 0.85rem;
    flex: 1;
  }
  .searchbar-in-header {
    display: none;
    width: 35vw;
    max-width: 400px;
    padding: 0.25rem;
    vertical-align: middle;
    line-height: 1.5rem;
    box-sizing: border-box;
  }
  .searchbar {
    display: block;
    line-height: 1.5rem;
    width: 100%;
    box-sizing: border-box;
  }
  .searchbar-container, .filter-container {
    background-color: var(--cl-background-light);
    padding: 12px;
  }
  button {
    display: inline-block;
    vertical-align: middle;
    border-radius: 0.25rem;
    background-color: inherit;
    border: none;
    padding: 0px;
  }
  button:hover {
    color: var(--lead-colour);
  }

  .menu-button {
    color: var(--lead-colour);
  }

  .filter-button {
    color: var(--lead-colour);
  }
  .menu {
    background-color: #fff;
    box-shadow: -1px 0px 3px rgba(0, 0, 0, 0.1), -1px 1px 2px rgba(0, 0, 0, 0.06);
  }
  a.menu-item {
    display: block;
    padding: 1rem 1.5rem;
    width: calc(100% - 3rem);
    font-weight: 500;
    color: var(--dasch-text);
    text-decoration: none;
  }
  a.menu-item:hover {
    background-color: var(--cl-transparent-light);
  }
  /* resetting links animation for header */
  a::before, a::after { background: none;}
  @media screen and (min-width: 768px) {
    .menu {
      width: 20rem;
      height: 90vh;
      position: absolute;
      right: 0px;
      z-index: 0;
    }
    .title {font-size: 1.25rem;}
  }
@media screen and (min-width: 1200px) {
  .header-container {
    max-width: 1200px;
    margin: 0 auto;
  }
}
</style>
