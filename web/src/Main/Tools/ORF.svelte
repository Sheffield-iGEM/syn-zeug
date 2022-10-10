<script lang="ts">
  import { Seq } from "biobox";
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) =>
    s
      .find_orfs(1)
      .map((p) => {
        let seq = Seq.from_js(p[1]).convert("Protein").to_string();
        return `S${p[0].start} E${p[0].end} O${p[0].offset}: ${seq}`;
      })
      .join("\n");
</script>

<ListBoxItem value={tool}>Find ORFs</ListBoxItem>
