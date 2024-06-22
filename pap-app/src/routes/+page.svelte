<script lang="ts">
  import { onMount } from "svelte";
  import { H2, P } from "$lib/components/ui/typography/index";
  import { invoke } from "@tauri-apps/api";
  import { DbStringGet } from "$lib/stores";
  import type { Livro } from "$lib/types";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import Icon from "@iconify/svelte";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";
  import H3 from "$lib/components/ui/typography/H3.svelte";

  DbStringGet();

  let books: Livro[] = [];
  let booksPerPage = 12;
  let bookPagesCount = 0;
  let currentPage = 0;
  const isLoading = writable(false);

  onMount(async () => {
    try {
      isLoading.set(true);
      books = await invoke("get_books", { limit: booksPerPage, offset: 0 });
      bookPagesCount =
        ((await invoke("get_books_count")) as number) / booksPerPage;
      console.log(books);
    } catch (error) {
      toast.error("Falha ao carregar livros");
    } finally {
      console.log("done");
      isLoading.set(false);
    }
  });

  $: {
    if (currentPage < 0) {
      currentPage = 0;
    } else {
      (async () => {
        try {
          isLoading.set(true);
          books = await invoke("get_books", {
            limit: booksPerPage,
            offset: currentPage * 10,
          });
        } catch (error) {
          toast.error("Falha ao carregar livros");
        } finally {
          isLoading.set(false);
        }
      })();
    }
  }
</script>

<div class="flex justify-start items-start flex-col w-full h-full">
  <H2>Livros</H2>
  {#if $isLoading}
    <div class="flex justify-center items-center flex-col w-full h-full">
      <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
    </div>
  {:else}
    <div
      class="grid grid-cols-2 md:grid-cols-4 xl:grid-cols-6 gap-8 w-full h-full pt-4 pb-4 overflow-auto"
    >
      {#each books as book}
        <div
          class="flex justify-start items-start w-full h-full bg-primary-foreground rounded-lg relative"
        >
          <div
            class="flex flex-col justify-start items-start w-full h-full p-4"
          >
            <H3>{book.nome}</H3>
            <P>{book.autor}</P>
            <P>{book.ano_edicao}</P>
          </div>
        </div>
      {/each}
    </div>
  {/if}
  <div class="flex justify-evenly items-center w-full pt-2">
    <Button on:click={() => (currentPage -= 1)} variant="outline" size="icon">
      <ChevronLeft class="w-[1.2rem] h-[1.2rem]" />
    </Button>
    <div class="flex flex-row gap-12">
      <div>
        {#if currentPage + 1 > 1}
          {#each Array.from({ length: 3 })
            .map((_, i) => currentPage - i - 1)
            .reverse() as page}
            {#if page >= 0}
              <Button
                on:click={() => (currentPage = Number(page))}
                variant="outline"
                size="icon"
              >
                {Number(page + 1)}
              </Button>
            {/if}
          {/each}
        {/if}
      </div>
      <Button variant="outline" size="icon">{currentPage + 1}</Button>
      <div>
        {#if currentPage + 1 < bookPagesCount}
          {#each Array.from( { length: 3 } ).map((_, i) => currentPage + i + 1) as page}
            {#if page < bookPagesCount}
              <Button
                on:click={() => (currentPage = Number(page))}
                variant="outline"
                size="icon"
              >
                {Number(page + 1)}
              </Button>
            {/if}
          {/each}
        {/if}
      </div>
    </div>
    <Button on:click={() => (currentPage += 1)} variant="outline" size="icon">
      <ChevronRight class="w-[1.2rem] h-[1.2rem]" />
    </Button>
  </div>
</div>
