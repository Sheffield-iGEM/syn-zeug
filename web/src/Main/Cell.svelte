<script lang="ts">
  export let title: String;
</script>

<div
  class="card flex flex-col !ring-0 portrait:w-4/5 landscape:w-1/4 portrait:h-1/4 landscape:h-4/6 !bg-surface-300 dark:!bg-surface-700"
>
  <header class="card-header">
    <p class="font-plex-mono font-bold uppercase text-xl -mt-[1.9rem] ml-[2%]">
      <span class="bg-surface-300 dark:!bg-surface-700 px-2 rounded-lg"
        >{title}</span
      >
    </p>
  </header>
  <div class="card-body grow overflow-y-auto">
    <slot />
  </div>
  <footer class="card-footer">
    <slot name="footer" />
  </footer>
</div>
