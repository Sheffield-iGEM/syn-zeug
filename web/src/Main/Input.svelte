<script lang="ts">
  import Cell from "./Cell.svelte";

  export let value: String;
</script>

<Cell title="Input">
    <div class="w-full h-full focus-within:ring-2 ring-primary-500 rounded p-2">
      <textarea
        class="w-full h-full font-plex-mono focus:outline-none resize-none caret-primary-500 !bg-transparent !shadow-none"
        placeholder="Enter a DNA, RNA, or Protein sequence here..."
        spellcheck="false"
        bind:value
      />
    </div>
</Cell>
