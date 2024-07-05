<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";

  let leitorNome = "";
  let leitorMorada = "";
  let leitorTelefone = "";
  let leitorEmail = "";

  async function createLeitor() {
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
    } catch (error) {
      toast.error(error as string);
    } finally {
      leitorNome = "";
      leitorMorada = "";
      leitorTelefone = "";
      leitorEmail = "";
    }
  }
</script>

<Dialog.Root>
  <div>
    se este for um novo leitor, podes
    <Dialog.Trigger class="text-primary cursor-pointer">cria-lo</Dialog.Trigger>
  </div>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Criar Leitor</Dialog.Title>
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
        <Button type="submit" on:click={createLeitor}>Criar Leitor</Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
