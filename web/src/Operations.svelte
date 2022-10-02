<script lang="ts">
  import { Card } from "@brainandbones/skeleton";
  import { Divider } from "@brainandbones/skeleton";
  import { SlideToggle } from "@brainandbones/skeleton";
  import { Tooltip } from "@brainandbones/skeleton";
  import { AccordionGroup, AccordionItem } from "@brainandbones/skeleton";

  export let searched = "";
  
  export let functions = [
    {
      name: "Reverse Complement",
      functionality: (o) => o.reverse_complement().to_string(),
    },
  ];

  let activeFunctions = functions.map((func) => {
    return { myValue: false };
  });

  export let handleSelectedTool = (val) => console.log(val);

</script>

<div id="operations">
  <div class="flex-column mt-4">

    <AccordionGroup>
      <Card>
        <AccordionItem open>
          <svelte:fragment slot="summary">Chainable Functions</svelte:fragment>
          <svelte:fragment slot="content">

            {#each functions as func}
              <div class="functions-grid">
                <div class="m-4 func">
                  <Tooltip
                    background="bg-accent-500"
                    color="text-primary-200"
                    width="w-[300px]"
                    whitespace="whitespace-normal"
                    rounded="rounded-xl"
                    duration={0}
                  >
                    <svelte:fragment slot="message">
                      <h3 class="mb-1">Hello, Skeleton 💀</h3>
                      <span class="text-xs text-white/60"
                        >Lorem ipsum dolor sit, amet consectetur adipisicing
                        elit. Obcaecati id atque laboriosam provident eum
                        facere, architecto veniam.</span
                      >
                    </svelte:fragment>
                    <svelte:fragment slot="content">
                      <p on:click={() => handleSelectedTool(func.name)}>
                        {func.name}
                      </p>
                    </svelte:fragment>
                  </Tooltip>
                </div>

                <SlideToggle
                  class="m-2"
                  bind:checked={activeFunctions[functions.indexOf(func)]
                    .myValue}
                  size="sm"
                  accent="bg-primary-500"
                />
                <Divider />
              </div>
            {/each}
          </svelte:fragment>
        </AccordionItem>
      </Card>
    </AccordionGroup>

    <AccordionGroup>
      <Card>
        <AccordionItem open>
          <svelte:fragment slot="summary">Non Chainable Functions</svelte:fragment>
          <svelte:fragment slot="content">

            {#each functions as func}
              <div class="functions-grid">
                <div class="m-4 func">
                  <Tooltip
                    background="bg-accent-500"
                    color="text-primary-200"
                    width="w-[300px]"
                    whitespace="whitespace-normal"
                    rounded="rounded-xl"
                    duration={0}
                  >
                    <svelte:fragment slot="message">
                      <h3 class="mb-1">Hello, Skeleton 💀</h3>
                      <span class="text-xs text-white/60"
                        >Lorem ipsum dolor sit, amet consectetur adipisicing
                        elit. Obcaecati id atque laboriosam provident eum
                        facere, architecto veniam.</span
                      >
                    </svelte:fragment>
                    <svelte:fragment slot="content">
                      <p on:click={() => handleSelectedTool(func.name)}>
                        {func.name}
                      </p>
                    </svelte:fragment>
                  </Tooltip>
                </div>

                <SlideToggle
                  class="m-2"
                  bind:checked={activeFunctions[functions.indexOf(func)]
                    .myValue}
                  size="sm"
                  accent="bg-primary-500"
                />

                <Divider />

              </div>
            {/each}
          </svelte:fragment>
        </AccordionItem>
      </Card>
    </AccordionGroup>
  </div>
</div>

<style>
  .functions-grid {
    display: grid;
    grid-template-columns: 70% 30%;
  }

  .func {
    cursor: pointer;
  }
</style>
