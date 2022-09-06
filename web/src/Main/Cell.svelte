<script lang="ts">
  import { Card } from "@brainandbones/skeleton";

  export let title: String;
</script>

<Card
  class="shadow-lg portrait:w-4/5 landscape:w-1/4 portrait:h-1/4 landscape:h-4/6"
  background="bg-surface-700"
  color="text-surface-200"
>
  <svelte:fragment slot="header">
    <p class="font-plex-mono font-bold uppercase text-xl -mt-[1.9rem] ml-[2%]">
      <span class="bg-surface-700 px-2 rounded-lg">{title}</span>
    </p>
  </svelte:fragment>
  <slot />
</Card>
