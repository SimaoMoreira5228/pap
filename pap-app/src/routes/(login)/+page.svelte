<script lang="ts">
  import { goto } from "$app/navigation";
  import { dbStringStore, jwtStore } from "$lib/stores";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  onMount(() => {
    if (dbStringStore.get().dbUrl === "") {
      toast.warning("Conexão á base de dados não configurada.");
      return goto("/setup");
    }

    if (jwtStore.get() === "") {
      return goto("/login");
    } else {
      return goto("/books");
    }
  });
</script>
