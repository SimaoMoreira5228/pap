<script lang="ts">
  import { call } from "$lib/call";
  import NewLibrarianDialog from "$lib/components/custom/NewLibrarianDialog.svelte";
  import NewRoleDialog from "$lib/components/custom/NewRoleDialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import { H3 } from "$lib/components/ui/typography";
  import type { bibliotecario, Cargo } from "$lib/types";
  import { hasPermission } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  type treatedRole = { nome: string; permissoes: number[] };

  $: librarians = [] as bibliotecario[];
  let nonTreatedRoles = writable<Cargo[]>([]);
  $: treatedRoles = [] as treatedRole[];

  nonTreatedRoles.subscribe((roles) => {
    treatedRoles = [];

    for (let role of roles) {
      const exists = treatedRoles.find(
        (treatedRole) => treatedRole.nome === role.nome
      );

      if (exists) {
        exists.permissoes.push(role.permissao);
      } else {
        treatedRoles.push({ nome: role.nome, permissoes: [role.permissao] });
      }
    }
  });

  let hasCreateLibrarianPermission = false;
  let hasUpdateLibrarianPermission = false;
  let hasDeleteLibrarianPermission = false;

  let hasCreateRolePermission = false;
  let hasUpdateRolePermission = false;
  let hasDeleteRolePermission = false;

  let isLibrarianDeleteLoading = writable(false);
  let isRoleDeleteLoading = writable(false);

  async function load() {
    try {
      librarians = await call("get_librarians");
      nonTreatedRoles.set(await call("get_roles"));
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    }
  }

  onMount(async () => {
    try {
      await load();

      hasCreateLibrarianPermission = await hasPermission("criar_bibliotecario");
      hasUpdateLibrarianPermission = await hasPermission(
        "atualizar_bibliotecario"
      );
      hasDeleteLibrarianPermission = await hasPermission(
        "apagar_bibliotecario"
      );

      hasCreateRolePermission = await hasPermission("criar_cargo");
      hasUpdateRolePermission = await hasPermission("atualizar_cargo");
      hasDeleteRolePermission = await hasPermission("apagar_cargo");
    } catch (error) {
      console.error(error);
    }
  });

  async function deleteLibrarian(librarian: bibliotecario) {
    try {
      isLibrarianDeleteLoading.set(true);

      await call("delete_librarian", {
        id: librarian.id,
      });

      toast.success("Bibliotecário excluído com sucesso");

      await load();
    } catch (error) {
      toast.error(error as string);
    } finally {
      isLibrarianDeleteLoading.set(false);
    }
  }

  async function deleteRole(role: treatedRole) {
    try {
      isRoleDeleteLoading.set(true);

      await call("delete_role", {
        role: role.nome,
      });

      toast.success("Cargo excluído com sucesso");

      await load();
    } catch (error) {
      toast.error(error as string);
    } finally {
      isRoleDeleteLoading.set(false);
    }
  }
</script>

<div class="grid grid-cols-1 md:grid-cols-2 gap-4 overflow-auto w-full h-full">
  <div class="flex flex-col gap-2 overflow-auto">
    <H3>Bibliotecários</H3>
    <div class="gap-2 rounded-lg border overflow-auto p-2 w-full h-full">
      {#each librarians as librarian}
        <div
          class="flex flex-row items-center justify-between rounded-lg border gap-4 p-4"
        >
          <div class="flex flex-col">
            <p class="text-xl font-bold">{librarian.nome}</p>
            <p class="text-xs text-muted-foreground">{librarian.cargo}</p>
          </div>
          <div class="flex flex-row gap-2">
            {#if hasUpdateLibrarianPermission}
              <NewLibrarianDialog
                action="update"
                id={librarian.id}
                updateLibrarians={async () => await load()}
              >
                <Button variant="outline" size="icon" slot="trigger">
                  <Icon
                    icon="material-symbols-light:ink-pen-outline"
                    class="w-6 h-6 text-secondary-muted"
                  />
                </Button>
              </NewLibrarianDialog>
            {/if}
            {#if hasDeleteLibrarianPermission}
              {#if $isLibrarianDeleteLoading}
                <div class="flex justify-center items-center w-full h-full">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-8 h-8 text-primary"
                  />
                </div>
              {:else}
                <Button
                  variant="destructive"
                  size="icon"
                  on:click={() => {
                    deleteLibrarian(librarian);
                  }}
                >
                  <Icon
                    icon="material-symbols-light:delete-outline-rounded"
                    class="w-6 h-6 text-secondary-muted"
                  />
                </Button>
              {/if}
            {/if}
          </div>
        </div>
      {/each}
    </div>
    {#if hasCreateLibrarianPermission}
      <NewLibrarianDialog
        action="create"
        updateLibrarians={async () => await load()}
      >
        <Button slot="trigger" class="w-full">Adicionar Bibliotecário</Button>
      </NewLibrarianDialog>
    {/if}
  </div>
  <div class="flex flex-col gap-2 overflow-auto">
    <H3>Cargos</H3>
    <div class="gap-2 rounded-lg border overflow-auto p-2 w-full h-full">
      {#each treatedRoles as role}
        <div
          class="flex flex-row items-center justify-between gap-4 rounded-lg border p-4"
        >
          <div class="flex flex-col">
            <p class="text-xl font-bold">{role.nome}</p>
            <p class="text-xs text-muted-foreground line-clamp-1">
              {role.permissoes.join(", ")}
            </p>
          </div>
          <div class="flex flex-row gap-2">
            {#if hasUpdateRolePermission}
              <NewRoleDialog
                action="update"
                roleName={role.nome}
                updateRoles={async () => await load()}
              >
                <Button variant="outline" size="icon" slot="trigger">
                  <Icon
                    icon="material-symbols-light:ink-pen-outline"
                    class="w-6 h-6 text-secondary-muted"
                  />
                </Button>
              </NewRoleDialog>
            {/if}
            {#if hasDeleteRolePermission}
              {#if $isRoleDeleteLoading}
                <div class="flex justify-center items-center w-full h-full">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-8 h-8 text-primary"
                  />
                </div>
              {:else}
                <Button
                  variant="destructive"
                  size="icon"
                  on:click={() => {
                    deleteRole(role);
                  }}
                >
                  <Icon
                    icon="material-symbols-light:delete-outline-rounded"
                    class="w-6 h-6 text-secondary-muted"
                  />
                </Button>
              {/if}
            {/if}
          </div>
        </div>
      {/each}
    </div>
    {#if hasCreateRolePermission}
      <NewRoleDialog action="create" updateRoles={async () => await load()}>
        <Button class="w-full" slot="trigger">Adicionar Cargo</Button>
      </NewRoleDialog>
    {/if}
  </div>
</div>
