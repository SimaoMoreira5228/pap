<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import { Input } from "../ui/input";
  import { Label } from "../ui/label";

  export let updateCategories: () => Promise<void>;
  export let categoryId: number;

  let name: string;

  async function createSubCategory() {
    try {
      await call("create_sub_category", {
        categoryId,
        name,
      });

      toast.success("Categoria criada com sucesso");
      await updateCategories();
    } catch (error) {
      toast.error(error as string);
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger>
    <slot name="trigger" />
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Criar nova sub-categoria</Dialog.Title>
    </Dialog.Header>
    <div class="flex flex-col gap-4 py-4">
      <div class="flex flex-col gap-4">
        <Label for="name">Nome da sub-categoria</Label>
        <Input
          name="name"
          bind:value={name}
          placeholder="Nome da sub-categoria"
          type="text"
          required
        />
      </div>
      <Dialog.Footer>
        <div class="flex felx-row justify-end items-center gap-2">
          <Dialog.Close>
            <Button type="submit" on:click={createSubCategory}>Confirmar</Button
            >
          </Dialog.Close>
        </div>
      </Dialog.Footer>
    </div>
  </Dialog.Content>
</Dialog.Root>
