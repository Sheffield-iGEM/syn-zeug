<script lang="ts">
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) => s.convert("Rna").to_string()
</script>

<ListBoxItem value={tool}>Convert to RNA</ListBoxItem>
