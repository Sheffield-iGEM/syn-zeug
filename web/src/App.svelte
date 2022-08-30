<script lang="ts">
  import { Button } from "@brainandbones/skeleton";
  import { Seq } from "biobox";
  import Logo from "../Logo.svg";

  let dna = "";
  let input = "";
  let output = "";
  let searched = "";
  let darkBg = true;
  let seq = null; // TODO: Add a real type!
  const storageSize = 1000;
  let display = false;

  let functions = [
    {
      name: "Reverse Complement",
      functionality: (o) => o.reverse_complement().to_string(),
    },
    { name: "Sequence Length", functionality: (o) => o.len() },
    { name: "Reverse Sequence", functionality: (o) => o.rev().to_string() },
    {
      name: "Count Sequence Elements (Bases / Residues)",
      functionality: (o) => {
        return [...o.count_elements()]
          .sort()
          .map((pair) => pair.join(": "))
          .join("\n");
      },
    },
    {
      name: "Convert to RNA",
      functionality: (o) => o.convert("Rna").to_string(),
    },
    {
      name: "Convert to DNA",
      functionality: (o) => o.convert("Dna").to_string(),
    },
    {
      name: "Convert to Protein",
      functionality: (o) => o.convert("Protein").to_string(),
    },
    { name: "Type", functionality: (o) => `${o.kind()} (${o.alphabet()})` },
  ];

  let chanableFunctions = [
    { name: "this", functionality: (o) => null },
    { name: "that", functionality: (o) => null },
  ];

  let pipeline = [
    "No tool selected",
    [{ name: "No tool selected", functionality: (o) => null }],
  ];

  $: input = dna;
  $: try {
    seq = new Seq(dna.trim());
  } catch (e) {
    output = e;
  }
  $: try {
    output = pipeline[1][0].functionality(seq);
  } catch (e) {
    output = e;
  }

  const handleDisplay = () => {
    chanableFunctions.forEach((func) => {
      let functions = document.getElementsByClassName("functions");
      for (let f of functions) {
        if (func.name == f.innerText) {
          f.classList.toggle("inactive", display);
          f.children.item(0).classList.toggle("inactive", display);
        }
      }
    });

    display = !display;
  };

  const handleCopy = () => {
    let elem = document.createElement("textarea");
    document.body.appendChild(elem);
    elem.value = pipeline[1][0].functionality(seq);
    elem.select();
    document.execCommand("copy");
    document.body.removeChild(elem);
  };

  const handleExport = () => {
    let elem = document.createElement("textarea");
    document.body.appendChild(elem);

    let itemsToRetrieve = [];
    for (let i = 0; i < storageSize; i++) {
      try {
        itemsToRetrieve.push(
          JSON.parse(localStorage.getItem(localStorage.key(i)))
        );
      } catch (e) {
        console.log(e);
      }
    }
    itemsToRetrieve = itemsToRetrieve.filter((item) => item !== null);
    console.log(itemsToRetrieve);
    elem.value = JSON.stringify(itemsToRetrieve);
    elem.select();
    document.execCommand("copy");
    document.body.removeChild(elem);
  };

  const handleSave = () => {
    const timeElapsed = Date.now();
    const today = new Date(timeElapsed);
    const output = {
      time: today,
      name: pipeline[0],
      outputText: pipeline[1][0].functionality(seq),
    };
    localStorage.setItem(output.name, JSON.stringify(output));
    console.log("Output has been saves to local storage", output);
  };

  const handleBgChange = () => {
    const bodyElement = document.querySelector("body");
    bodyElement.classList.toggle("light", !darkBg);
    darkBg = !darkBg;
  };

  const handleSelectedTool = (e) => {
    let name = e.target.innerText;
    pipeline = [name, functions.filter((func) => func.name == name)];
    console.log(pipeline[1][0]);
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
      <li><a href="#">About</a></li>
      <li><a href="#">Contact</a></li>
      <li><a href="#">Projects</a></li>
    </ul>
  </nav>
  <div id="wrapper" class="grid-content">
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
            class={func.name.includes(searched) || searched == ""
              ? "functions"
              : "functions inactive"}
            on:click={(e) => handleSelectedTool(e)}
          >
            <a
              class={func.name.includes(searched) || searched == ""
                ? ""
                : "inactive"}>{func.name}</a
            >
          </div>
        {/each}
        <div
          class={"Chanable Functions".includes(searched) || searched == ""
            ? "functions"
            : "functions inactive"}
          id="chanable-functions"
          on:click={handleDisplay}
        >
          <a
            class={"Chanable Functions".includes(searched) || searched == ""
              ? ""
              : "inactive"}>Chanable Functions</a
          >
        </div>
        {#each chanableFunctions as func}
          <div
            class={func.name.includes(searched) || searched == ""
              ? "functions"
              : "functions inactive"}
            on:click={(e) => handleSelectedTool(e)}
          >
            <a
              class={func.name.includes(searched) || searched == ""
                ? ""
                : "inactive"}>{func.name}</a
            >
          </div>
        {/each}
      </div>
    </div>
    <div class="gutter" />
    <div id="recipe">
      <div class="flex-column">
        <div class="title grid-title">
          <p>Recipe</p>
          <i class="fas fa-save" />
          <i class="fas fa-folder" />
          <i class="fas fa-trash" />
        </div>
        <textarea name="recipe" class="text-area" cols="30" rows="10"
          >{pipeline[0]}</textarea
        >
        <div class="controls flex-row">
          <button class="btn">Verbose Recipe</button>
        </div>
      </div>
    </div>
    <div class="gutter" />
    <div id="IO">
      <div class="flex-column">
        <div class="title grid-title">
          <p>Input</p>
          <i class="fas fa-folder-plus" />
          <i class="fas fa-upload" />
          <i class="fas fa-trash" on:click={() => (dna = "")} />
        </div>
        <textarea
          bind:value={dna}
          name="input"
          class="text-area"
          cols="30"
          rows="10"
        />
        <div class="title grid-title">
          <p>Output</p>
          <i class="fas fa-save" on:click={handleSave} />
          <i class="fas fa-copy" on:click={handleCopy} />
          <i class="fas fa-reply-all" on:click={handleExport} />
        </div>
        <textarea name="output" class="text-area" cols="30" rows="10"
          >{output}</textarea
        >
      </div>
    </div>
  </div>
</main>

<style>
  @import url("https://fonts.googleapis.com/css?family=Lato:100");
  @import url("https://fonts.googleapis.com/css2?family=Red+Hat+Mono:wght@300&display=swap");
  @import url("https://fonts.googleapis.com/css2?family=Source+Code+Pro:wght@200&display=swap");
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  :global(body) {
    --primary-font-family: "Red Hat Mono", monospace;
    --primary-font-colour: #2bc1e2;
    --secondary-font-color: #abb8ca;
    --primary-font-size: 1.3rem;
    --primary-font-weight: 600;
    --secondary-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI",
      Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue",
      sans-serif;
    --primary-color: #111827;
    --secondary-color: #1d2432;
    --tertiary-color: #050c1b;
    --nav-color: #39424e;
    --borders-color: #111827;
    background-color: var(--secondary-color);
    max-height: 100vh;
    margin: 0px;
    overflow: hidden;
    scrollbar-color: var(--secondary-color);
  }
  /*-------------------------------------------------- General Styling ---------------------------------------------------------------------------------*/
  /* TODO: find a better light color pallete please */
  :global(body.light) {
    --primary-font-family: "Red Hat Mono", monospace;
    --primary-font-colour: #18394a;
    --secondary-font-color: red;
    --primary-font-size: 1.3rem;
    --primary-font-weight: 600;
    --secondary-font-family: -apple-system, BlinkMacSystemFont, "Segoe UI",
      Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue",
      sans-serif;
    --primary-color: white;
    --secondary-color: #62839c;
    --tertiary-color: #62839c;
    --nav-color: #62839c;
    --borders-color: #ffeea4;
    --static-color: white;
    background-color: var(--secondary-color);
    max-height: 50vh;
    margin: 0px;
    overflow: hidden;
    scrollbar-color: var(--secondary-color);
  }
  .flex-row {
    display: flex;
    justify-content: space-around;
    flex-direction: row;
  }
  .flex-column {
    display: flex;
    flex-direction: column;
  }
  .grid-content {
    display: grid;
    grid-template-columns: 20% 0.2% 39.8% 0.2% 39.8%;
  }

  .logo {
    width: 40px;
    margin-top: 10px;
  }

  .grid-title {
    display: grid;
    grid-template-columns: 82% 6% 6% 6%;
  }

  .inactive {
    display: none;
  }

  .btn {
    height: 60px;
    width: 200px;
    padding: 2px;
    background-color: white;
    color: var(--primary-font-colour);
    font-size: 1.2rem;
    font-weight: 900;
    font-family: "Source Code Pro", monospace;
    border-radius: 10px;
  }
  .title i:hover {
    color: var(--secondary-font-color);
  }
  /*---------------------------------------------------COMPONENTS-----------------------------------------------------------------------------------------*/
  .gutter {
    background-color: var(--borders-color);
    width: 100%;
  }
  .title {
    background-color: var(--primary-color);
    height: 50px;
    color: var(--primary-font-colour);
    font-size: 1.3rem;
    border: 0.5px var(--borders-color) solid;
    padding: 10px;
    font-family: var(--primary-font-family);
    font-weight: var(--primary-font-weight);
  }
  .functions {
    border: 0.5px var(--borders-color) solid;
    font-family: var(--secondary-font-family);
    background-color: var(--tertiary-color);
    font-size: 1.2rem;
    font-weight: 500;
    display: flex;
    padding: 10px;
    height: 70px;
    align-items: center;
  }
  .functions:hover {
    background-color: var(--primary-color);
  }
  .functions a:hover {
    border-bottom: 1px solid var(--secondary-font-color);
  }
  .functions a {
    text-decoration: none;
    color: var(--secondary-font-color);
  }

  .search-functions {
    border: 0.5px var(--borders-color) solid;
    font-family: var(--secondary-font-family);
    background-color: var(--tertiary-color);
    font-size: 1.2rem;
    font-weight: 500;
    display: flex;
    padding: 10px;
    height: 70px;
    align-items: center;
  }
  .search-functions:hover {
    background-color: var(--primary-color);
  }
  .search-functions a:hover {
    border-bottom: 1px solid var(--secondary-font-color);
  }
  .search-functions a {
    text-decoration: none;
    color: var(--secondary-font-color);
  }

  .text-area {
    height: 500px;
    margin: 0px;
    border-top-style: hidden;
    border-right-style: hidden;
    border-left-style: hidden;
    border-bottom-style: hidden;
    background-color: var(--secondary-color);
    width: 100%;
    font-size: 1.1rem;
    font-weight: var(--primary-font-weight);
    color: var(--secondary-font-color);
    font-family: "Source Code Pro", monospace;
  }
  .controls {
    background-color: var(--primary-color);
    border: 0.5px var(--borders-color) solid;
    padding: 10px;
    align-items: center;
    margin-top: calc(100vh - 907px);
  }
  /* -------------------------- SPECIFIC ----------------------*/
  #operations {
    height: 100vh;
    background-color: var(--borders-color);
    overflow: scroll;
  }
  #IO {
    height: 100vh;
  }
  #recipe {
    height: 100vh;
  }
  #recipe .flex-column {
    height: 100%;
  }
  #recipe .text-area {
    height: 700px;
  }
  #search {
    height: 100%;
    width: 100%;
    font-family: var(--primary-font-family);
    font-weight: var(--primary-font-weight);
    border: 1px solid grey;
    border-radius: 10px;
    padding: 10px;
  }
  .nav-grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
  }
  .site-title {
    width: 50%;
    height: 100%;
    align-items: center;
    margin: auto;
    font-family: var(--primary-font-family);
  }
  .site-title {
    animation: fade-in-bottom-right 2s;
  }
  .site-title a {
    color: var(--primary-font-colour);
  }
  .site-title {
    color: white;
  }
  .site-title h1 {
    animation: flash 4s infinite;
    margin: 13px;
  }
  nav {
    height: 6vh;
    background: var(--nav-color);
  }
  .nav-links {
    width: 50%;
    height: 100%;
    align-items: center;
    margin-left: auto;
    list-style: none;
  }
  .nav-links a {
    font-family: "Source Code Pro", monospace;
    color: white;
    text-decoration: none;
  }
  .nav-links a:hover {
    border-bottom: 1px solid white;
  }

  @keyframes fade-in-bottom-right {
    from {
      opacity: 0;
      transform: translate3d(100%, 100%, 0);
    }
    to {
      opacity: 1;
      transform: translate3d(0, 0, 0);
    }
  }
  @keyframes flash {
    0%,
    50%,
    100% {
      opacity: 1;
    }
    25%,
    75% {
      opacity: 0.5;
    }
  }
</style>
