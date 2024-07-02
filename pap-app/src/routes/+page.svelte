<script lang="ts">
  import { goto } from "$app/navigation";
  import { dbStringStore, jwtStore } from "$lib/stores";
  import { toast } from "svelte-sonner";

  if (dbStringStore.get() === "") {
    toast.error("Falha ao carregar configurações do banco de dados");
  }

  if (jwtStore.get() === "") {
    goto("/login", { invalidateAll: true });
  } else {
    goto("/books", { invalidateAll: true });
  }
</script>
