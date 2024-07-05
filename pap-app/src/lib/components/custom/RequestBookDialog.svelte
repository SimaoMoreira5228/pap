<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import * as Select from "$lib/components/ui/select";
  import type { Leitor, Livro } from "$lib/types";
  import { writable } from "svelte/store";

  export let book: Livro;
  export let updateBook: () => Promise<void>;
  let leitores = writable<Leitor[]>([]);
  let selectedLeitor: any;

  async function findLeitor(event: any) {
    try {
      const value = event.target.value;

      if (parseInt(value)) {
        async function get_leitor() {
          const leitor = await call<Leitor | undefined>("get_reader_by_id", {
            id: parseInt(value),
          });

          return leitor ? [leitor] : [];
        }

        leitores.set(await get_leitor());
      } else {
        leitores.set(
          (await call<Leitor[] | undefined>("get_readers_by_name", {
            name: value,
          })) ?? []
        );
      }
    } catch (error) {
      toast.error(error as string);
    }
  }

  async function requisitar() {
    try {
      if (!selectedLeitor) {
        return toast.error("Selecione um leitor");
      }

      await call("request_book", {
        bookId: book.id,
        readerId: selectedLeitor.value,
      });

      toast.success("Livro requisitado com sucesso");
      await updateBook();
    } catch (error) {
      toast.error(error as string);
    } finally {
      selectedLeitor = null;
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger class={buttonVariants({ variant: "default" })}>
    Requisitar
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Requisitar</Dialog.Title>
      <Dialog.Description>
        Preencha os campos abaixo para requisitar o livro ao leitor
      </Dialog.Description>
    </Dialog.Header>
    <div class="flex flex-col gap-4 py-4">
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <Label for="nleitor-nome">Número de leitor / Nome</Label>
          <Input
            type="text"
            id="nleitor-nome"
            name="nleitor-nome"
            class="border border-muted rounded-lg"
            on:change={(event) => findLeitor(event)}
          />
        </div>

        <Select.Root
          onSelectedChange={(value) => {
            selectedLeitor = value;
          }}
        >
          <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Leitor" />
          </Select.Trigger>
          <Select.Content>
            {#each $leitores as leitor}
              <Select.Item value={leitor.id}>
                {leitor.id} - {leitor.nome}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>

      <slot />
    </div>
    <Dialog.Footer>
      <Dialog.Close>
        <Button type="submit" on:click={requisitar}>Requisitar</Button
        >
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>