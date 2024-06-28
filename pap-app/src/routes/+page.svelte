<script lang="ts">
  import { onMount } from "svelte";
  import { H2, P, H3 } from "$lib/components/ui/typography/index";
  import * as Card from "$lib/components/ui/card";
  import { invoke } from "@tauri-apps/api";
  import { DbStringGet } from "$lib/stores";
  import type { Livro } from "$lib/types";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import Icon from "@iconify/svelte";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

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
    } catch (error) {
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

<div class="w-full h-full flex flex-col justify-start">
  <H2 class="w-[15%]">Livros</H2>
  <div class="w-full h-full flex flex-col justify-between">
    {#if $isLoading}
      <div class="flex justify-center items-center w-full h-full">
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      </div>
    {:else}
      <div class="py-6 px-4 sm:px-6 lg:px-8">
        <div
          class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4"
        >
          {#each books as book, index}
            <Card.Root class="shadow-md">
              <Card.Content>
                <a href="" class="block">
                  <img
                    src={book.img_url}
                    alt={`Book ${index + 1}`}
                    width={150}
                    height={250}
                    class="w-full h-48 object-cover"
                  />
                </a>
                <div>
                  <a href="" class="block">
                    <H3 class="text-lg font-bold line-clamp-2">{book.nome}</H3>
                    <P class="text-muted-foreground">{book.autor}</P>
                    <P class="text-xs line-clamp-2">
                      {book.resumo}
                    </P>
                  </a>
                </div>
              </Card.Content>
            </Card.Root>
          {/each}
        </div>
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
</div>
