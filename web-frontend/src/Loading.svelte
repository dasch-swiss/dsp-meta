<script>
  let src = "assets/icon/Fingerprint_Logo.svg";
</script>

<img {src} alt="animated DaSCH icon" />

<style>
  @keyframes blink {
    0% { opacity: 0; }
    50% { opacity: 1; }
    100% { opacity: 0; }
  }

  img {
    width: 150px;
    animation: blink 2s ease-in-out infinite;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>
