<script lang="ts">
  import { AppBar, LightSwitch } from "@brainandbones/skeleton";
  import DarkLogo from "./assets/Dark Logo.svg";
  import LightLogo from "./assets/Light Logo.svg";
</script>

<AppBar class="h-16 z-10 !shadow-2xl !bg-surface-300 dark:!bg-surface-900">
  <svelte:fragment slot="lead">
    <img
      class="h-16 py-2 pl-4 dark:hidden"
      src={LightLogo}
      alt="The SynZeug Logo"
    />
    <img
      class="h-16 py-2 pl-4 hidden dark:block"
      src={DarkLogo}
      alt="The SynZeug Logo"
    />
  </svelte:fragment>
  <!-- Put main header content here! -->
  <svelte:fragment slot="trail">
    <LightSwitch class="mr-4" />
  </svelte:fragment>
</AppBar>
