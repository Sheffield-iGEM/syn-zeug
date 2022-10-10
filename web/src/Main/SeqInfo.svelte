<script lang="ts">
  export let value;

  let gc = 0;
  $: try {
    gc = Math.round(value.gc_content() * 100);
    gc = isNaN(gc) ? 0 : gc;
  } catch {
    gc = 0;
  }
</script>

<div class="flex gap-3 font-plex-mono">
  <span
    ><strong>Type:</strong>
    <span>{value.kind()} ({value.alphabet()})</span></span
  >
  <span><strong>Length:</strong> {value.len()}bp </span>
  <span><strong>GC:</strong> {gc}%</span>
</div>
