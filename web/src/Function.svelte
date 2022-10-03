<script lang="ts">
  import { Card, SlideToggle, Tooltip } from "@brainandbones/skeleton";

  let myValue: boolean = false;
  export let functionName = "";
  let isChecked = false;

  const handleCheckboxChange = (e) => {
    isChecked = !isChecked;
  };
</script>

<Card class="w-full m-2">
  <div class="flex flex-row w-full justify-between">
    <div class="flex flex-col">
      {functionName}
      <Tooltip position="right">
        <svelte:fragment slot="message"
          >This is a basic tooltip.</svelte:fragment
        >
        <svelte:fragment slot="content">
          <input
            type="checkbox"
            class="checkbox mt-2"
            checked={isChecked}
            on:change={handleCheckboxChange}
          />
        </svelte:fragment>
      </Tooltip>
    </div>
    <SlideToggle bind:checked={myValue} />
  </div>
</Card>

<style>
  .checkbox {
    flex-shrink: 0;
    appearance: none;
    -webkit-appearance: none;
    border-radius: var(--_radius);
    width: 1em;
    height: 1em;
    background-color: hsl(var(--muted));
    border-radius: var(--_radius);
    box-shadow: 0 0 0.5em hsl(var(--accent) / 0.1),
      0 0 0 0.05em hsl(var(--accent) / 0.5), 0 0 0 -0.2em hsl(var(--accent));
    transition: box-shadow var(--_tspeed_fast) ease-in-out,
      background-color 80ms ease-in-out;
  }

  .checkbox:focus {
    outline: none;
  }

  .checkbox:is(:focus-visible, :hover) {
    box-shadow: 0 0 0 hsl(var(--bg)), 0 0 0 0.05em hsl(var(--accent)),
      0 0 0 0.225em hsl(var(--accent) / 0.3);
  }

  .checkbox:checked {
    background-color: hsl(var(--accent));
  }
</style>
