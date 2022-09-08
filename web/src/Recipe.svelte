<script lang="ts">
  import { Button, Card } from "@brainandbones/skeleton";
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
    </div>
    <div>
      <!-- {#each recipeOperations as recipe}
        <RecipeFunctions functionName={recipe} />
      {/each} -->
      <Card class="flex justify-center align-center">
          <div class="flex-justify-around w-full">
          <p>Function Name</p>
          <i class="fa fa-power-off"></i>
        </div>
        <Button>See Example</Button>
      </Card>
    </div>
  </div>
</div>
