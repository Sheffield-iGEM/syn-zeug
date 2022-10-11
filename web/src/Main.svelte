<script lang="ts">
  import { Seq } from "biobox";
  import { writable } from "svelte/store";
  import Grid from "./assets/Grid.svg";
  import Input from "./Main/Input.svelte";
  import Output from "./Main/Output.svelte";
  import Pipeline from "./Main/Pipeline.svelte";

  let input = "";
  let pipeline = writable((s) => s.to_string());
  let output = "";
  let error = false;

  let swap = () => (input = output);
  let copy = () => navigator.clipboard.writeText(output);

  let seq = new Seq("");
  $: try {
    // FIXME: Move whitespace handling to the Rust side of things!
    seq = new Seq(input.replace(/\s/g, ""));
    error = false;
  } catch (e) {
    output = e;
    error = true;
  }

  $: try {
    if (!error) {
      output = $pipeline(seq);
    }
  } catch (e) {
    output = e;
  }
</script>

<main
  class="flex portrait:flex-col landscape:flex-row justify-evenly items-center bg-surface-200 dark:bg-surface-800 bg-repeat"
  style="background-image: url({Grid})"
>
  <Input bind:value={input} bind:seq />
  <Pipeline bind:value={pipeline} />
  <Output bind:value={output} on:swap={swap} on:copy={copy} />
</main>
