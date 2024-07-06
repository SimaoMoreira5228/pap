<script lang="ts">
  import { call } from "$lib/call";
  import SearchBar from "$lib/components/custom/SearchBar.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { H2, H3, P } from "$lib/components/ui/typography";
  import { jwtStore } from "$lib/stores";
  import type { Autor } from "$lib/types";
  import Icon from "@iconify/svelte";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let authors: Autor[] = [];
  let booksPerPage = 18;
  $: totalAuthors = 0;
  let currentPageStore = writable(0);
  let authorsSearch = writable<string | null>(null);
  const isLoading = writable(false);

  $: pages = getPages($currentPageStore, totalAuthors);

  function getPages(current: number, total: number) {
    let firstPage = 0;
    let lastPage = total;

    let backPages = Array.from({ length: 3 })
      .map((_, i) => current - i - 1)
      .reverse();
    let frontPages = Array.from({ length: 3 }).map((_, i) => current + i + 1);

    return { firstPage, lastPage, backPages, frontPages };
  }

  async function getAuthors($currentPageStore: number, search: string | null) {
    if (jwtStore.get() === "") return;

    if (search === "") {
      authorsSearch.set(null);
    } else {
      authorsSearch.set(search);
    }

    try {
      isLoading.set(true);

      authors = await call("get_authors", {
        limit: booksPerPage,
        offset: $currentPageStore * 10,
        search,
      });

      totalAuthors = Math.ceil(
        (await call<number>("get_authors_count", { search })) / booksPerPage
      );
    } catch (error) {
      console.error(error);
      toast.error("Falha ao carregar livros");
    } finally {
      isLoading.set(false);
    }
  }

  onMount(async () => {
    getAuthors($currentPageStore, $authorsSearch);
  });

  currentPageStore.subscribe((value) => {
    if (value < 0) {
      currentPageStore.set(0);
    } else if (value >= totalAuthors) {
      currentPageStore.set(totalAuthors - 1);
    }

    getAuthors(value, $authorsSearch);
  });
</script>

<div class="w-full h-full flex flex-col justify-start">
  <H2 class="w-[15%]">Autores</H2>
  <div class="w-full flex items-center justify-center py-4">
    <SearchBar
      searchFunction={(value) => getAuthors($currentPageStore, value)}
      class="!w-[90%]"
    />
  </div>
  <div class="w-full h-full flex flex-col justify-between overflow-auto">
    {#if $isLoading}
      <div class="flex justify-center items-center w-full h-full">
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      </div>
    {:else}
      <div class="py-6 px-4 sm:px-6 lg:px-8 overflow-auto w-full h-full">
        <div
          class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4 w-full h-full"
        >
          {#each authors as author}
            <Card.Root class="shadow-md">
              <Card.Content>
                <a href={`/authors/${author.id}`} class="block py-6">
                  <H3>{author.nome}</H3>
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
                </a>
              </Card.Content>
            </Card.Root>
          {/each}
        </div>
      </div>
    {/if}
    <div class="flex justify-evenly items-center w-full pt-2">
      <Button
        on:click={() => ($currentPageStore -= 1)}
        size="icon"
        variant="outline"
      >
        <ChevronLeft class="w-[1.2rem] h-[1.2rem]" />
      </Button>
      <div class="flex flex-row gap-12">
        <div>
          {#if $currentPageStore - 1 >= 0}
            <Button
              on:click={() => ($currentPageStore = pages.firstPage)}
              variant="outline"
              size="icon"
            >
              {pages.firstPage}
            </Button>
            {#each pages.backPages as page}
              {#if page >= 0}
                <Button
                  on:click={() => ($currentPageStore = Number(page))}
                  variant="outline"
                  size="icon"
                >
                  {Number(page + 1)}
                </Button>
              {/if}
            {/each}
          {/if}
        </div>
        <Button size="icon" variant="outline">{$currentPageStore + 1}</Button>
        <div>
          {#if $currentPageStore + 1 < pages.lastPage}
            {#each pages.frontPages as page}
              {#if page < pages.lastPage}
                <Button
                  on:click={() => ($currentPageStore = Number(page))}
                  variant="outline"
                  size="icon"
                >
                  {Number(page + 1)}
                </Button>
              {/if}
            {/each}
            <Button
              on:click={() => ($currentPageStore = pages.lastPage)}
              variant="outline"
              size="icon"
            >
              {pages.lastPage}
            </Button>
          {/if}
        </div>
      </div>
      <Button
        on:click={() => ($currentPageStore += 1)}
        size="icon"
        variant="outline"
      >
        <ChevronRight class="w-[1.2rem] h-[1.2rem]" />
      </Button>
    </div>
  </div>
</div>
