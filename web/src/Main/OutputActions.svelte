<script>
  import Fa from "svelte-fa";
  import { faReplyAll, faCopy } from "@fortawesome/free-solid-svg-icons";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();
</script>

<div class="flex justify-between font-plex-mono">
  <button on:click={() => dispatch("copy")}>
    <Fa icon={faCopy} />
  </button>
  <button on:click={() => dispatch("swap")}>
    <Fa icon={faReplyAll} />
  </button>
</div>
