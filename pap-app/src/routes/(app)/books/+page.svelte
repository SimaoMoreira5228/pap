<script lang="ts">
  import { onMount } from "svelte";
  import { H2, H3, P } from "$lib/components/ui/typography/index";
  import type { Livro } from "$lib/types";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import Icon from "@iconify/svelte";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import { get, writable } from "svelte/store";
  import { call } from "$lib/call";
  import { jwtStore } from "$lib/stores";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";
  import SearchBar from "$lib/components/custom/SearchBar.svelte";
  import NewBookDialog from "$lib/components/custom/NewBookDialog.svelte";
  import { hasPermission } from "$lib/utils";

  let books: Livro[] = [];
  let booksPerPage = 12;
  $: totalBooks = 0;
  let currentPageStore = writable(0);
  let bookSearch = writable<string | null>(null);
  const isLoading = writable(false);

  $: pages = getPages($currentPageStore, totalBooks);

  function getPages(current: number, total: number) {
    let firstPage = 0;
    let lastPage = total;

    let backPages = Array.from({ length: 3 })
      .map((_, i) => current - i - 1)
      .reverse();

    let frontPages = Array.from({ length: 3 }).map((_, i) => current + i + 1);

    return { firstPage, lastPage, backPages, frontPages };
  }

  async function getBooks($currentPageStore: number, search: string | null) {
    if (jwtStore.get() === "") return;

    if (search === "") {
      bookSearch.set(null);
    } else {
      bookSearch.set(search);
    }

    try {
      isLoading.set(true);

      books = await call<Livro[]>("get_books", {
        limit: booksPerPage,
        offset: $currentPageStore * 10,
        search,
      });

      totalBooks = Math.ceil(
        (await call<number>("get_books_count", { search })) / booksPerPage
      );
    } catch (error) {
      console.error(error);
      toast.error("Falha ao carregar livros");
    } finally {
      isLoading.set(false);
    }
  }

  onMount(async () => {
    getBooks($currentPageStore, $bookSearch);
  });

  currentPageStore.subscribe((value) => {
    if (value < 0) {
      currentPageStore.set(0);
    } else if (value >= totalBooks) {
      currentPageStore.set(totalBooks - 1);
    }

    getBooks(value, $bookSearch);
  });

  const hasCreatePermission = hasPermission("criar_livro");
</script>

<div class="w-full h-full flex flex-col justify-start">
  <H2 class="w-[15%]">Livros</H2>
  <div class="w-full flex items-center justify-center py-4 gap-1">
    <SearchBar
      searchFunction={(value) => {
        currentPageStore.set(0);
        getBooks($currentPageStore, value);
      }}
      class="!w-[90%]"
    />
    {#await hasCreatePermission}
      <div class="flex justify-center items-center">
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      </div>
    {:then hasPermission}
      {#if hasPermission}
        <NewBookDialog
          updateBooks={async () => {
            currentPageStore.set(0);
            bookSearch.set(null);
            await getBooks($currentPageStore, $bookSearch);
          }}
          action="create"
        >
          <Button variant="outline" size="icon" slot="trigger">
            <Icon
              icon="material-symbols-light:add-box-outline-rounded"
              class="w-8 h-8 text-secondary-muted"
            />
          </Button>
        </NewBookDialog>
      {/if}
    {/await}
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
      <BooksDisplay {books} />
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
            {#if pages.backPages.find((page) => page === pages.firstPage) === undefined}
              <Button
                on:click={() => ($currentPageStore = pages.firstPage)}
                variant="outline"
                size="icon"
              >
                {pages.firstPage + 1}
              </Button>
            {/if}
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
