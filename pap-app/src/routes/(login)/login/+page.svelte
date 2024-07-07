<script lang="ts">
  import { goto } from "$app/navigation";
  import { call } from "$lib/call";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import * as Card from "$lib/components/ui/card";
  import { P, H3 } from "$lib/components/ui/typography/index";
  import { jwtStore } from "$lib/stores";
  import type { permissao } from "$lib/types";
  import Icon from "@iconify/svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  const isLoading = writable(false);
  const librariansExistence = call<boolean>("check_librarians_existence");
  const permissions = call<permissao[]>("get_permissions");

  let name = "";
  let password = "";
  let roleName = "";
  let choosenPermissions: number[] = [28];

  let error = "";

  function setError(message: string) {
    error = message;
    setTimeout(() => {
      error = "";
    }, 3000);
  }

  async function handleLoginSubmit(event: Event) {
    event.preventDefault();

    if (name === "" || password === "") {
      return setError("Preencha todos os campos");
    }

    try {
      isLoading.set(true);

      const token = await call<string>("login", { name, password });

      jwtStore.set(token);

      while (jwtStore.get() === "")
        await new Promise((r) => setTimeout(r, 100));
      goto("/books");
    } catch (error) {
      toast.error(error as string);
    } finally {
      isLoading.set(false);
    }
  }

  async function handleFirstAccessSubmit(event: Event) {
    event.preventDefault();

    if (name === "" || password === "" || roleName === "") {
      return setError("Preencha todos os campos");
    }

    if (choosenPermissions.length === 0) {
      return setError("Escolha pelo menos uma permissão");
    }

    try {
      isLoading.set(true);
      const role = await call("add_permission_to_role", {
        token: "first_librarian",
        role: roleName,
        permissions: choosenPermissions,
      });

      await call("new_librarian", {
        name,
        password,
        role: role,
        token: "first_librarian",
      });

      window.location.reload();
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    } finally {
      isLoading.set(false);
    }
  }
</script>

<div class="w-full h-full flex flex-col justify-center items-center">
  {#await librariansExistence}
    <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
  {:then exists}
    {#if exists}
      <Card.Root class="w-[65%] shadow-lg">
        <Card.Header>
          <Card.Title>Inciar Sessão</Card.Title>
        </Card.Header>
        <Card.Content>
          <form
            class="flex flex-col w-full mt-4 gap-6"
            on:submit={handleLoginSubmit}
          >
            <div>
              <Label for="name" class="py-2">Nome</Label>
              <Input type="name" id="name" name="name" bind:value={name} />
            </div>

            <div>
              <Label for="password" class="py-2">Password</Label>
              <Input
                type="password"
                id="password"
                name="password"
                bind:value={password}
              />
            </div>

            <div class="flex flex-col justify-center">
              {#if error}
                <P class="text-red-500">{error}</P>
              {/if}

              {#if $isLoading}
                <div class="flex justify-center items-center">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-8 h-8 text-primary"
                  />
                </div>
              {:else}
                <Button type="submit">Entrar</Button>
              {/if}
            </div>
          </form>
        </Card.Content>
      </Card.Root>
    {:else}
      <Card.Root class="w-[75%] shadow-lg overflow-auto">
        <Card.Header>
          <Card.Title>Primeiro acesso</Card.Title>
          <Card.Description>
            No primeiro acesso, é abrigatório a criação de um administrador
          </Card.Description>
        </Card.Header>
        <Card.Content class="overflow-auto">
          <form
            class="flex flex-col w-full mt-4 gap-6 h-[75%]"
            on:submit={handleFirstAccessSubmit}
          >
            <div>
              <Label for="name" class="py-2">Nome do Administrador</Label>
              <Input type="text" id="name" name="name" bind:value={name} />
            </div>

            <div>
              <Label for="password" class="py-2">Password</Label>
              <Input
                type="password"
                id="password"
                name="password"
                bind:value={password}
              />
            </div>

            <div>
              <Label for="roleName" class="py-2">Nome do Cargo</Label>
              <Input
                type="text"
                id="roleName"
                name="roleName"
                bind:value={roleName}
              />
            </div>

            <div class="flex flex-col overflow-auto">
              <Label for="permissions" class="py-2">Permissões</Label>
              {#await permissions}
                <div class="flex justify-center items-center w-full h-full">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-8 h-8 text-primary"
                  />
                </div>
              {:then permissions}
                <div
                  class="grid grid-cols-2 sm:grid-cols-3 gap-2 overflow-auto"
                >
                  {#each permissions as permission}
                    <div class="flex flex-row items-center gap-2">
                      <input
                        type="checkbox"
                        id={permission.id.toString()}
                        name={permission.acao}
                        bind:group={choosenPermissions}
                        value={permission.id}
                        disabled={permission.acao === "mudar_configuracoes"}
                        on:change={() => console.log(choosenPermissions)}
                      />
                      <Label for={permission.id.toString()}>
                        {permission.label}
                      </Label>
                    </div>
                  {/each}
                </div>
              {/await}
            </div>

            <div class="flex flex-col justify-center">
              {#if error}
                <P class="text-red-500">{error}</P>
              {/if}

              {#if $isLoading}
                <div class="flex justify-center items-center">
                  <Icon
                    icon="svg-spinners:270-ring-with-bg"
                    class="w-8 h-8 text-primary"
                  />
                </div>
              {:else}
                <Button type="submit">Criar</Button>
              {/if}
            </div>
          </form>
        </Card.Content>
      </Card.Root>
    {/if}
  {/await}
</div>
