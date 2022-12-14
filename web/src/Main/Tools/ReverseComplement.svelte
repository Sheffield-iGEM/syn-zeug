<script lang="ts">
  import { ListBoxItem } from "@skeletonlabs/skeleton";

  let tool = (s) => s.reverse_complement().to_string();
</script>

<ListBoxItem value={tool}>Reverse Complement</ListBoxItem>
