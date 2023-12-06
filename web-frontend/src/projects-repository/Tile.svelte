<script lang="ts">
  import { Router, Link } from 'svelte-routing';
  import type { ProjectMetadata } from '../interfaces';

  export let metadata: ProjectMetadata;

</script>

<section>
  <div class="status">
    <img src="assets/icon/{metadata.status === 'ongoing' ? 'Badge_Ongoing.svg' : 'Badge_Finished.svg'}" alt="Status {metadata.status}" />
  </div>
  <div class=header>
    <h5>{ metadata.name }</h5>
  </div>
  <div class=content>{ metadata.description }</div>
  <div class=footer>
    <Router>
      <Link to={`/projects/${metadata.id}`} class="read-more regular-link">
        Read more
      </Link>
    </Router>
  </div>
</section>

<style>
  section {
    border: 1px solid #cdcdcd;
    border-radius: 5px;
    background-color: #fff;
    padding: 5px 5vw;
    margin: 5px;
    width: 75vw;
    float: left;
    box-shadow: var(--shadow-2);
  }
  .header{
    font-family: 'robotomedium';
    height: 30px;
    padding-top: 10px;
  }
  .header h5 {
    text-overflow: ellipsis;
    overflow: hidden; 
    white-space: nowrap;
    margin: 0;
  }
  .content {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    font-size: 0.8em;
    height: 45px;
    margin: 10px 0 5px;
  }
  .footer {
    margin: 10px 0;
    color: var(--lead-colour);
    font-size: 0.8em;
    text-align: center;
  }

  .status {
    height: 20px;
    padding: 5px 0px;
    text-align: right;
    width: 100%;
  }

  @media screen and (min-width: 768px) {
    section {
      width: 240px;
      height: 228px;
      padding: 10px 30px;
      margin: 5px;
    }
    .content {
      -webkit-line-clamp: 7;
      height: 107px;
      margin: 5px 0 5px;
    }
  }
  @media screen and (min-width: 1200px) {
    /* section {margin: 10px;} */
  }
</style>
