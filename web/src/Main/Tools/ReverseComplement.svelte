<script lang="ts">
  import { ListBoxItem } from "@brainandbones/skeleton";

  let tool = (s) => s.reverse_complement().to_string()
</script>

<ListBoxItem value={tool}>Reverse Complement</ListBoxItem>
