<script lang="ts">
  import { call } from "$lib/call";
  import SearchBar from "$lib/components/custom/SearchBar.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { H2, H3, P } from "$lib/components/ui/typography";
  import { jwtStore } from "$lib/stores";
  import type { Editora } from "$lib/types";
  import Icon from "@iconify/svelte";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let publishers: Editora[] = [];
  let booksPerPage = 18;
  $: totalPublishers = 0;
  let currentPageStore = writable(0);
  let publishersSearch = writable<string | null>(null);
  const isLoading = writable(false);

  $: pages = getPages($currentPageStore, totalPublishers);

  function getPages(current: number, total: number) {
    let firstPage = 0;
    let lastPage = total;

    let backPages = Array.from({ length: 3 })
      .map((_, i) => current - i - 1)
      .reverse();
    let frontPages = Array.from({ length: 3 }).map((_, i) => current + i + 1);

    return { firstPage, lastPage, backPages, frontPages };
  }

  async function getPublishers(
    $currentPageStore: number,
    search: string | null
  ) {
    if (jwtStore.get() === "") return;

    if (search === "") {
      publishersSearch.set(null);
    } else {
      publishersSearch.set(search);
    }

    try {
      isLoading.set(true);

      publishers = await call("get_publishers", {
        limit: booksPerPage,
        offset: $currentPageStore * 10,
        search,
      });

      totalPublishers = Math.ceil(
        (await call<number>("get_publishers_count", { search })) / booksPerPage
      );
    } catch (error) {
      console.error(error);
      toast.error("Falha ao carregar livros");
    } finally {
      isLoading.set(false);
    }
  }

  onMount(async () => {
    getPublishers($currentPageStore, $publishersSearch);
  });

  currentPageStore.subscribe((value) => {
    if (value < 0) {
      currentPageStore.set(0);
    } else if (value >= totalPublishers) {
      currentPageStore.set(totalPublishers - 1);
    }

    getPublishers(value, $publishersSearch);
  });
</script>

<div class="w-full h-full flex flex-col justify-start">
  <H2 class="w-[15%]">Editoras</H2>
  <div class="w-full flex items-center justify-center py-4">
    <SearchBar
      searchFunction={(value) => getPublishers($currentPageStore, value)}
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
          {#each publishers as publisher}
            <Card.Root class="shadow-md">
              <Card.Content>
                <a href={`/publishers/${publisher.id}`} class="block py-6">
                  <H3>{publisher.nome}</H3>
                  <div class="flex flex-col gap-0">
                    {#if publisher.morada}
                      <P class="!mt-1">
                        Morada: {publisher.morada} - {publisher.codigo_postal}
                      </P>
                    {/if}
                    {#if publisher.email}
                      <P class="!mt-1 line-clamp-2">Email: {publisher.email}</P>
                    {/if}

                    {#if publisher.telefone}
                      <P class="!mt-1">Telefone: {publisher.telefone}</P>
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
