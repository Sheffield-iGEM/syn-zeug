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
  <div class="flex-column mt-4">
    <div class="recipe-card">
      <!-- {#each recipeOperations as recipe}
        <RecipeFunctions functionName={recipe} />
      {/each} -->
      <Card class="flex justify-center align-center">
        <div class="flex justify-between w-[250px] align-center m-4">
          <p>Function Name</p>
          <i class="fa fa-power-off" />
        </div>
        <Button
          class="bg-green-400"
          size="base"
          color="text-white"
          ring="ring-transparent"
          weight="ring-none"
          rounded="rounded-lg"
          width="w-auto"
          disabled={false}
        >
          <svelte:fragment slot="lead">See Example</svelte:fragment>
        </Button>
      </Card>
    </div>
  </div>
</div>

<style>
  .recipe-card {
    animation: fade-in 1s;
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  i {
    color: rgb(16, 185, 129);
    cursor: pointer;
  }
</style>
