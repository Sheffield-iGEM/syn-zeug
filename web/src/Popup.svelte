<script lang="ts">
  import { writable } from "svelte/store";
  import { Button, Divider } from "@brainandbones/skeleton";
  import { Card } from "@brainandbones/skeleton";
  import { ListBoxItem, ListBox } from "@brainandbones/skeleton";
  import Search from "./Search.svelte";

  export let searched = "";
  const storeSingle = writable(1);
</script>

<!-- search function -->
<!-- the following div is used to center the search bar -->
<div class="flex justify-center items-center w-full m-2">
  <Search />
</div>

<Divider borderWidth="border-l" />

<!-- functions and descriptions -->
<!-- the default styling assumes that the screen is small, for larger screens it is used md: -->
<div
  class="flex flex-col md:flex-row md:items-start w-full justify-center items-center "
>
  <!-- functions card -->
    <Card class="w-[250px] mt-4 md:w-1/2 overflow-y-scroll h-[400px]">
    <ListBox selected={storeSingle} label="Single Selection">
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={1}>Selection Example 1</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={2}>Selection Example 2</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={3}>Selection Example 1</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={4}>Selection Example 2</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={5}>Selection Example 1</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={6}>Selection Example 2</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={7}>Selection Example 1</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={8}>Selection Example 2</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={9}>Selection Example 1</ListBoxItem
      >
      <ListBoxItem
        accent="bg-green-500"
        class="cursor-pointer rounded"
        value={10}>Selection Example 2</ListBoxItem
      >
    </ListBox>
  </Card>

  <!-- descriptions card -->
  <Card class="w-[250px] md:w-1/2 mt-4 flex flex-col">
    <div class="flex w-full justify-center items-center">
      <h3 class="font-bold">example</h3>
    </div>
    <Card class="m-3 bg-white h-[250px]" />
    <div class="flex items-center justify-around">
      <button class="btn"><i class="fa fa-edit fa-2x" /></button>
      <button class="btn delete">
        <i class="fa fa-trash fa-2x" />
      </button>
    </div>
  </Card>
</div>

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
    height: 60px;
    width: 60px;
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

  .delete {
    --accent: var(--accent1);
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
