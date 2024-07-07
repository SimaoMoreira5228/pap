<script lang="ts">
  import { call } from "$lib/call";
  import { toast } from "svelte-sonner";
  import { H3, P } from "$lib/components/ui/typography";
  import type { Livro } from "$lib/types";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import Icon from "@iconify/svelte";
  import RequestBookDialog from "$lib/components/custom/RequestBookDialog.svelte";
  import NewReaderDialog from "$lib/components/custom/NewReaderDialog.svelte";
  import BookReturnDialog from "$lib/components/custom/BookReturnDialog.svelte";
  import NewBookDialog from "$lib/components/custom/NewBookDialog.svelte";
  import { hasPermission } from "$lib/utils";
  import { Button } from "$lib/components/ui/button";
  import { goto } from "$app/navigation";

  let params = $page.params;
  let isLoading = writable(true);
  let book: Livro;

  let hasUpdateBookPermission = false;
  let hasDeleteBookPermission = false;
  let hasCreateReaderPermission = false;
  let hasCreateRequestPermission = false;
  let hasUpdateRequestPermission = false;

  const getBook = async () => {
    try {
      isLoading.set(true);
      book = await call("get_book_by_id", {
        id: parseInt(params.id),
      });
    } catch (error) {
      toast.error(error as string);
      console.error(error);
    } finally {
      isLoading.set(false);
    }
  };

  onMount(async () => {
    await getBook();

    hasUpdateBookPermission = await hasPermission("atualizar_livro");
    hasDeleteBookPermission = await hasPermission("apagar_livro");
    hasCreateReaderPermission = await hasPermission("criar_leitor");
    hasCreateRequestPermission = await hasPermission("criar_requisicao");
    hasUpdateRequestPermission = await hasPermission("atualizar_requisicao");
  });

  async function deleteBook() {
    try {
      await call("delete_book", {
        id: parseInt(params.id),
      });

      toast.success("Livro apagado com sucesso");

      goto("/books");
    } catch (error) {
      toast.error(error as string);
    } finally {
      isLoading.set(false);
    }
  }
</script>

{#if $isLoading}
  <div class="flex justify-center items-center w-full h-full">
    <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
  </div>
{:else}
  <div
    class="grid md:grid-cols-2 gaP-6 lg:gaP-12 items-start max-w-6xl Px-4 mx-auto Py-6"
  >
    <div class="flex flex-col gaP-4">
      <img
        src={book.img_url}
        alt="Book Cover"
        class="asPect-[2/3] object-cover border rounded-lg overflow-hidden w-[75%] h-[80%]"
      />
    </div>
    <div class="grid gaP-4 md:gaP-8">
      <div>
        <H3 class="text-3xl font-bold">{book.nome}</H3>
        <a href={`/authors/${book.autor_id}`}>
          <P class="text-muted-foreground">{book.autor}</P>
        </a>
      </div>
      <div>
        <P class="text-muted-foreground">{book.ano_edicao}</P>
      </div>
      <div>
        <P class="line-clamp-5">{book.resumo}</P>
      </div>
      <div class="mt-8 flex flex-row gap-2">
        {#if book.requisitado}
          {#if hasUpdateRequestPermission}
            <BookReturnDialog bookId={book.id} updateBook={getBook} />
          {/if}
        {:else if !book.requisitado && hasCreateRequestPermission}
          <RequestBookDialog {book} updateBook={getBook}>
            {#if hasCreateReaderPermission}
              <div class="flex flex-row gap-2">
                se este for um novo leitor, podes
                <NewReaderDialog action="create">
                  <p slot="trigger" class="text-primary cursor-pointer">
                    cria-lo
                  </p>
                </NewReaderDialog>
              </div>
            {/if}
          </RequestBookDialog>
        {/if}
        {#if hasDeleteBookPermission}
          <Button variant="outline" size="icon" on:click={deleteBook}>
            <Icon
              icon="material-symbols-light:delete-outline-rounded"
              class="w-8 h-8 text-secondary-muted"
            />
          </Button>
        {/if}
        {#if hasUpdateBookPermission}
          <NewBookDialog action="update" bookId={book.id} updateBooks={getBook}>
            <Button variant="outline" size="icon" slot="trigger">
              <Icon
                icon="material-symbols-light:ink-pen-outline"
                class="w-8 h-8 text-secondary-muted"
              />
            </Button>
          </NewBookDialog>
        {/if}
      </div>
    </div>
  </div>
{/if}
