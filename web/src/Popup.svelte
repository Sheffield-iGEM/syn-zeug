<script lang="ts">
  import { writable } from "svelte/store";
  import { Button, Divider } from "@brainandbones/skeleton";
  import { Card } from "@brainandbones/skeleton";
  import { ListItem, List } from "@brainandbones/skeleton";
  import Something from "./something.svelte";

  export let searched = "";
  const storeSingle = writable(1);
</script>

<!-- popup card -->
<Card class="shadow-2xl shadow-[rgb(65,225,222)] w-[300px] md:w-2/3 h-2/3 overflow-y-scroll">
  <!-- search function -->
  <!-- the following div is used to center the search bar -->
  <div class="flex justify-center items-center w-full m-2">
    <Something />
  </div>

  <Divider vertical={true} borderWidth="border-l" />

  <!-- functions and descriptions -->
  <!-- the default styling assumes that the screen is small, for larger screens it is used md: -->
  <div
    class="flex flex-col md:flex-row md:items-start w-full justify-center items-center "
  >
    <!-- functions card -->
    <Card class="w-[250px] mt-4 md:w-1/2">
      <p class="m-3 font-bold">Functions</p>
      <List selected={storeSingle} label="Single Selection">
        <ListItem class="cursor-pointer rounded" value={1}
          >Selection Example 1</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={2}
          >Selection Example 2</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={3}
          >Selection Example 1</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={4}
          >Selection Example 2</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={5}
          >Selection Example 1</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={6}
          >Selection Example 2</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={5}
          >Selection Example 1</ListItem
        >
        <ListItem class="cursor-pointer rounded" value={6}
          >Selection Example 2</ListItem
        >
      </List>
    </Card>

    <!-- descriptions card -->
    <Card class="w-[250px] md:w-1/2 mt-4 flex flex-col">
      <div class="flex w-full justify-center items-center">
        <h3 class="font-bold">example</h3>
      </div>
      <Card class="m-3 shadow-inner shadow-2xl h-[250px]" />
      <div class="flex items-center justify-around">
        <Button>Apply</Button>
        <Button>
          <i class="fa fa-trash" />
        </Button>
      </div>
    </Card>
  </div>
</Card>

<!-- TODO: this is the backup search functionality -->

<!-- <div class="search-functions w-[250px]">
  <input
    id="search"
    type="search"
    name="search-function"
    placeholder="Search a function.."
    bind:value={searched}
  />
</div> -->
<style>
  .active {
    color: rgb(95, 230, 230);
  }

</style>
