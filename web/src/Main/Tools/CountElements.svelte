<script lang="ts">
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) =>
    [...s.count_elements()]
      .sort()
      .map((pair) => pair.join(": "))
      .join("\n");
</script>

<ListBoxItem value={tool}>Count Elements</ListBoxItem>
