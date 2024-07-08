<script lang="ts">
  import { call } from "$lib/call";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import type { Cargo, permissao } from "$lib/types";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { Label } from "../ui/label";
  import { Input } from "../ui/input";

  type Role = { name: string; permissions: number[] };

  export let updateRoles = async () => {};
  export let action: "create" | "update";
  export let roleName: string | undefined = undefined;
  let permissions: permissao[] = [];
  let fetchedRole: Role | undefined = undefined;

  let choosenPermissions: number[] = [];
  let newRoleName: string | undefined = undefined;

  onMount(async () => {
    permissions = await call<permissao[]>("get_permissions");
  });

  async function createRole() {
    if (action !== "create") return;

    try {
      if (!newRoleName) {
        return toast.error("O nome é obrigatório");
      }

      await call("create_role", {
        role: newRoleName,
        permissions: choosenPermissions,
      });

      toast.success("Cargo criado com sucesso");

      updateRoles();
    } catch (error) {
      toast.error(error as string);
    } finally {
      newRoleName = "";
      choosenPermissions = [];
    }
  }

  async function updateRole() {
    if (action !== "update") return;

    try {
      await call("update_role", {
        role: roleName,
        newRoleName,
        permissions: choosenPermissions,
      });

      toast.success("Cargo atualizado com sucesso");

      updateRoles();
    } catch (error) {
      toast.error(error as string);
    } finally {
      newRoleName = "";
      choosenPermissions = [];
    }
  }

  async function handleSubmit() {
    if (action === "create") {
      if (!newRoleName) {
        return toast.error("O nome é obrigatório");
      }

      await createRole();
    } else if (action === "update") {
      await updateRole();
    }
  }

  async function getInCaseOfUpdate() {
    if (action === "update") {
      newRoleName = roleName;

      try {
        fetchedRole = await call<Role>("get_role_by_name", {
          role: roleName,
        });

        choosenPermissions = fetchedRole.permissions;
        newRoleName = fetchedRole.name;

        console.log(fetchedRole);
      } catch (error) {
        console.error(error);
        toast.error(error as string);
      }
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger on:click={async () => await getInCaseOfUpdate()}>
    <slot name="trigger" />
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title
        >{action === "create" ? "Criar Cargo" : "Atualizar Cargo"}</Dialog.Title
      >
    </Dialog.Header>
    <div class="flex flex-col gap-4 py-4">
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <Label for="nome">Nome</Label>
          <Input
            type="text"
            id="nome"
            name="nome"
            class="border border-muted rounded-lg"
            bind:value={newRoleName}
          />
        </div>
        <div class="flex flex-col gap-4 overflow-auto">
          <Label>Permissões</Label>
          <div class="grid grid-cols-2 sm:grid-cols-3 gap-2 overflow-auto">
            {#each permissions as permission}
              <div class="flex flex-row items-center gap-2">
                <input
                  type="checkbox"
                  id={permission.id.toString()}
                  name={permission.acao}
                  bind:group={choosenPermissions}
                  value={permission.id}
                />
                <Label for={permission.id.toString()}>
                  {permission.label}
                </Label>
              </div>
            {/each}
          </div>
        </div>
        <Dialog.Footer>
          <Dialog.Close>
            <Button type="submit" on:click={handleSubmit}>Confirmar</Button>
          </Dialog.Close>
        </Dialog.Footer>
      </div>
    </div></Dialog.Content
  >
</Dialog.Root>
