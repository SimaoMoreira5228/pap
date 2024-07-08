<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { call } from "$lib/call";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";
  import NewPublisherDialog from "$lib/components/custom/NewPublisherDialog.svelte";
  import NewReaderDialog from "$lib/components/custom/NewReaderDialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import { H3, H2, P } from "$lib/components/ui/typography";
  import type { Leitor, Livro } from "$lib/types";
  import { hasPermission } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let params = $page.params;
  let reader: Leitor;
  let books: Livro[];
  let isLoading = writable(true);

  let hasUpdateReaderPermission = false;
  let hasDeleteReaderPermission = false;

  async function loadReaders() {
    try {
      reader = await call("get_reader_by_id", {
        id: parseInt(params.id),
      });

      books = await call("get_requested_books_by_reader_id", {
        id: parseInt(params.id),
      });
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    }
  }

  onMount(async () => {
    try {
      isLoading.set(true);

      await loadReaders();

      hasUpdateReaderPermission = await hasPermission("atualizar_leitor");
      hasDeleteReaderPermission = await hasPermission("apagar_leitor");
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    } finally {
      isLoading.set(false);
    }
  });

  async function deleteReader() {
    try {
      isLoading.set(true);

      await call("delete_reader", {
        id: parseInt(params.id),
      });

      toast.success("Leitor excluído com sucesso");

      goto("/readers");
    } catch (error) {
      toast.error(error as string);
    } finally {
      isLoading.set(false);
    }
  }
</script>

<div class="flex w-full h-full overflow-auto">
  {#if $isLoading}
    <div class="flex justify-center items-center w-full h-full">
      <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
    </div>
  {:else}
    <div class="flex flex-col w-full h-full">
      <div class="flex flex-row justify-start items-center gap-2">
        <H2>{reader.nome}</H2>
        {#if hasDeleteReaderPermission}
          <Button variant="destructive" size="icon" on:click={deleteReader}>
            <Icon
              icon="material-symbols-light:delete-outline-rounded"
              class="w-8 h-8 text-secondary-muted"
            />
          </Button>
        {/if}
        {#if hasUpdateReaderPermission}
          <NewReaderDialog
            updateReaders={loadReaders}
            id={reader.id}
            action="update"
          >
            <Button slot="trigger" variant="outline" size="icon">
              <Icon
                icon="material-symbols-light:ink-pen-outline"
                class="w-8 h-8 text-secondary-muted"
              />
            </Button>
          </NewReaderDialog>
        {/if}
      </div>
      <div class="flex flex-col gap-0">
        {#if reader.morada}
          <P class="!mt-1">Morada: {reader.morada}</P>
        {/if}
        {#if reader.email}
          <P class="!mt-1">Email: {reader.email}</P>
        {/if}

        {#if reader.telefone}
          <P class="!mt-1">Telefone: {reader.telefone}</P>
        {/if}
      </div>
      <div class="flex flex-col w-full h-full">
        <H3 class="!mt-2">Livros Requisitados</H3>
        <BooksDisplay {books} />
      </div>
    </div>
  {/if}
</div>
