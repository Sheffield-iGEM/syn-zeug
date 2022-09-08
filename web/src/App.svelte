<script lang="ts">
  import { Seq } from "biobox";
  import { AppBar, LightSwitch } from "@brainandbones/skeleton";
  import Logo from "../Logo.svg";
  import InputOutput from "./InputOutput.svelte";
  import Operations from "./Operations.svelte";
  import Recipe from "./Recipe.svelte";

  let dna = "";
  let input = "";
  let searched = "";
  let darkBg = true;
  let seq = null; // TODO: Add a real type!

  let functions = [
    {
      name: "Reverse Complement",
      functionality: (o) => o.reverse_complement().to_string(),
    },
    { name: "Sequence Length", functionality: (o) => o.len() },
    { name: "Reverse Sequence", functionality: (o) => o.rev().to_string() },
    {
      name: "Count Sequence Elements",
      functionality: (o) => JSON.stringify([...o.count_elements().entries()]),
    },
    {
      name: "Sequence Conversion",
      functionality: (o) => o.convert("Rna").to_string(),
    },
    { name: "Type", functionality: (o) => `${o.kind()} (${o.alphabet()})` },
  ];

  let pipeline = [
    "No tool selected",
    [{ name: "No tool selected", functionality: (o) => null }],
  ];

  $: input = dna;
  $: try {
    seq = new Seq(dna.trim());
  } catch (e) {
    seq = new Seq("");
    input = e;
  }

  let recipeFunctions = [];

  let functionsDroppedFlag = false;

  const onChangeFunctionDroppedFlag = () => {
    functionsDroppedFlag = true;
  };

  const onFunctionDropped = (val) => {
    if (functionsDroppedFlag) {
      recipeFunctions.push(val);
    }
    console.log(recipeFunctions);
  };

  const handleSelectedTool = (name) => {
    pipeline = [name, functions.filter((func) => func.name == name)];
    console.log(pipeline);
  };
</script>

<main>
  <AppBar class="h-16">
    <svelte:fragment slot="lead"
      ><i class="fa fa-bars nav-icon" /></svelte:fragment
    >
    <div class="search-functions">
      <input
        id="search"
        type="search"
        name="search-function"
        placeholder="Search a function.."
        bind:value={searched}
      />
    </div>
    <svelte:fragment slot="trail"
      >
      <div class="flex justify-around align-center ml-4">
        <img src={Logo} class="ml-6" alt="logo of the igem team" srcset="" />
        <LightSwitch class="ml-6"/>
      </div>
    </svelte:fragment>
  </AppBar>

  <div id="wrapper" class="grid-content">
    <Operations
      {searched}
      {functions}
      {handleSelectedTool}
      {onFunctionDropped}
    />
    <div class="gutter" />
    <Recipe {onChangeFunctionDroppedFlag} {recipeFunctions} />
    <div class="gutter" />
    <InputOutput {dna} {pipeline} {seq} />
  </div>
</main>

<style>
  img {
    width: 40px;
  }

  .nav-icon {
    height: 40px;
    width: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .nav-icon:hover {
    box-shadow: 20px 30px 38px rgb(188, 193, 201);
    background-color: rgb(188, 193, 201);
    border-radius: 50%;
  }
</style>
