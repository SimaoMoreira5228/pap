<script lang="ts">
  import { call } from "$lib/call";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Select from "$lib/components/ui/select";
  import type { bibliotecario, Cargo } from "$lib/types";
  import { writable } from "svelte/store";
  import { Input } from "../ui/input";
  import { Label } from "../ui/label";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";

  type treatedRole = { nome: string; permissoes: number[] };

  export let updateLibrarians = async () => {};
  export let action: "create" | "update";
  export let id: number | undefined = undefined;
  let nonTreatedRoles = writable<Cargo[]>([]);
  let treatedRoles: treatedRole[] = [];

  let name: string | undefined = undefined;
  let password: string | undefined = undefined;
  let role: string | undefined = undefined;

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

  onMount(async () => {
    nonTreatedRoles.set(await call<Cargo[]>("get_roles"));
  });

  async function create() {
    try {
      if (!name || !password) {
        throw new Error("Nome ou password não preenchidos");
      }

      await call("new_librarian", {
        name,
        password,
        role,
      });
    } catch (error) {
      console.error(error);
    } finally {
      name = "";
      password = "";
      role = "";
      await updateLibrarians();
    }
  }

  async function update() {
    try {
      await call("update_librarian", {
        id,
        name,
        password,
        role,
      });
    } catch (error) {
      console.error(error);
    } finally {
      name = "";
      password = "";
      role = "";
      await updateLibrarians();
    }
  }

  async function handleSubmit() {
    if (action === "create") {
      await create();
    } else {
      await update();
    }
  }

  async function getInCaseOfUpdate() {
    if (action === "update") {
      try {
        const librarian = await call<bibliotecario>("get_librarian_by_id", {
          id,
        });

        name = librarian.nome;
        role = librarian.cargo;
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
        >{action === "create"
          ? "Criar Bibliotecário"
          : "Atualizar Bibliotecário"}</Dialog.Title
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
            bind:value={name}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="password">Password</Label>
          <Input
            type="password"
            id="password"
            name="password"
            class="border border-muted rounded-lg"
            bind:value={password}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="role">Cargo</Label>
          <Select.Root
            name="role"
            onSelectedChange={(value) => {
              role = value ? String(value.value) : undefined;
            }}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="Cargo" />
            </Select.Trigger>
            <Select.Content>
              {#each treatedRoles as role}
                <Select.Item value={role.nome}>
                  {role.nome}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
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
