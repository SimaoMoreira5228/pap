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

  let params = $page.params;
  let isLoading = writable(true);
  let book: Livro;

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
  });
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
        <P>{book.resumo}</P>
      </div>
      <div class="mt-8">
        {#if book.requisitado}
          <BookReturnDialog bookId={book.id} updateBook={getBook} />
        {:else}
          <RequestBookDialog {book} updateBook={getBook}>
            <NewReaderDialog />
          </RequestBookDialog>
        {/if}
      </div>
    </div>
  </div>
{/if}
