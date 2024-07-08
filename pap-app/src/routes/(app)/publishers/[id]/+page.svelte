<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { call } from "$lib/call";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";
  import NewPublisherDialog from "$lib/components/custom/NewPublisherDialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import { H3, H2, P } from "$lib/components/ui/typography";
  import type { Editora, Livro } from "$lib/types";
  import { hasPermission } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let params = $page.params;
  let publisher: Editora;
  let books: Livro[];
  let isLoading = writable(true);

  let hasUpdatePublisherPermission = false;
  let hasDeletePublisherPermission = false;

  async function loadPublisher() {
    try {
      publisher = await call("get_publisher_by_id", {
        id: parseInt(params.id),
      });

      books = await call("get_books_by_publisher_id", {
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

      await loadPublisher();

      hasUpdatePublisherPermission = await hasPermission("atualizar_editora");
      hasDeletePublisherPermission = await hasPermission("apagar_editora");

    } catch (error) {
      console.error(error);
      toast.error(error as string);
    } finally {
    isLoading.set(false);
    }
  });

  async function deletePublisher() {
    try {
      isLoading.set(true);

      await call("delete_publisher", {
        id: parseInt(params.id),
      });

      toast.success("Autor excluído com sucesso");

      goto("/publishers");
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
        <H2>{publisher.nome}</H2>
        {#if hasDeletePublisherPermission}
          <Button variant="destructive" size="icon" on:click={deletePublisher}>
            <Icon
              icon="material-symbols-light:delete-outline-rounded"
              class="w-8 h-8 text-secondary-muted"
            />
          </Button>
        {/if}
        {#if hasUpdatePublisherPermission}
          <NewPublisherDialog
            updatePublishers={loadPublisher}
            id={publisher.id.toString()}
            action="update"
          >
            <Button slot="trigger" variant="outline" size="icon">
              <Icon
                icon="material-symbols-light:ink-pen-outline"
                class="w-8 h-8 text-secondary-muted"
              />
            </Button>
          </NewPublisherDialog>
        {/if}
      </div>
      <div class="flex flex-col gap-0">
        {#if publisher.morada}
          <P class="!mt-1"
            >Morada: {publisher.morada} - {publisher.codigo_postal}</P
          >
        {/if}
        {#if publisher.email}
          <P class="!mt-1">Email: {publisher.email}</P>
        {/if}

        {#if publisher.telefone}
          <P class="!mt-1">Telefone: {publisher.telefone}</P>
        {/if}
      </div>
      <div class="flex flex-col w-full h-full">
        <H3 class="!mt-2">Livros do autor</H3>
        <BooksDisplay {books} />
      </div>
    </div>
  {/if}
</div>
