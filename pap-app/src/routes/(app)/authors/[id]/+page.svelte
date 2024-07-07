<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { call } from "$lib/call";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";
  import NewAuthorDialog from "$lib/components/custom/NewAuthorDialog.svelte";
  import { Button } from "$lib/components/ui/button";
  import { H3, H2, P } from "$lib/components/ui/typography";
  import type { Autor, Livro } from "$lib/types";
  import { hasPermission } from "$lib/utils";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let params = $page.params;
  let author: Autor;
  let books: Livro[];
  let isLoading = writable(true);

  let hasUpdateAuthorPermission = false;
  let hasDeleteAuthorPermission = false;

  async function loadAuthor() {
    try {
      author = await call("get_author_by_id", {
        id: parseInt(params.id),
      });

      books = await call("get_books_by_author_id", {
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
      await loadAuthor();

      hasUpdateAuthorPermission = await hasPermission("atualizar_autor");
      hasDeleteAuthorPermission = await hasPermission("apagar_autor");

      isLoading.set(false);
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    }
  });

  async function deleteAuthor() {
    try {
      await call("delete_author", {
        id: parseInt(params.id),
      });

      toast.success("Autor excluído com sucesso");

      goto("/authors");
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
        <H2>{author.nome}</H2>
        {#if hasDeleteAuthorPermission}
          <Button variant="outline" size="icon" on:click={deleteAuthor}>
            <Icon
              icon="material-symbols-light:delete-outline-rounded"
              class="w-8 h-8 text-secondary-muted"
            />
          </Button>
        {/if}
        {#if hasUpdateAuthorPermission}
          <NewAuthorDialog
            updateAuthors={loadAuthor}
            id={author.id}
            action="update"
          >
            <Button slot="trigger" variant="outline" size="icon">
              <Icon
                icon="material-symbols-light:ink-pen-outline"
                class="w-8 h-8 text-secondary-muted"
              />
            </Button>
          </NewAuthorDialog>
        {/if}
      </div>
      <div class="flex flex-col gap-0">
        {#if author.nacionalidade}
          <P class="!mt-1">Nacionalidade: {author.nacionalidade}</P>
        {/if}
        {#if author.data_nasc}
          <P class="!mt-1">Nasceu em: {author.data_nasc}</P>
        {/if}

        {#if author.data_morte}
          <P class="!mt-1">Faleceu em: {author.data_morte}</P>
        {/if}
      </div>

      <div class="flex flex-col w-full h-full">
        <H3 class="!mt-2">Livros do autor</H3>
        <BooksDisplay {books} />
      </div>
    </div>
  {/if}
</div>
