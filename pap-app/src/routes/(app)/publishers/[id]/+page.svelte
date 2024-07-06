<script lang="ts">
  import { page } from "$app/stores";
  import { call } from "$lib/call";
  import BooksDisplay from "$lib/components/custom/BooksDisplay.svelte";
  import { H3, H2, P } from "$lib/components/ui/typography";
  import type { Editora, Livro } from "$lib/types";
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { writable } from "svelte/store";

  let params = $page.params;
  let publisher: Editora;
  let books: Livro[];
  let isLoading = writable(true);

  onMount(async () => {
    try {
      isLoading.set(true);

      publisher = await call("get_publisher_by_id", {
        id: parseInt(params.id),
      });

      books = await call("get_books_by_publisher_id", {
        id: parseInt(params.id),
      });

      isLoading.set(false);
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    }
  });
</script>

<div class="flex w-full h-full overflow-auto">
  {#if $isLoading}
    <div class="flex justify-center items-center w-full h-full">
      <Icon icon="svg-spinners:270-ring-with-bg" class="w-8 h-8 text-primary" />
    </div>
  {:else}
    <div class="flex flex-col w-full h-full">
      <H2>{publisher.nome}</H2>
      <div class="flex flex-col gap-0">
        {#if publisher.morada}
          <P class="!mt-1">Morada: {publisher.morada} - {publisher.codigo_postal}</P>
        {/if}
        {#if publisher.email}
          <P class="!mt-1">Email: {publisher.email}</P>
        {/if}

        {#if publisher.telefone}
          <P class="!mt-1">Telefone: {publisher.telefone}</P>
        {/if}
      </div>
      <div class="flex flex-col w-full h-full">
        <H3 class="!mt-2">Livros do autor</H3>
        <BooksDisplay {books} />
      </div>
    </div>
  {/if}
</div>
