<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import type { Editora } from "$lib/types";
  import { writable } from "svelte/store";

  let name = "";
  let address = "";
  let postalCode = "";
  let phone = "";
  let email = "";

  export let updatePublishers: () => Promise<void> = async () => {};

  let isLoading = writable(false);

  export let action: "create" | "update";
  export let id: number | undefined = undefined;

  async function createPublisher() {
    if (action !== "create") return;

    try {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await call("create_publisher", {
        name,
        address,
        postalCode,
        phone,
        email,
      });

      toast.success("Editora criada com sucesso");

      updatePublishers();
    } catch (error) {
      toast.error(error as string);
    } finally {
      name = "";
      address = "";
      postalCode = "";
      phone = "";
      email = "";
    }
  }

  async function updatePublisher() {
    if (action !== "update") return;

    try {
      await call("update_publisher", {
        id,
        name,
        address,
        postalCode,
        phone,
        email,
      });

      toast.success("Editora atualizado com sucesso");

      updatePublishers();
    } catch (error) {
      toast.error(error as string);
    } finally {
      name = "";
      address = "";
      postalCode = "";
      phone = "";
      email = "";
    }
  }

  async function handleSubmit() {
    if (action === "create") {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await createPublisher();
    } else if (action === "update") {
      if (!name) {
        return toast.error("O nome é obrigatório");
      }

      await updatePublisher();
    }
  }

  async function getInCaseOfUpdate() {
    if (action === "update") {
      try {
        isLoading.set(true);

        const publisher = await call<Editora>("get_publisher_by_id", {
          id,
        });

        name = publisher.nome;
        address = publisher.morada ? publisher.morada : "";
        postalCode = publisher.codigo_postal ? publisher.codigo_postal : "";
        phone = publisher.telefone ? publisher.telefone : "";
        email = publisher.email ? publisher.email : "";
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

  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title
        >{action === "create"
          ? "Criar Editora"
          : "Atualizar Editora"}</Dialog.Title
      >
    </Dialog.Header>
    <div class="flex flex-col gap-4 py-4">
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
        <Label for="morada">Morada</Label>
        <Input
          type="text"
          id="morada"
          name="morada"
          class="border border-muted rounded-lg"
          bind:value={address}
        />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="telefone">codigoPostal</Label>
        <Input
          type="text"
          id="codigoPostal"
          name="codigoPostal"
          class="border border-muted rounded-lg"
          bind:value={postalCode}
        />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="telefone">Telefone</Label>
        <Input
          type="text"
          id="telefone"
          name="telefone"
          class="border border-muted rounded-lg"
          bind:value={phone}
        />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="email">Email</Label>
        <Input
          type="email"
          id="email"
          name="email"
          class="border border-muted rounded-lg"
          bind:value={email}
        />
      </div>
    </div>
    <Dialog.Footer>
      <Dialog.Close>
        <Button type="submit" on:click={handleSubmit}>
          {action === "create" ? "Criar Editora" : "Atualizar Editora"}
        </Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
