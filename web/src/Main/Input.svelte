<script lang="ts">
  import Cell from "./Cell.svelte";
  import SeqInfo from "./SeqInfo.svelte";

  export let value;
  export let seq;
</script>

<Cell title="Input">
  <div class="w-full h-full focus-within:ring-2 ring-primary-500 rounded p-2">
    <textarea
      class="w-full h-full font-plex-mono focus:outline-none resize-none caret-primary-500 !bg-transparent !shadow-none"
      placeholder="Enter a DNA, RNA, or protein sequence here..."
      spellcheck="false"
      bind:value
    />
  </div>
  <svelte:fragment slot="footer">
    <SeqInfo bind:value={seq} />
  </svelte:fragment>
</Cell>
