<script lang="ts">
  import { Seq } from "biobox";
  let dna = "";
  let input = "";
  let seq = null; // FIXME: Add a real type!
  let kind = "";
  let len = 0;
  let count = null; // FIXME: What type should this really be?
  let revcomp = "";
  let rna = "";
  $: input = dna;
  $: try {
    seq = new Seq(dna.trim());
  } catch (e) {
    seq = new Seq("");
    input = e;
  }
  $: kind = seq.kind();
  $: len = seq.len();
  $: count = JSON.stringify([...seq.count_elements().entries()]);
  $: try {
    revcomp = seq.reverse_complement().to_string();
  } catch (e) {
    revcomp = e;
  }
  $: try {
    rna = seq.convert("Rna").to_string();
  } catch (e) {
    rna = e;
  }
</script>

<main>
  <nav class="nav-grid">
    <div class="site-title">
      <h1>i<a>GEM</a></h1>
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
        <div class="functions">
          <input
            id="search"
            type="search"
            name="search-function"
            placeholder="Search a function.."
          />
        </div>
        <div class="functions">
          <a href="#">Count bases</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
        <div class="functions">
          <a href="#">that</a>
        </div>
        <div class="functions">
          <a href="#">this</a>
        </div>
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
        <textarea name="recipe" class="text-area" cols="30" rows="10" />
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
          <i class="fas fa-trash" />
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
          <i class="fas fa-save" />
          <i class="fas fa-copy" />
          <i class="fas fa-reply-all" />
        </div>
        <textarea name="output" class="text-area" cols="30" rows="10"
          >{`Input: "${input}"
Type: ${kind}
Length: ${len}
Counts: ${count}
RevComp: "${revcomp}"
To RNA: "${rna}"`}</textarea>
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
  :root,
  :root.classic {
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
  }
  :global(body) {
    background-color: var(--secondary-color);
    max-height: 100vh;
    margin: 0px;
    overflow: hidden;
    scrollbar-color: var(--secondary-color);
  }
  /*-------------------------------------------------- General Styling ---------------------------------------------------------------------------------*/
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
  .grid-title {
    display: grid;
    grid-template-columns: 82% 6% 6% 6%;
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
