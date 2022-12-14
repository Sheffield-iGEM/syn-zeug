<script lang="ts">
  import { ListBoxItem } from "@skeletonlabs/skeleton";

  let tool = (s) => s.convert("Dna").to_string();
</script>

<ListBoxItem value={tool}>Convert to DNA</ListBoxItem>
