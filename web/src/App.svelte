<script lang="ts">
  import { Seq } from "biobox";
  let dna = "";
  let revcomp = "";
  $: try {
    revcomp = new Seq(dna).reverse_complement().to_string();
  } catch (e) {
    revcomp = e;
  }
</script>

<main>
  <h1>Reverse Complement</h1>
  <input bind:value={dna} />
  <p>"{dna}" → "{revcomp}"</p>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
