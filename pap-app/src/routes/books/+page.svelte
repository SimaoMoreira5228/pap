<script lang="ts">
  import { onMount } from "svelte";
  import { H2, H3, P } from "$lib/components/ui/typography/index";
  import type { Livro } from "$lib/types";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import Icon from "@iconify/svelte";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";
  import { call } from "$lib/call";
  import { jwtStore } from "$lib/stores";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";

  let books: Livro[] = [];
  let booksPerPage = 12;
  let bookPagesCount = 0;
  let currentPage = 0;
  const isLoading = writable(false);

  onMount(async () => {
    if (jwtStore.get() === "") return;

    try {
      isLoading.set(true);
      books = await call<Livro[]>("get_books", {
        limit: booksPerPage,
        offset: 0,
      });
      bookPagesCount = (await call<number>("get_books_count")) / booksPerPage;
    } catch (error) {
      console.error(error);
      toast.error("Falha ao carregar livros");
    } finally {
      isLoading.set(false);
    }
  });

  $: {
    if (currentPage < 0) {
      currentPage = 0;
    } else {
      (async () => {
        if (jwtStore.get() === "") return;

        try {
          isLoading.set(true);
          books = await call<Livro[]>("get_books", {
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

<div class="w-full h-full flex flex-col justify-start">
  <H2 class="w-[15%]">Livros</H2>
  <div class="w-full h-full flex flex-col justify-between overflow-auto">
    {#if $isLoading}
      <div class="flex justify-center items-center w-full h-full">
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      </div>
    {:else}
      <BooksDisplay {books} />
    {/if}
    <div class="flex justify-evenly items-center w-full pt-2">
      <Button on:click={() => (currentPage -= 1)} size="icon" variant="outline">
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
        <Button size="icon" variant="outline">{currentPage + 1}</Button>
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
      <Button on:click={() => (currentPage += 1)} size="icon" variant="outline">
        <ChevronRight class="w-[1.2rem] h-[1.2rem]" />
      </Button>
    </div>
  </div>
</div>
