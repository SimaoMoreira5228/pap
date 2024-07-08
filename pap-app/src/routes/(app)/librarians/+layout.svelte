<script lang="ts">
  import { goto } from "$app/navigation";
  import { jwtStore } from "$lib/stores";
  import { hasLibrariansPageAccess, hasPermission } from "$lib/utils";
  import { onMount } from "svelte";

  if (jwtStore.get() === "") goto("/login");

  onMount(async () => {
    if (jwtStore.get() === "") return;

    if (!(await hasLibrariansPageAccess())) {
      goto("/books");
    }
  });
</script>

<slot />
