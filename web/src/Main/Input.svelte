<script lang="ts">
  import { Seq } from "biobox";
  import Cell from "./Cell.svelte";
  import SeqInfo from "./SeqInfo.svelte";

  export let value;

  let input = "";
  $: value = new Seq(input);
</script>

<Cell title="Input">
  <div class="w-full h-full focus-within:ring-2 ring-primary-500 rounded p-2">
    <textarea
      class="w-full h-full font-plex-mono focus:outline-none resize-none caret-primary-500 !bg-transparent !shadow-none"
      placeholder="Enter a DNA, RNA, or protein sequence here..."
      spellcheck="false"
      bind:value={input}
    />
  </div>
  <svelte:fragment slot="footer">
    <SeqInfo bind:value />
  </svelte:fragment>
</Cell>
