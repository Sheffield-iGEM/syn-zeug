<script lang="ts">
  import { onMount } from "svelte";

  export let searched = "";
  export let functions = [
    {
      name: "Reverse Complement",
      functionality: (o) => o.reverse_complement().to_string(),
    },
  ];
  export let handleSelectedTool = (val) => console.log(val);
  export let onFunctionDropped = (val) => console.log(val);

  onMount(() => {
    const functions = document.querySelectorAll(".functions");
    Array.from(functions).forEach((func) => {
      func.addEventListener("dragend", (e) => {
        onFunctionDropped(e.target.childNodes.item(0).innerText);
      });
    });
  });
</script>

<div id="operations">
  <div class="flex-column">
    <div class="title">Operations</div>
    <div class="search-functions">
      <input
        id="search"
        type="search"
        name="search-function"
        placeholder="Search a function.."
        bind:value={searched}
      />
    </div>
    <!-- TODO: change this to have the functions you want popping up from the top
          this can be done by filtering the functions list into a filtered functions array 
          and then view only the filtered functions array
           -->
    {#each functions as func}
      <div
        draggable="true"
        class={func.name.includes(searched) || searched == ""
          ? "functions"
          : "functions inactive"}
        on:click={(e) => handleSelectedTool(e)}
      >
        <p
          class={func.name.includes(searched) || searched == ""
            ? ""
            : "inactive"}
        >
          {func.name}
        </p>
      </div>
    {/each}
  </div>
</div>
