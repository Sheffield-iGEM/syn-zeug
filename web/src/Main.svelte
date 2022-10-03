<script lang="ts">
  import { Seq } from "biobox";
  import Grid from "./assets/Grid.svg";
  import Cell from "./Main/Cell.svelte";
  import Input from "./Main/Input.svelte";
  import Output from "./Main/Output.svelte";
  import { Card } from "@brainandbones/skeleton";
  import type { DialogAlert } from "@brainandbones/skeleton";
  import { Dialog } from "@brainandbones/skeleton";
  import { dialogStore } from "@brainandbones/skeleton";
  import Popup from "./Popup.svelte";
  import { each } from "svelte/internal";
  import Function from "./Function.svelte";

  let input = "";
  let output = "";
  let seq = new Seq("");

  $: seq = new Seq(input);
  $: output = seq.reverse_complement().to_string();

  const d: DialogAlert = {
    title: "",
    body: "",
    component: {
      element: Popup,
    },
  };

  const displayPopup = () => {
    console.log(dialogStore);
    dialogStore.trigger(d);
  };

  let chainedFunctions = [];
</script>

<main
  class="flex portrait:flex-col landscape:flex-row justify-evenly items-center bg-surface-200 dark:bg-surface-800 bg-repeat"
  style="background-image: url({Grid})"
>
  <Input bind:value={input} />
  <Cell title="Pipeline">
    <Dialog
      blur="backdrop-blur-sm"
      card="card h-full overflow-y-scroll md:overflow-hidden md:h-[620px]"
      duration={250}
    />
    <div class="flex flex-col justify-center items-center w-full h-full">
      {#if chainedFunctions.length > 0}
        {#each chainedFunctions as chained}
          <Function functionName={chained} />
        {/each}
      {:else}
        <button class="btn" on:click={displayPopup}>
          <div class="fa fa-plus" />
        </button>
      {/if}
    </div>
  </Cell>
  <Output bind:value={output} />
</main>

<style>
  button {
    font: inherit;
    border: none;
    cursor: pointer;
  }

  input,
  label {
    font: inherit;
  }

  :root {
    --_hue: 135;
    --_size: 1.8rem;
    --_radius: 0.2em;
    --_tspeed_fast: 180ms;
    --_tspeed_slow: 300ms;
    --_ttype_squish: cubic-bezier(0.86, -0.1, 0.27, 1.15);

    /* Light Mode */
    --bg--light: var(--_hue) 30% 94%;
    --txt--light: var(--_hue) 40% 5%;
    --accent--light: var(--_hue) 55% 50%;
    --accent1--light: 10 80% 60%;
    --muted--light: var(--_hue) 30% 99%;
  }

  @media (prefers-color-scheme: light) {
    :root {
      --bg: var(--bg--light);
      --txt: var(--txt--light);
      --accent: var(--accent--light);
      --accent1: var(--accent1--light);
      --muted: var(--muted--light);
      color-scheme: light;
    }
  }

  [color-scheme="light"] {
    --bg: var(--bg--light);
    --txt: var(--txt--light);
    --accent: var(--accent--light);
    --accent1: var(--accent1--light);
    --muted: var(--muted--light);
    color-scheme: light;
  }

  @media screen and (max-width: 768px) {
    :root {
      --_size: 1.3rem;
    }
  }

  /* prefers reduced motion */
  @media (prefers-reduced-motion: reduce) {
    :root {
      --_tspeed_slow: 50ms;
      --_tspeed_fast: 50ms;
    }
  }

  body {
    background-color: hsl(var(--bg));
    color: hsl(var(--txt));
  }

  ::selection {
    background: hsl(var(--accent) / 0.8);
    color: hsl(var(--bg));
  }

  .btn {
    height: 50px;
    width: 50px;
    display: flex;
    place-items: center;
    gap: 0.5em;
    background-color: hsl(var(--accent));
    color: hsl(var(--bg));
    text-decoration: none;
    padding: 0.5em;
    border-radius: var(--_radius);
    box-shadow: 0.05em 0.1em 0.9em hsl(var(--accent) / 0.3),
      0 0 0 -0.1em hsl(var(--bg)), 0 0 0 -0.2em hsl(var(--accent));
    transition: box-shadow var(--_tspeed_fast) var(--_ttype_squish),
      background-color var(--_tspeed_fast) var(--_ttype_squish);
  }

  .btn :where(svg, img, span) {
    pointer-events: none;
  }

  .btn :where(svg, img) {
    width: var(--_size);
    height: var(--_size);
  }

  .btn:where(:active, :hover) {
    background-color: hsl(var(--accent) / 0.7);
    box-shadow: 0 0 0 hsl(var(--accent) / 0.3), 0 0 0 -0.1em hsl(var(--bg)),
      0 0 0 -0.2em hsl(var(--accent));
  }

  .btn:focus {
    outline: none;
  }

  .btn:focus-visible {
    box-shadow: 0 0 0 hsl(var(--accent) / 0.3), 0 0 0 0.2em hsl(var(--bg)),
      0 0 0 0.4em hsl(var(--accent) / 0.7);
  }
</style>
