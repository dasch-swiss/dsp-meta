<script lang="ts">
  import { getCookie, setCookie } from "./cookies-service";

  // prevents display modal locally
  const noLocalhost = window.location.hostname !== 'localhost';
  let modalOn = getCookie('cookiesAccepted') ? false : true;

  const handleModal = () => {
    modalOn = !modalOn;
    setCookie('cookiesAccepted', 'true');
  };
</script>

{#if modalOn && noLocalhost}
  <div id="cookieConsent">
    <div class="modal-wrapper">
      <div class="modal-text">
        DaSCH uses cookies to provide greater user experience. By using our applications you accept
        <a href="https://dasch.swiss/cookie-policy/" target=_blank>cookie policy</a>
        <!-- <a href="https://dasch.swiss/eula/" target=_blank>EULA</a> -->
        and
        <a href="https://dasch.swiss/privacy-policy/" target=_blank>privacy policy</a>.
      </div>
      <div class="modal-buttons">
        <button on:click={() => handleModal()} class="btn-accept-all">OK</button>
      </div>
    </div>
  </div>
{/if}

<style>
a {color: var(--dasch-primary)}
#cookieConsent {
  position: fixed;
  bottom: 0;
  right: 0;
  left: 0;
  z-index: 2;
  width: 100%;
  background-color: #3B4856;
  color: #fff
}
.modal-wrapper {
  display: flex;
  flex-direction: column;
  justify-content: center;
  border: none;
}
.modal-text {
  padding: 1rem 1.5rem;
  line-height: 1.3;
  align-self: center;
}
.modal-text a {
  color: #fff;
  text-decoration: underline;
  text-decoration-color: #fff
}
.modal-buttons {
  padding: 0 1rem 1rem 1rem;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}
button {
  padding: 0.6rem 1rem;
  font-size: 100%;
  border-radius: 0.2rem;
  margin: 0 0.5rem 0.5rem;
}
.btn-accept-all:hover {
  border: 1px solid #fff;
  background-color: #3B4856;
  color: #fff;
}
.btn-accept-all {
  border: 1px solid transparent;
  background-color: #fff;
  color: #3B4856;
}
@media screen and (min-width: 768px) {
  .modal-wrapper {flex-direction: row;}
  .modal-text {
    padding: 1rem 1.5rem;
    width: 80%;
  }
  .modal-buttons {
    margin-top: 0.5rem;
    padding: 1rem;
    flex-direction: row;
    flex-flow: column-reverse;
    width: 20%;
  }
  button {width: auto;}
}
@media screen and (min-width: 1200px) {
  .modal-wrapper {
    margin: 0 auto;
    max-width: 1200px;
  }
}
</style>
