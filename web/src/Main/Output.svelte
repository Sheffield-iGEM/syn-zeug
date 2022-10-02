<script lang="ts">
  import Cell from "./Cell.svelte";

  export let value: String;
</script>

<Cell title="Output">
  <div class="w-full h-full focus-within:ring-2 ring-warning-500 rounded p-2">
    <textarea
      class="w-full h-full font-plex-mono focus:outline-none resize-none caret-warning-500 !bg-transparent !shadow-none"
      spellcheck="false"
      bind:value
    />
  </div>
</Cell>
