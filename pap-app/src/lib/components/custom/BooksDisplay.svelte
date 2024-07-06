<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { H3, P } from "$lib/components/ui/typography";
  import type { Livro } from "$lib/types";
  export let books: Livro[] = [];
</script>

<div class="py-6 px-4 sm:px-6 lg:px-8 overflow-auto">
  <div
    class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4"
  >
    {#each books as book, index}
      <Card.Root class="shadow-md">
        <Card.Content>
          <a href={`/books/${book.id}`} class="block">
            <img
              src={book.img_url}
              alt={`Book ${index + 1}`}
              width={150}
              height={250}
              class="w-full h-48 object-cover"
            />
            <div>
              <H3 class="text-lg font-bold line-clamp-2">{book.nome}</H3>
              <a href={`/authors/${book.autor_id}`}>
                <P class="text-muted-foreground">{book.autor}</P>
              </a>
              <!-- Todo: add book publisher -->
              <P class="text-xs line-clamp-2">
                {book.resumo}
              </P>
            </div>
          </a>
        </Card.Content>
      </Card.Root>
    {/each}
  </div>
</div>
