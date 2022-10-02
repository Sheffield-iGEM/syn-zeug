<script lang="ts">
  import { Seq } from "biobox";
  import Header from "./Header.svelte";
  import Main from "./Main.svelte";
  import Footer from "./Footer.svelte";

  let dna = "";
  let input = "";
  let output = "";
  let searched = "";
  let darkBg = true;
  let seq = null; // TODO: Add a real type!
  const storageSize = 1000;
  let display = false;

  let functions = [
    { name: "Type", functionality: (o) => `${o.kind()} (${o.alphabet()})` },
    { name: "Sequence Length", functionality: (o) => o.len() },
    {
      name: "Convert to Lower Case",
      functionality: (o) => o.normalize_case("Lower").to_string(),
    },
    {
      name: "Convert to Upper Case",
      functionality: (o) => o.normalize_case("Upper").to_string(),
    },
    { name: "Reverse Sequence", functionality: (o) => o.rev().to_string() },
    {
      name: "Count Elements",
      functionality: (o) => {
        return [...o.count_elements()]
          .sort()
          .map((pair) => pair.join(": "))
          .join("\n");
      },
    },
    {
      name: "Reverse Complement",
      functionality: (o) => o.reverse_complement().to_string(),
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
    {
      name: "Find ORFs",
      functionality: (o) => {
        return o
          .find_orfs(1)
          .map((p) => {
            let seq = Seq.from_js(p[1]).convert("Protein").to_string();
            return `S${p[0].start} E${p[0].end} O${p[0].offset}: ${seq}`;
          })
          .join("\n");
      },
    },
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

<div id="app" class="w-screen h-screen">
  <Header />
  <Main />
  <Footer />
</div>
