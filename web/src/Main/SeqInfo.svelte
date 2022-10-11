<script lang="ts">
  import { Seq } from "biobox";
  export let value;

  let unit = "bp";
  $: unit = value.kind() == "Protein" ? "aa" : "bp";

  let gc = 0;
  $: try {
    gc = Math.round(value.gc_content() * 100);
    gc = isNaN(gc) ? 0 : gc;
  } catch {
    gc = 0;
  }
</script>

<div class="flex gap-3 font-plex-mono">
  <span><strong>Type: </strong>{value.kind()} ({value.alphabet()})</span>
  <span><strong>Length: </strong>{value.len()}{unit}</span>
  {#if value.kind() != "Protein"}
    <span><strong>GC: </strong>{gc}%</span>
  {/if}
</div>
