<script lang="ts">
  import { P } from "$lib/components/ui/typography";
  import type { Livro, Requisicao } from "$lib/types";
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { loanPeriodStore } from "$lib/stores";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import { onMount } from "svelte";

  export let bookId: number;
  export let updateBook: () => Promise<void>;

  let requestedBook: [Livro, Requisicao];

  async function getRequestedBook() {
    try {
      requestedBook = await call("get_requested_book_by_book_id", {
        bookId,
      });
    } catch (error) {
      toast.error(error as string);
      console.error(error);
    }
  }

  onMount(async () => {
    await getRequestedBook();
  });

  function DaysBetween(StartDate: string, EndDate: string) {
    const startDate = new Date(StartDate);
    const endDate = new Date(EndDate);

    const oneDay = 1000 * 60 * 60 * 24;

    const start = Date.UTC(
      endDate.getFullYear(),
      endDate.getMonth(),
      endDate.getDate()
    );
    const end = Date.UTC(
      startDate.getFullYear(),
      startDate.getMonth(),
      startDate.getDate()
    );

    return (start - end) / oneDay;
  }

  async function confirmReturn() {
    try {
      await call("return_book", {
        bookId,
      });

      toast.success("Livro devolvido com sucesso");
      await updateBook();
    } catch (error) {
      toast.error(error as string);
    }
  }
</script>

<Dialog.Root>
  <Dialog.Trigger class={buttonVariants({ variant: "secondary" })}>
    Confirmar Devolução
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Confirmar Devolução</Dialog.Title>
    </Dialog.Header>
    <div class="flex flex-col gap-4 py-4">
      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-[2px]">
          <P
            >Este livro foi requisitado em {new Date(
              requestedBook[1].data_requisicao
            ).toLocaleDateString()}</P
          >
          {#if DaysBetween(requestedBook[1].data_requisicao, new Date().toISOString()) > loanPeriodStore.get()}
            <P>
              Este livro está atrasado em {DaysBetween(
                requestedBook[1].data_requisicao,
                new Date().toISOString()
              )} dias
            </P>
          {:else}
            <P>
              Este livro está em empréstimo há {DaysBetween(
                requestedBook[1].data_requisicao,
                new Date().toISOString()
              )} dia(s)
            </P>
          {/if}
        </div>
      </div>
      <Dialog.Footer>
        <div class="flex felx-row justify-end items-center gap-2">
          <P>Confirmar a devolução?</P>
          <Dialog.Close>
            <Button type="submit" on:click={confirmReturn}>Confirmar</Button>
          </Dialog.Close>
        </div>
      </Dialog.Footer>
    </div>
  </Dialog.Content>
</Dialog.Root>
