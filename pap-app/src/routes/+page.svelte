<script lang="ts">
  import { goto } from "$app/navigation";
  import { dbStringStore, jwtStore } from "$lib/stores";
  import { toast } from "svelte-sonner";

  try {
    if (dbStringStore.get() === "") {
      toast.warning("Conexão á base de dados não configurada.");
    }
  } catch (error) {
    toast.error(error as string);
  }

  if (jwtStore.get() === "") {
    goto("/login");
  } else {
    goto("/books");
  }
</script>
