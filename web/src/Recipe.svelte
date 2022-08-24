<script lang="ts">
  import Button from '@smui/button';
  import { onMount } from "svelte";
  import RecipeFunctions from "./RecipeFunctions.svelte";

  export let recipeFunctions = ["this", "that"];
  $: recipeOperations = recipeFunctions.map((recipe) => {
    return recipe;
  });
  export let onChangeFunctionDroppedFlag = () => console.log("called");

  onMount(() => {
    const recipes = document.querySelector("#recipe");
    recipes.addEventListener("dragenter", () => {
      onChangeFunctionDroppedFlag();
    });
  });
  
</script>

<div id="recipe">
  <div class="flex-column">
    <div class="title grid-title">
      <p>Recipe</p>
      <i class="fas fa-save" />
      <i class="fas fa-folder" />
      <i class="fas fa-trash" />
    </div>
    <div name="recipe" class="text-area">
      {#each recipeOperations as recipe}
        <RecipeFunctions functionName={recipe} />
      {/each}
    </div>
    <div class="controls flex-row">
      <Button>Verbose Recipe</Button>
    </div>
  </div>
</div>
