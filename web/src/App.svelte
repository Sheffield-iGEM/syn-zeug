<script lang="ts">
  import { Seq } from "biobox";
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
      name: "Count Sequence Elements (Bases / Residues)",
      functionality: (o) => JSON.stringify([...o.count_elements().entries()]),
    },
    {
      name: "Sequence Conversion (DNA -> RNA)",
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

  const handleBgChange = () => {
    const bodyElement = document.querySelector("body");
    bodyElement.classList.toggle("light", !darkBg);
    darkBg = !darkBg;
  };

  const handleSelectedTool = (e) => {
    let name = e.target.innerText;
    pipeline = [name, functions.filter((func) => func.name == name)];
    console.log(pipeline);
  };
</script>

<main>
  <nav class="nav-grid">
    <div class="site-title">
      <img
        src={Logo}
        alt="University of Sheffield iGEM Logo"
        class="logo"
        on:click={handleBgChange}
      />
    </div>
    <ul class="nav-links flex-row">
      <li>
        <i class="fas fa-grip-dots" />
      </li>
      <li><a href="#">Contact</a></li>
      <li><a href="#">Feedback</a></li>
    </ul>
  </nav>
  <div id="wrapper" class="grid-content">
    <Operations
      {searched}
      {functions}
      {handleSelectedTool}
      {onFunctionDropped}
    />
    <div class="gutter" />
    <Recipe {onChangeFunctionDroppedFlag} {recipeFunctions}/>
    <div class="gutter" />
    <InputOutput {dna} {pipeline} {seq} />
  </div>
</main>
<!-- 
TODO: Here, (thing.id) is the key, which tells Svelte how to 
figure out which DOM node to change when the component updates.

{#if x > 10}
	<p>{x} is greater than 10</p>
{:else if 5 > x}
	<p>{x} is less than 5</p>
{:else}
	<p>{x} is between 5 and 10</p>
{/if}

spread props
<script>
	import Info from './Info.svelte';

	const pkg = {
		name: 'svelte',
		version: 3,
		speed: 'blazing',
		website: 'https://svelte.dev'
	};
</script>

<Info {...pkg}/>
-->
