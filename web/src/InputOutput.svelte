<script lang="ts">
  export let dna = "";
  export let pipeline = [
    "No tool selected",
    [{ name: "No tool selected", functionality: (o) => null }],
  ];
  export let seq = ""
</script>

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
      >{`${pipeline[0]}: ${pipeline[1][0].functionality(seq)}`}</textarea
    >
  </div>
</div>
