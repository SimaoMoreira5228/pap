<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import type { Leitor } from "$lib/types";
  import { writable } from "svelte/store";
  import Icon from "@iconify/svelte";

  export let action: "create" | "update" = "create";
  export let id: string | null = null;
  export let updateReaders: () => Promise<void> = async () => {};

  let isLoading = writable(false);

  let leitorNome = "";
  let leitorMorada = "";
  let leitorTelefone = "";
  let leitorEmail = "";

  async function createLeitor() {
    if (action !== "create") return;

    try {
      if (!leitorNome || !leitorMorada || !leitorTelefone || !leitorEmail) {
        return toast.error("Preencha todos os campos");
      }

      await call("create_reader", {
        name: leitorNome,
        address: leitorMorada,
        phone: leitorTelefone,
        email: leitorEmail,
      });

      toast.success("Leitor criado com sucesso");

      updateReaders();
    } catch (error) {
      toast.error(error as string);
    } finally {
      leitorNome = "";
      leitorMorada = "";
      leitorTelefone = "";
      leitorEmail = "";
    }
  }

  async function updateLeitor() {
    if (action !== "update") return;

    try {
      await call("update_reader", {
        id,
        name: leitorNome,
        address: leitorMorada,
        phone: leitorTelefone,
        email: leitorEmail,
      });

      toast.success("Leitor atualizado com sucesso");

      updateReaders();
    } catch (error) {
      toast.error(error as string);
    } finally {
      leitorNome = "";
      leitorMorada = "";
      leitorTelefone = "";
      leitorEmail = "";
    }
  }

  async function handleSubmit() {
    if (action === "create") {
      if (!leitorNome || !leitorMorada || !leitorTelefone || !leitorEmail) {
        return toast.error("Preencha todos os campos");
      }

      await createLeitor();
    } else if (action === "update") {
      if (!leitorNome || !leitorMorada || !leitorTelefone || !leitorEmail) {
        return toast.error("Preencha todos os campos");
      }

      await updateLeitor();
    }
  }

  async function getInCaseOfUpdate() {
    if (action === "update") {
      try {
        isLoading.set(true);

        const reader = await call<Leitor>("get_reader_by_id", {
          id,
        });

        leitorNome = reader.nome;
        leitorMorada = reader.morada ? reader.morada : "";
        leitorTelefone = reader.telefone ? reader.telefone.toString() : "";
        leitorEmail = reader.email ? reader.email : "";
      } catch (error) {
        console.error(error);
        toast.error(error as string);
      } finally {
        isLoading.set(false);
      }
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger class="cursor-pointer">
    <button on:click={async () => await getInCaseOfUpdate()}>
      <slot name="trigger" />
    </button>
  </Dialog.Trigger>
  {#if $isLoading}
    <div class="flex justify-center items-center w-full h-full">
      <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
    </div>
  {:else}
    <Dialog.Content class="sm:max-w-[425px]">
      <Dialog.Header>
        <Dialog.Title>
          {action === "create" ? "Criar Leitor" : "Atualizar Leitor"}
        </Dialog.Title>
      </Dialog.Header>
      <div class="flex flex-col gap-4 py-4">
        <div class="flex flex-col gap-2">
          <Label for="nome">Nome</Label>
          <Input
            type="text"
            id="nome"
            name="nome"
            class="border border-muted rounded-lg"
            bind:value={leitorNome}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="morada">Morada</Label>
          <Input
            type="text"
            id="morada"
            name="morada"
            class="border border-muted rounded-lg"
            bind:value={leitorMorada}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="telefone">Telefone</Label>
          <Input
            type="text"
            id="telefone"
            name="telefone"
            class="border border-muted rounded-lg"
            bind:value={leitorTelefone}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="email">Email</Label>
          <Input
            type="email"
            id="email"
            name="email"
            class="border border-muted rounded-lg"
            bind:value={leitorEmail}
          />
        </div>
      </div>

      <Dialog.Footer>
        <Dialog.Close>
          <Button type="submit" on:click={handleSubmit}>
            {action === "create" ? "Criar Leitor" : "Atualizar Leitor"}
          </Button>
        </Dialog.Close>
      </Dialog.Footer>
    </Dialog.Content>
  {/if}
</Dialog.Root>
