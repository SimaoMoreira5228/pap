<script lang="ts">
  import { call } from "$lib/call";
  import { H3 } from "$lib/components/ui/typography";
  import type { Leitor, Livro, Requisicao } from "$lib/types";
  import BookReturnDialog from "$lib/components/custom/BookReturnDialog.svelte";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  let request: Requisicao[] = [];

  async function load(thing: number = 0) {
    request = await call("get_requests");
  }

  onMount(async () => {
    await load();
  });

  async function getBook(id: number) {
    const book = await call<Livro>("get_book_by_id", { id });
    return book;
  }

  async function getLeitor(id: number) {
    const leitor = await call<Leitor>("get_reader_by_id", { id });
    return leitor;
  }

  function getDate(date: string) {
    const dateObj = new Date(date);
    return dateObj.toLocaleString("pt-PT");
  }
</script>

<div class="flex flex-col gap-4 overflow-auto w-full h-full">
  <H3>Requisicões</H3>
  <div class="flex flex-col gap-2 overflow-auto">
    {#if request.length === 0}
      <div class="flex justify-center items-center w-full h-full">
        <H3>Nenhuma requisição encontrada</H3>
      </div>
    {:else}
      {#each request as req}
        <div class="flex flex-col gap-2 border p-2 rounded-lg">
          {#await getBook(req.id_livro_requisitado)}
            <div class="flex justify-center items-center w-full h-full">
              <Icon
                icon="svg-spinners:270-ring-with-bg"
                class="w-8 h-8 text-primary"
              />
            </div>
          {:then book}
            <div class="flex flex-col gap-2">
              <div
                class="flex flex-row items-center justify-between gap-4 w-full"
              >
                <div class="flex flex-col gap-2">
                  <a href="/books/{book.id}">Livro: {book.nome}</a>
                  {#await getLeitor(req.id_leitor)}
                    <div class="flex justify-center items-center w-full h-full">
                      <Icon
                        icon="svg-spinners:270-ring-with-bg"
                        class="w-2 h-2 text-primary"
                      />
                    </div>
                  {:then leitor}
                    <a href="/books/{leitor.id}">Leitor: {leitor.nome}</a>
                  {/await}
                </div>
                <div class="flex flex-col items-center gap-2">
                  <p>{getDate(req.data_requisicao)}</p>
                  <p>
                    {#if req.data_entrega}
                      {getDate(req.data_entrega)}
                    {:else}
                      <BookReturnDialog bookId={book.id} updateBook={load} />
                    {/if}
                  </p>
                </div>
              </div>
            </div>
          {/await}
        </div>
      {/each}
    {/if}
  </div>
</div>
