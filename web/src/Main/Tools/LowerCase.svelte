<script lang="ts">
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) => s.normalize_case("Lower").to_string()
</script>

<ListBoxItem value={tool}>Convert to Lower Case</ListBoxItem>
