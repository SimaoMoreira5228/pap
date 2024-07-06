<script lang="ts">
  import { goto } from "$app/navigation";
  import { jwtStore } from "$lib/stores";

  if (jwtStore.get() === "") goto("/login");
</script>

<slot />
