<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";
  import { call } from "$lib/call";
  import type { Autor, Editora, Livro, SubCategoria } from "$lib/types";
  import { writable } from "svelte/store";
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";

  export let action: "create" | "update";
  export let bookId: number = 0;

  let isLoading = writable(false);

  let name = "";
  let resume = "";
  let nPages = 0;
  let language = "";
  let imgUrl = "";
  let anoEdicao = "";
  let authorId: any;
  let publisherId: any;
  let subCategoryId: any;

  let author = "";
  let publisher = "";
  let subCategory = "";

  async function getInCaseOfUpdate() {
    if (action === "update") {
      try {
        isLoading.set(true);

        const book = await call<Livro>("get_book_by_id", {
          id: bookId,
        });

        name = book.nome;
        resume = book.resumo ? book.resumo : "";
        nPages = book.n_paginas;
        language = book.idioma;
        imgUrl = book.img_url ? book.img_url : "";
        anoEdicao = book.ano_edicao ? book.ano_edicao.toString() : "";
        author = book.autor_id ? book.autor_id.toString() : "";
        findAuthor({ target: { value: author } });
        publisher = book.editora;
        findPublisher({ target: { value: publisher } });
        subCategory = book.sub_categoria ? book.sub_categoria : "";
        findSubCategory({ target: { value: subCategory } });
      } catch (error) {
        console.error(error);
        toast.error(error as string);
      } finally {
        isLoading.set(false);
      }
    }
  }

  export let updateBooks: () => Promise<void> = async () => {};

  async function handleSubmit() {
    try {
      if (action === "create") {
        if (!name) {
          return toast.error("O nome é obrigatório");
        }

        await call("create_book", {
          name,
          resume,
          nPages: nPages.toString(),
          language,
          imgUrl,
          anoEdicao,
          authorId: authorId.toString(),
          publisherId: publisherId.toString(),
          subCategoryId: subCategoryId.toString(),
        });

        toast.success("Livro criado com sucesso");
      } else if (action === "update") {
        if (!name) {
          return toast.error("O nome é obrigatório");
        }

        if (!authorId) {
          return toast.error("O autor é obrigatório");
        }

        if (!publisherId) {
          return toast.error("O editora é obrigatório");
        }

        if (!subCategoryId) {
          return toast.error("A subcategoria é obrigatória");
        }

        await call("update_book", {
          id: bookId,
          name,
          resume,
          nPages: nPages.toString(),
          language,
          imgUrl,
          anoEdicao,
          authorId: authorId.toString(),
          publisherId: publisherId.toString(),
          subCategoryId: subCategoryId.toString(),
        });

        toast.success("Livro atualizado com sucesso");
      }

      updateBooks();
    } catch (error) {
      toast.error(error as string);
    } finally {
      name = "";
      resume = "";
      nPages = 0;
      language = "";
      imgUrl = "";
      anoEdicao = "";
      authorId = 0;
      publisherId = 0;
      subCategoryId = 0;
    }
  }

  let authors = writable<Autor[]>([]);

  async function findAuthor(event: any) {
    try {
      const value = event.target.value;

      if (parseInt(value)) {
        async function get_leitor() {
          const author = await call<Autor | undefined>("get_author_by_id", {
            id: parseInt(value),
          });

          return author ? [author] : [];
        }

        authors.set(await get_leitor());
      } else {
        authors.set(
          (await call<Autor[] | undefined>("get_authors_by_name", {
            name: value,
          })) ?? []
        );
      }
    } catch (error) {
      toast.error(error as string);
    }
  }

  let publishers = writable<Editora[]>([]);

  async function findPublisher(event: any) {
    try {
      const value = event.target.value;

      if (parseInt(value)) {
        async function get_leitor() {
          const publisher = await call<Editora | undefined>(
            "get_publisher_by_id",
            {
              id: parseInt(value),
            }
          );

          return publisher ? [publisher] : [];
        }

        publishers.set(await get_leitor());
      } else {
        publishers.set(
          (await call<Editora[] | undefined>("get_publishers_by_name", {
            name: value,
          })) ?? []
        );
      }
    } catch (error) {
      toast.error(error as string);
    }
  }

  let subCategories = writable<SubCategoria[]>([]);

  async function findSubCategory(event: any) {
    try {
      const value = event.target.value;

      subCategories.set(
        (await call<SubCategoria[] | undefined>("get_sub_categories_by_name", {
          name: value,
        })) ?? []
      );
    } catch (error) {
      toast.error(error as string);
    }
  }

  onMount(async () => {
    if (action === "update") {
      await getInCaseOfUpdate();
    }
  });
</script>

<Dialog.Root>
  <Dialog.Trigger class="cursor-pointer">
    <button on:click={async () => await getInCaseOfUpdate()}>
      <slot name="trigger" />
    </button>
  </Dialog.Trigger>

  <Dialog.Content class="max-w-[50%] overflow-auto">
    <Dialog.Header>
      <Dialog.Title>
        {action === "create" ? "Criar Livro" : "Atualizar Livro"}
      </Dialog.Title>
    </Dialog.Header>
    {#if $isLoading}
      <div class="flex justify-center items-center w-full h-full">
        <Icon
          icon="svg-spinners:270-ring-with-bg"
          class="w-8 h-8 text-primary"
        />
      </div>
    {:else}
      <div
        class="grid grid-cols-1 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-y-4 gap-x-36 w-full h-full py-4 overflow-auto items-center"
      >
        <div class="flex flex-col gap-2">
          <Label for="nome">Nome</Label>
          <Input
            type="text"
            id="nome"
            name="nome"
            class="border border-muted rounded-lg"
            bind:value={name}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="resume">Resumo</Label>
          <Input
            type="text"
            id="resume"
            name="resume"
            class="border border-muted rounded-lg"
            bind:value={resume}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="nPages">Número de páginas</Label>
          <Input
            type="number"
            id="nPages"
            name="nPages"
            class="border border-muted rounded-lg"
            bind:value={nPages}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="language">Idioma</Label>
          <Input
            type="text"
            id="language"
            name="language"
            class="border border-muted rounded-lg"
            bind:value={language}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="imgUrl">Imagem</Label>
          <Input
            type="text"
            id="imgUrl"
            name="imgUrl"
            class="border border-muted rounded-lg"
            bind:value={imgUrl}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="anoEdicao">Ano de edição</Label>
          <Input
            type="text"
            id="anoEdicao"
            name="anoEdicao"
            class="border border-muted rounded-lg"
            bind:value={anoEdicao}
          />
        </div>
        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <Label for="nautor-nome">Número de author / Nome</Label>
            <Input
              type="text"
              id="nautor-nome"
              name="nautor-nome"
              class="border border-muted rounded-lg"
              on:change={(event) => findAuthor(event)}
              bind:value={author}
            />
          </div>

          <Select.Root
            onSelectedChange={(value) => {
              authorId = value?.value;
            }}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="Autor" />
            </Select.Trigger>
            <Select.Content>
              {#each $authors as author}
                <Select.Item value={author.id}>
                  {author.id} - {author.nome}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>

        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <Label for="npublisher-nome">Número de publisher / Nome</Label>
            <Input
              type="text"
              id="npublisher-nome"
              name="npublisher-nome"
              class="border border-muted rounded-lg"
              bind:value={publisher}
              on:change={(event) => findPublisher(event)}
            />
          </div>
          <Select.Root
            onSelectedChange={(value) => {
              publisherId = value?.value;
            }}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="Editora" />
            </Select.Trigger>
            <Select.Content>
              {#each $publishers as publisher}
                <Select.Item value={publisher.id}>
                  {publisher.id} - {publisher.nome}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>

        <div class="flex flex-col gap-4">
          <div class="flex flex-col gap-2">
            <Label for="nsubcategory-nome">Nome da subcategoria</Label>
            <Input
              type="text"
              id="nsubcategory-nome"
              name="nsubcategory-nome"
              class="border border-muted rounded-lg"
              bind:value={subCategory}
              on:change={(event) => findSubCategory(event)}
            />
          </div>
          <Select.Root
            onSelectedChange={(value) => {
              subCategoryId = value?.value;
            }}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="Sub Categoria" />
            </Select.Trigger>
            <Select.Content>
              {#each $subCategories as subCategory}
                <Select.Item value={subCategory.id}>
                  {subCategory.id} - {subCategory.nome}
                </Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>

        <Dialog.Footer>
          <Dialog.Close>
            <Button type="submit" on:click={handleSubmit}
              >{action === "create" ? "Criar Livro" : "Atualizar Livro"}</Button
            >
          </Dialog.Close>
        </Dialog.Footer>
      </div>
    {/if}
  </Dialog.Content>
</Dialog.Root>
