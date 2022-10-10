<script lang="ts">
  import { Seq } from "biobox";
  import { writable } from "svelte/store";
  import Grid from "./assets/Grid.svg";
  import Input from "./Main/Input.svelte";
  import Output from "./Main/Output.svelte";
  import Pipeline from "./Main/Pipeline.svelte";

  let input = new Seq("");
  let pipeline = writable((s) => s.to_string());
  let output = "";

  $: try {
    output = $pipeline(input);
  } catch (e) {
    output = e;
  }
</script>

<main
  class="flex portrait:flex-col landscape:flex-row justify-evenly items-center bg-surface-200 dark:bg-surface-800 bg-repeat"
  style="background-image: url({Grid})"
>
  <Input bind:value={input} />
  <Pipeline bind:value={pipeline} />
  <Output bind:value={output} />
</main>
