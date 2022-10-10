<script lang="ts">
  import { ListBox, ListBoxItem } from "@brainandbones/skeleton";
  import { writable } from "svelte/store";
  import Cell from "./Cell.svelte";
  import CountElements from "./Tools/CountElements.svelte";
  import Dna from "./Tools/DNA.svelte";
  import LowerCase from "./Tools/LowerCase.svelte";
  import Orf from "./Tools/ORF.svelte";
  import Protein from "./Tools/Protein.svelte";
  import Reverse from "./Tools/Reverse.svelte";
  import ReverseComplement from "./Tools/ReverseComplement.svelte";
  import Rna from "./Tools/RNA.svelte";
  import UpperCase from "./Tools/UpperCase.svelte";

  export let value;
</script>

<Cell title="Pipeline">
  <ListBox selected={value} accent="bg-accent-500">
    <LowerCase />
    <UpperCase />
    <Reverse />
    <CountElements />
    <ReverseComplement />
    <Rna />
    <Dna />
    <Protein />
    <Orf />
  </ListBox>
</Cell>
