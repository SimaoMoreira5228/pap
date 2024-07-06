<script lang="ts">
  import { goto } from "$app/navigation";
  import { call } from "$lib/call";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { dbStringStore, jwtStore, loanPeriodStore } from "$lib/stores";
  import type { permissao } from "$lib/types";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  let dbUrl = dbStringStore.get().dbUrl;
  let loanPeriod = loanPeriodStore.get();

  onMount(async () => {
    try {
      if (await call<boolean>("check_librarians_existence")) {
        if (jwtStore.get() !== "") {
          const has_permission = await call<boolean>(
            "does_librarian_has_permission_by_acao",
            {
              acao: "mudar_configuracoes",
            }
          );

          if (!has_permission) {
            return goto("/login");
          }
        }
      }
    } catch (error) {
      toast.error(error as string);
    }
  });
</script>

<div class="flex justify-start items-start flex-col gap-6 w-full h-full">
  <form class="flex flex-col gap-2 w-full">
    <Label for="dbUrl">URL da base de dados</Label>
    <div class="flex flex-row gap-2 w-full h-full">
      <Input
        id="dbUrl"
        type="text"
        placeholder="Database URL"
        bind:value={dbUrl}
        class="w-[30%]"
      />
      <Button
        type="submit"
        on:click={(event) => {
          event.preventDefault();
          dbStringStore.set(dbUrl);
        }}
      >
        Guardar
      </Button>
    </div>
  </form>
  <form class="flex flex-col gap-2 w-full">
    <Label for="loanPeriod">Período de empréstimo</Label>
    <div class="flex flex-row gap-2 w-full h-full">
      <Input
        id="loanPeriod"
        type="number"
        placeholder="Loan Period"
        bind:value={loanPeriod}
        class="w-[30%]"
      />
      <Button
        type="submit"
        on:click={(event) => {
          event.preventDefault();
          loanPeriodStore.set(loanPeriod);
        }}
      >
        Guardar
      </Button>
    </div>
  </form>
</div>
