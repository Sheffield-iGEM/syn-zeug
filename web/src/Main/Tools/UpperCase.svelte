<script lang="ts">
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) => s.normalize_case("Upper").to_string();
</script>

<ListBoxItem value={tool}>Convert to Upper Case</ListBoxItem>
