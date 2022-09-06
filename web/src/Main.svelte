<script lang="ts">
  import { Seq } from "biobox";
  import Grid from "./assets/Grid.svg";
  import Cell from "./Main/Cell.svelte";
  import Input from "./Main/Input.svelte";
  import Output from "./Main/Output.svelte";

  let input = "";
  let output = "";
  let seq = new Seq("");

  $: seq = new Seq(input);
  $: output = seq.reverse_complement().to_string();
</script>

<main
  class="flex portrait:flex-col landscape:flex-row justify-evenly items-center bg-surface-800 bg-repeat"
  style="background-image: url({Grid})"
>
  <Input bind:value={input} />
  <Cell title="Pipeline" />
  <!-- This should flash it's color when a new output is computed! -->
  <Output bind:value={output} />
</main>
