<script lang="ts">
  import { ListBoxItem } from "@skeletonlabs/skeleton";

  let tool = (s) => s.convert("Protein").to_string();
</script>

<ListBoxItem value={tool}>Convert to Protein</ListBoxItem>
